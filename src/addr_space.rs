use crate::bindings;
use crate::types::FromRaw;

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
}

impl FromRaw<bindings::mm_struct> for AddressSpace {
    unsafe fn from_raw(raw: *mut bindings::mm_struct) -> Self {
        Self {
            raw,
        }
    }
}


