use crate::bindings;

pub enum CPUFeature {
    LA57 = bindings::X86_FEATURE_LA57 as _,
}

pub struct CPUInfo {
    raw: *const bindings::cpuinfo_x86,
}

impl CPUInfo {
    pub fn boot_cpu() -> Self {
        Self {
            raw: unsafe { &bindings::boot_cpu_data },
        }
    }

    pub fn has_capability(&self, capability: CPUFeature) -> bool {
        let index = capability as usize;
        let word = unsafe { (*self.raw) .__bindgen_anon_1.x86_capability }[index / 32];
        let mask = 1 << (index % 32);

        word & mask == mask
    }
}
