use crate::bindings;

pub struct Interface {
    raw: *mut bindings::usb_interface,
}

impl Interface {
    pub unsafe fn from_raw(
        raw: *mut bindings::usb_interface,
    ) -> Self {
        Self {
            raw,
        }
    }

    pub fn raw(&self) -> *const bindings::usb_interface {
        self.raw as *const bindings::usb_interface
    }

    pub fn raw_mut(&self) -> *mut bindings::usb_interface {
        self.raw
    }
}
