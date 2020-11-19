use alloc::boxed::Box;
use core::convert::{TryFrom, TryInto};
use core::marker;
use core::mem;
use core::ptr;

use crate::bindings;
use crate::c_types;
use crate::error::{Error, KernelResult};
use crate::file_operations::{File, SeekFrom};
use crate::ioctl::Ioctl;
use crate::user_ptr::{UserSlicePtr, UserSlicePtrReader, UserSlicePtrWriter};

unsafe extern "C" fn open_callback<T: ProcOperations>(
    _inode: *mut bindings::inode,
    file: *mut bindings::file,
) -> c_types::c_int {
    let f = match T::open() {
        Ok(f) => Box::new(f),
        Err(e) => return e.to_kernel_errno(),
    };

    (*file).private_data = Box::into_raw(f) as *mut c_types::c_void;

    0
}

unsafe extern "C" fn read_callback<T: ProcOperations>(
    file: *mut bindings::file,
    buf: *mut c_types::c_char,
    len: c_types::c_size_t,
    offset: *mut bindings::loff_t,
) -> c_types::c_ssize_t {
    let mut data = match UserSlicePtr::new(buf as *mut c_types::c_void, len) {
        Ok(ptr) => ptr.writer(),
        Err(e) => return e.to_kernel_errno().try_into().unwrap(),
    };

    let f = &*((*file).private_data as *const T);

    // No FMODE_UNSIGNED_OFFSET support, so offset must be in [0, 2^63).
    // See discussion in #113
    let positive_offset = match (*offset).try_into() {
        Ok(v) => v,
        Err(_) => return Error::EINVAL.to_kernel_errno().try_into().unwrap(),
    };

    match f.read(&File::from_ptr(file), &mut data, positive_offset) {
        Ok(()) => {
            let written = len - data.len();
            (*offset) += bindings::loff_t::try_from(written).unwrap();
            written.try_into().unwrap()
        }
        Err(e) => e.to_kernel_errno().try_into().unwrap(),
    }
}

unsafe extern "C" fn write_callback<T: ProcOperations>(
    file: *mut bindings::file,
    buf: *const c_types::c_char,
    len: c_types::c_size_t,
    offset: *mut bindings::loff_t,
) -> c_types::c_ssize_t {
    let mut data = match UserSlicePtr::new(buf as *mut c_types::c_void, len) {
        Ok(ptr) => ptr.reader(),
        Err(e) => return e.to_kernel_errno().try_into().unwrap(),
    };

    let f = &*((*file).private_data as *const T);

    // No FMODE_UNSIGNED_OFFSET support, so offset must be in [0, 2^63).
    // See discussion in #113
    let positive_offset = match (*offset).try_into() {
        Ok(v) => v,
        Err(_) => return Error::EINVAL.to_kernel_errno().try_into().unwrap(),
    };

    match f.write(&File::from_ptr(file), &mut data, positive_offset) {
        Ok(()) => {
            let read = len - data.len();
            (*offset) += bindings::loff_t::try_from(read).unwrap();
            read.try_into().unwrap()
        }
        Err(e) => e.to_kernel_errno().try_into().unwrap(),
    }
}

unsafe extern "C" fn release_callback<T: ProcOperations>(
    _inode: *mut bindings::inode,
    file: *mut bindings::file,
) -> c_types::c_int {
    let ptr = mem::replace(&mut (*file).private_data, ptr::null_mut());

    drop(Box::from_raw(ptr as *mut T));

    0
}

unsafe extern "C" fn lseek_callback<T: ProcOperations>(
    file: *mut bindings::file,
    offset: bindings::loff_t,
    whence: c_types::c_int,
) -> bindings::loff_t {
    let off = match whence as u32 {
        bindings::SEEK_SET => match offset.try_into() {
            Ok(v) => SeekFrom::Start(v),
            Err(_) => return Error::EINVAL.to_kernel_errno().into(),
        },
        bindings::SEEK_CUR => SeekFrom::Current(offset),
        bindings::SEEK_END => SeekFrom::End(offset),
        _ => return Error::EINVAL.to_kernel_errno().into(),
    };

    let f = &*((*file).private_data as *const T);

    match f.lseek(&File::from_ptr(file), off) {
        Ok(off) => off as bindings::loff_t,
        Err(e) => e.to_kernel_errno().into(),
    }
}

unsafe extern "C" fn ioctl_callback<T: ProcOperations>(
    file: *mut bindings::file,
    num: c_types::c_uint,
    param: c_types::c_ulong,
) -> c_types::c_long {
    let num = Ioctl::from(num);

    let f = &*((*file).private_data as *const T);

    match f.ioctl(&File::from_ptr(file), num, param) {
        Ok(ret) => ret as c_types::c_long,
        Err(e) => e.to_kernel_errno().into(),
    }
}

