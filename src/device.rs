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


