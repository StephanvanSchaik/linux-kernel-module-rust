use crate::bindings;
use crate::c_types;
use crate::proc_operations::{ProcOperations, ProcOperationsVtable};
use crate::types::CStr;

pub struct ProcDirEntry {
    raw: *mut bindings::proc_dir_entry,
}

impl ProcDirEntry {
    pub fn mkdir(
        name: CStr<'static>,
        mode: bindings::umode_t,
        parent: Option<&ProcDirEntry>,
    ) -> Self {
        let parent = match parent {
            Some(parent) => parent.raw,
            _ => core::ptr::null_mut(),
        };

        let raw = unsafe {
            bindings::proc_mkdir_mode(
                name.as_ptr() as *const c_types::c_char,
                mode,
                parent,
            )
        };

        Self {
            raw,
        }
    }

    pub fn create<T: ProcOperations>(
        name: CStr<'static>,
        mode: bindings::umode_t,
        parent: Option<&ProcDirEntry>,
    ) -> Self {
        let parent = match parent {
            Some(parent) => parent.raw,
            _ => core::ptr::null_mut(),
        };

        let raw = unsafe {
            bindings::proc_create(
                name.as_ptr() as *const c_types::c_char,
                mode,
                parent,
                &ProcOperationsVtable::<T>::VTABLE,
            )
        };

        Self {
            raw,
        }
    }
}

impl Drop for ProcDirEntry {
    fn drop(&mut self) {
        unsafe {
            bindings::proc_remove(self.raw);
        }
    }
}