pub(crate) struct ProcOperationsVtable<T>(marker::PhantomData<T>);

#[cfg(kernel_5_6_0_or_greater)]
impl<T: ProcOperations> ProcOperationsVtable<T> {
    pub(crate) const VTABLE: bindings::proc_ops = bindings::proc_ops {
        proc_flags: 0,

        proc_open: Some(open_callback::<T>),
        proc_release: Some(release_callback::<T>),
        proc_read: Some(read_callback::<T>),
        proc_write: Some(write_callback::<T>),
        proc_lseek: Some(lseek_callback::<T>),
        proc_ioctl: Some(ioctl_callback::<T>),

        proc_poll: None,
        proc_compat_ioctl: None,
        proc_mmap: None,
        proc_get_unmapped_area: None,
    };
}

#[cfg(not(kernel_5_6_0_or_greater))]
impl<T: ProcOperations> ProcOperationsVtable<T> {
    pub(crate) const VTABLE: bindings::file_operations = bindings::file_operations {
        open: Some(open_callback::<T>),
        release: Some(release_callback::<T>),
        read: Some(read_callback::<T>),
        write: Some(write_callback::<T>),
        llseek: Some(lseek_callback::<T>),
        unlocked_ioctl: Some(ioctl_callback::<T>),

        #[cfg(not(kernel_4_9_0_or_greater))]
        aio_fsync: None,
        check_flags: None,
        #[cfg(all(kernel_4_5_0_or_greater, not(kernel_4_20_0_or_greater)))]
        clone_file_range: None,
        compat_ioctl: None,
        #[cfg(kernel_4_5_0_or_greater)]
        copy_file_range: None,
        #[cfg(all(kernel_4_5_0_or_greater, not(kernel_4_20_0_or_greater)))]
        dedupe_file_range: None,
        fallocate: None,
        #[cfg(kernel_4_19_0_or_greater)]
        fadvise: None,
        fasync: None,
        flock: None,
        flush: None,
        fsync: None,
        get_unmapped_area: None,
        iterate: None,
        #[cfg(kernel_4_7_0_or_greater)]
        iterate_shared: None,
        #[cfg(kernel_5_1_0_or_greater)]
        iopoll: None,
        lock: None,
        mmap: None,
        #[cfg(kernel_4_15_0_or_greater)]
        mmap_supported_flags: 0,
        owner: ptr::null_mut(),
        poll: None,
        read_iter: None,
        #[cfg(kernel_4_20_0_or_greater)]
        remap_file_range: None,
        sendpage: None,
        #[cfg(kernel_aufs_setfl)]
        setfl: None,
        setlease: None,
        show_fdinfo: None,
        splice_read: None,
        splice_write: None,
        write_iter: None,
    };
}

/// `ProcOperations` correspondgs to the kernel's `struct proc_operations`. You
/// implement this treait whenever you would create a `struct proc_operations`.
/// File descriptors may be used from multiple threads (or processes)
/// concurrently, so your type must be `Sync`.
pub trait ProcOperations: Sync + Sized {
    /// Creates a new instance of this file. Corresponds to the `proc_open`
    /// function pointer in `struct proc_operations`.
    fn open() -> KernelResult<Self>;

    /// Reads data from this file to userspace. Corresponds to the `proc_read`
    /// function pointer in `struct proc_operations`.
    fn read(
        &self,
        _file: &File,
        _buf: &mut UserSlicePtrWriter,
        _offset: u64,
    ) -> KernelResult<()> {
        Err(Error::EINVAL)
    }

    /// Writes data from userspace to this file. Corresponds to the
    /// `proc_write` function pointer in `struct proc_operations`.
    fn write(
        &self,
        _file: &File,
        _buf: &mut UserSlicePtrReader,
        _offset: u64,
    ) -> KernelResult<()> {
        Err(Error::EINVAL)
    }

    /// Changes the position of the file. Corresponds to the `proc_lseek`
    /// function pointer in `struct proc_operations`.
    fn lseek(
        &self,
        _file: &File,
        _from: SeekFrom,
    ) -> KernelResult<u64> {
        Err(Error::EINVAL)
    }

    /// Corresponds to the `proc_ioctl` function pointer in
    /// `struct proc_operations`.
    fn ioctl(
        &self,
        _file: &File,
        _num: Ioctl,
        _param: u64,
    ) -> KernelResult<u64> {
        Err(Error::EINVAL)
    }
}
