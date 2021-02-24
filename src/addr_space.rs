use crate::bindings;
use crate::{Error, KernelResult};
use crate::types::FromRaw;
use crate::vma::VMA;

extern "C" {
    fn spin_lock_helper(lock: *const bindings::spinlock_t);
    fn spin_unlock_helper(lock: *const bindings::spinlock_t);
}

pub struct Spinlock {
    raw: *mut bindings::spinlock,
}

impl Drop for Spinlock {
    fn drop(&mut self) {
        unsafe {
            spin_unlock_helper(self.raw);
        }
    }
}

pub struct ReadLock {
    raw: *mut bindings::rw_semaphore,
}

impl Drop for ReadLock {
    fn drop(&mut self) {
        unsafe {
            bindings::up_read(self.raw);
        }
    }
}

pub struct WriteLock {
    raw: *mut bindings::rw_semaphore,
}

impl Drop for WriteLock {
    fn drop(&mut self) {
        unsafe {
            bindings::up_write(self.raw);
        }
    }
}

pub struct AddressSpace {
    raw: *mut bindings::mm_struct,
}

impl AddressSpace {
    pub fn raw(&self) -> *const bindings::mm_struct {
        self.raw
    }

    pub fn raw_mut(&self) -> *mut bindings::mm_struct {
        self.raw
    }

    pub fn vmas(&self) -> VMAIterator {
        VMAIterator {
            raw: unsafe { (*self.raw).__bindgen_anon_1.mmap },
        }
    }

    pub fn find_vma(&self, addr: u64) -> KernelResult<VMA> {
        let raw = unsafe {
            bindings::find_vma(self.raw, addr)
        };

        if raw.is_null() {
            return Err(Error::ENOENT);
        }

        let vma = unsafe {
            VMA::from_raw(raw)
        };

        Ok(vma)
    }

    #[cfg(kernel_5_8_0_or_greater)]
    pub fn lock_read(&self) -> ReadLock {
        let lock = unsafe { &mut (*self.raw).__bindgen_anon_1.mmap_lock };

        unsafe {
            bindings::down_read(lock);
        }

        ReadLock {
            raw: lock,
        }
    }

    #[cfg(not(kernel_5_8_0_or_greater))]
    pub fn lock_read(&self) -> ReadLock {
        let lock = unsafe { &mut (*self.raw).__bindgen_anon_1.mmap_sem };

        unsafe {
            bindings::down_read(lock);
        }

        ReadLock {
            raw: lock,
        }
    }

    #[cfg(kernel_5_8_0_or_greater)]
    pub fn lock_write(&self) -> WriteLock {
        let lock = unsafe { &mut (*self.raw).__bindgen_anon_1.mmap_lock };

        unsafe {
            bindings::down_write(lock);
        }

        WriteLock {
            raw: lock,
        }
    }

    #[cfg(not(kernel_5_8_0_or_greater))]
    pub fn lock_write(&self) -> WriteLock {
        let lock = unsafe { &mut (*self.raw).__bindgen_anon_1.mmap_sem };

        unsafe {
            bindings::down_write(lock);
        }

        WriteLock {
            raw: lock,
        }
    }
}

impl FromRaw<bindings::mm_struct> for AddressSpace {
    unsafe fn from_raw(raw: *mut bindings::mm_struct) -> Self {
        Self {
            raw,
        }
    }
}

pub struct VMAIterator {
    raw: *mut bindings::vm_area_struct,
}

impl Iterator for VMAIterator {
    type Item = VMA;

    fn next(&mut self) -> Option<VMA> {
        if self.raw.is_null() {
            return None;
        }

        let raw = self.raw;
        self.raw = unsafe { (*raw).vm_next };

        Some(unsafe {
            VMA::from_raw(raw)
        })
    }
}
