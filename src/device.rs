use alloc::sync::Arc;
use crate::bindings;
use crate::c_types;
use crate::types::CStr;

pub struct Class {
    class: *mut bindings::class,
}

unsafe impl Send for Class {}
unsafe impl Sync for Class {}

impl Class {
    pub fn new(name: CStr<'static>, key: &mut bindings::lock_class_key) -> Self {
        let class = unsafe {
            bindings::__class_create(
                &mut bindings::__this_module,
                name.as_ptr() as *const c_types::c_char,
                key as *mut bindings::lock_class_key,
            )
        };

        Self {
            class,
        }
    }
}

impl Drop for Class {
    fn drop(&mut self) {
        unsafe {
            bindings::class_destroy(self.class);
        }
    }
}

pub struct Device {
    class: Arc<Class>,
    _device: *mut bindings::device,
    number: bindings::dev_t,
}

unsafe impl Sync for Device {}

impl Device {
    pub fn new(
        class: Arc<Class>,
        name: CStr<'static>,
        dev: (bindings::dev_t, bindings::dev_t),
    ) -> Self {
        let number = ((dev.0 as bindings::dev_t) << 8) | dev.1 as bindings::dev_t;

        let device = unsafe {
            bindings::device_create(
                class.class,
                core::ptr::null_mut(),
                number,
                core::ptr::null_mut(),
                name.as_ptr() as *const c_types::c_char,
            )
        };

        Self {
            class: class,
            _device: device,
            number,
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            bindings::device_destroy(self.class.class, self.number);
        }
    }
}

