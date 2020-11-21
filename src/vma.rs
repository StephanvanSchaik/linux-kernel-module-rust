use bitflags::bitflags;
use crate::bindings;
use crate::types::FromRaw;

bitflags! {
    pub struct VMFlags: u64 {
        const READ = (1 << 0);
        const WRITE = (1 << 1);
        const EXEC = (1 << 2);
        const SHARED = (1 << 3);

        const MAY_READ = (1 << 4);
        const MAY_WRITE = (1 << 5);
        const MAY_EXEC = (1 << 6);
        const MAY_SHARE = (1 << 7);

        const GROWS_DOWN = (1 << 8);
        const UFFD_MISSING = (1 << 9);
        const PFN_MAP = (1 << 10);
        const DENY_WRITE = (1 << 11);
        const UFFD_WP = (1 << 12);

        const LOCKED = (1 << 13);
        const IO = (1 << 14);

        const SEQ_READ = (1 << 15);
        const RAND_READ = (1 << 16);

        const DONT_COPY = (1 << 17);
        const DONT_EXPAND = (1 << 18);
        const LOCK_ON_FAULT = (1 << 19);
        const ACCOUNT = (1 << 20);
        const NO_RESERVE = (1 << 21);
        const HUGE_TLB = (1 << 22);
        const SYNC = (1 << 23);
        const ARCH_1 = (1 << 24);
        const WIPE_ON_FORK = (1 << 25);
        const DONT_DUMP = (1 << 26);

        const SOFT_DIRTY = (1 << 27);

        const MIXED_MAP = (1 << 28);
        const HUGE_PAGE = (1 << 29);
        const NO_HUGE_PAGE = (1 << 30);
        const MERGEABLE = (1 << 31);
    }
}

pub struct VMA {
    raw: *mut bindings::vm_area_struct,
}

impl VMA {
    pub fn raw(
        &self,
    ) -> *mut bindings::vm_area_struct {
        self.raw
    }

    pub fn start(&self) -> u64 {
        unsafe { (*self.raw).vm_start }
    }

    pub fn end(&self) -> u64 {
        unsafe { (*self.raw).vm_end }
    }

    pub fn flags(&self) -> VMFlags {
        VMFlags::from_bits_truncate(unsafe { (*self.raw).vm_flags })
    }

    pub fn set_flags(&mut self, flags: VMFlags) {
        unsafe {
            (*self.raw).vm_flags = flags.bits();
        }
    }

    pub fn offset(&self) -> u64 {
        unsafe { (*self.raw).vm_pgoff }
    }
}

impl FromRaw<bindings::vm_area_struct> for VMA {
    unsafe fn from_raw(raw: *mut bindings::vm_area_struct) -> Self {
        Self {
            raw,
        }
    }
}
