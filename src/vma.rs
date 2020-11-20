use crate::bindings;
use crate::types::FromRaw;

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
