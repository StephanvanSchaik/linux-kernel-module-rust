use alloc::boxed::Box;
use alloc::vec;

use crate::bindings;
pub use crate::bindings::pt_regs;
use crate::c_types;
use crate::error::{Error, KernelResult};
use crate::types;

use intrusive_collections::container_of;

pub trait ReturnProbeHandler: Sync {
    fn handler(&self, regs: &mut pt_regs) -> i32;
}

#[cfg(kernel_5_11_0_or_greater)]
fn instance_to_retprobe(
    instance: &bindings::kretprobe_instance,
) -> *const bindings::kretprobe {
    unsafe { (*instance.rph) }.rp
}

#[cfg(not(kernel_5_11_0_or_greater))]
fn instance_to_retprobe(
    instance: &bindings::kretprobe_instance,
) -> *const bindings::kretprobe {
    instance.rp
}

unsafe extern "C" fn return_callback<T: ReturnProbeHandler>(
    instance: *mut bindings::kretprobe_instance,
    regs: *mut pt_regs,
) -> c_types::c_int {
    let instance = match instance.as_mut() {
        Some(instance) => instance,
        _ => return -1,
    };

    let storage: *const ReturnProbeStorage<T> = container_of!(instance_to_retprobe(instance), ReturnProbeStorage<T>, probe);
    let storage = match storage.as_ref() {
        Some(probe) => probe,
        _ => return -1,
    };

    let regs = match regs.as_mut() {
        Some(regs) => regs,
        _ => return -1,
    };

    storage.handler.handler(regs)
}

struct ReturnProbeStorage<T: ReturnProbeHandler> {
    handler: Box<T>,
    probe: bindings::kretprobe,
}

pub struct ReturnProbe<T: ReturnProbeHandler> {
    storage: Box<[ReturnProbeStorage<T>]>,
}

unsafe impl<T: ReturnProbeHandler> Sync for ReturnProbe<T> {}

impl<T: ReturnProbeHandler> ReturnProbe<T> {
    pub fn register(
        symbol: types::CStr<'static>,
        max_active: i32,
        handler: T,
    ) -> KernelResult<ReturnProbe<T>> {
        let probe = bindings::kretprobe {
            kp: bindings::kprobe {
                symbol_name: symbol.as_ptr() as *const i8,
                ..Default::default()
            },
            handler: Some(return_callback::<T>),
            maxactive: max_active,
            ..Default::default()
        };
        let handler = Box::new(handler);
        let mut storage = vec![ReturnProbeStorage {
            handler,
            probe,
        }].into_boxed_slice();

        let result = unsafe {
            bindings::register_kretprobe(&mut storage[0].probe)
        };

        if result < 0 {
            return Err(Error::from_kernel_errno(result));
        }

        Ok(Self {
            storage,
        })
    }
}

impl<T: ReturnProbeHandler> Drop for ReturnProbe<T> {
    fn drop(&mut self) {
        unsafe {
            bindings::unregister_kretprobe(&mut self.storage[0].probe);
        }
    }
}
