use crate::bindings;
use crate::c_types;

#[allow(non_upper_case_globals)]
const __START_KERNEL_map: u64 = 0xffffffff80000000;

pub fn virt_to_phys(x: c_types::c_ulong) -> c_types::c_ulong {
    if x < __START_KERNEL_map {
        unsafe { x - bindings::page_offset_base }
    } else {
        unsafe { bindings::phys_base + x - __START_KERNEL_map }
    }
}
