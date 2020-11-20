use crate::addr_space::AddressSpace;
use crate::bindings;
use crate::percpu::PerCpu;
use crate::types::FromRaw;

impl PerCpu<bindings::task_struct> {
    pub fn current_task() -> PerCpu<bindings::task_struct> {
        PerCpu::from_var(unsafe { bindings::current_task })
    }
}

pub struct Task {
    raw: *mut bindings::task_struct,
}

impl Task {
    pub fn current() -> Self {
        PerCpu::current_task().read()
    }

    pub fn mm(&self) -> AddressSpace {
        let raw = unsafe {
            (*self.raw).mm
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
