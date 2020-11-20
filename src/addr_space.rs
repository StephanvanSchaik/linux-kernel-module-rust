use crate::bindings;
use crate::{Error, KernelResult};
use crate::types::FromRaw;
use crate::vma::VMA;

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
}

impl FromRaw<bindings::mm_struct> for AddressSpace {
    unsafe fn from_raw(raw: *mut bindings::mm_struct) -> Self {
        Self {
            raw,
        }
    }
}
