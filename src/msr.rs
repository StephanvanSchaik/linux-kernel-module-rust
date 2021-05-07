use crate::bindings;
use crate::error::{Error, KernelResult};

pub fn rdmsr_safe_on_cpu(
    cpu: u32,
    reg: u32,
) -> KernelResult<u64> {
    let mut hi = 0u32;
    let mut lo = 0u32;

    let result = unsafe {
        bindings::rdmsr_safe_on_cpu(cpu, reg, &mut lo, &mut hi)
    };

    if result != 0 {
        return Err(Error::from_kernel_errno(result));
    }

    Ok((hi as u64) << 32 | (lo as u64))
}

pub fn wrmsr_safe_on_cpu(
    cpu: u32,
    reg: u32,
    value: u64,
) -> KernelResult<()> {
    let hi = (value >> 32) as u32;
    let lo = (value & 0xffffffff) as u32;

    let result = unsafe {
        bindings::wrmsr_safe_on_cpu(cpu, reg, lo, hi)
    };

    if result != 0 {
        return Err(Error::from_kernel_errno(result));
    }

    Ok(())
}
