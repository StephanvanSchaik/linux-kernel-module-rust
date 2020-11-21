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
}

impl FromRaw<bindings::vm_area_struct> for VMA {
    unsafe fn from_raw(raw: *mut bindings::vm_area_struct) -> Self {
        Self {
            raw,
        }
    }
}
