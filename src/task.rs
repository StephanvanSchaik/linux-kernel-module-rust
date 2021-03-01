use crate::addr_space::AddressSpace;
use crate::bindings;
use crate::types::FromRaw;

extern "C" {
    fn current_helper() -> *mut bindings::task_struct;
}

pub struct Task {
    raw: *mut bindings::task_struct,
}

impl Task {
    pub fn current() -> Self {
        let raw = unsafe {
            current_helper()
        };

        Self {
            raw,
        }
    }

    pub fn with_pid(pid: bindings::pid_t) -> Option<Self> {
        let vpid = unsafe {
            bindings::find_vpid(pid)
        };

        if vpid.is_null() {
            return None;
        }

        let raw = unsafe {
            bindings::pid_task(vpid, bindings::pid_type::PIDTYPE_PID)
        };

        if raw.is_null() {
            return None;
        }

        Some(Self {
            raw,
        })
    }

    pub fn raw(&self) -> *mut bindings::task_struct {
        self.raw
    }

    pub fn mm(&self) -> Option<AddressSpace> {
        let raw = unsafe {
            (*self.raw).mm
        };

        if raw.is_null() {
            return None;
        }

        Some(unsafe {
            AddressSpace::from_raw(raw)
        })
    }

    pub fn active_mm(&self) -> AddressSpace {
        let raw = unsafe {
            (*self.raw).active_mm
        };

        unsafe {
            AddressSpace::from_raw(raw)
        }
    }
}

impl FromRaw<bindings::task_struct> for Task {
    unsafe fn from_raw(raw: *mut bindings::task_struct) -> Self {
        Self {
            raw,
        }
    }
}
