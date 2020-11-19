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

#[derive(Clone, Copy)]
pub struct DeviceNumber(pub u32, pub u32);

impl DeviceNumber {
    pub fn major(&self) -> u32 {
        self.0
    }

    pub fn minor(&self) -> u32 {
        self.1
    }
}

impl From<bindings::dev_t> for DeviceNumber {
    fn from(value: u32) -> Self {
        Self(value >> 20, value & ((1 << 20) - 1))
    }
}

impl Into<u32> for DeviceNumber {
    fn into(self) -> bindings::dev_t {
        self.0 << 20 | self.1
    }
}

pub struct Device {
    class: Arc<Class>,
    _device: *mut bindings::device,
    dev: DeviceNumber,
}

unsafe impl Sync for Device {}

impl Device {
    pub fn new(
        class: Arc<Class>,
        name: CStr<'static>,
        dev: DeviceNumber,
    ) -> Self {
        let device = unsafe {
            bindings::device_create(
                class.class,
                core::ptr::null_mut(),
                dev.into(),
                core::ptr::null_mut(),
                name.as_ptr() as *const c_types::c_char,
            )
        };

        Self {
            class: class,
            _device: device,
            dev,
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            bindings::device_destroy(self.class.class, self.dev.into());
        }
    }
}

