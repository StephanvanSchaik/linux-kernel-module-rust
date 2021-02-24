use crate::bindings;
use crate::c_types;

extern "C" {
    fn pgd_none_helper(pgd: bindings::pgd_t) -> c_types::c_int;
    fn pgd_bad_helper(pgd: bindings::pgd_t) -> c_types::c_int;
    fn pgd_val_helper(pgd: bindings::pgd_t) -> bindings::pgdval_t;
    fn pgd_set_helper(pgd: *mut bindings::pgd_t, value: bindings::pgdval_t);

    fn pud_none_helper(pud: bindings::pud_t) -> c_types::c_int;
    fn pud_bad_helper(pud: bindings::pud_t) -> c_types::c_int;
    fn pud_val_helper(pud: bindings::pud_t) -> bindings::pudval_t;
    fn pud_set_helper(pud: *mut bindings::pud_t, value: bindings::pudval_t);

    fn pmd_offset_helper(pud: *const bindings::pud_t, va: c_types::c_ulong) -> *mut bindings::pmd_t;
    fn pmd_none_helper(pmd: bindings::pmd_t) -> c_types::c_int;
    fn pmd_bad_helper(pmd: bindings::pmd_t) -> c_types::c_int;
    fn pmd_val_helper(pmd: bindings::pmd_t) -> bindings::pmdval_t;
    fn pmd_set_helper(pmd: *mut bindings::pmd_t, value: bindings::pmdval_t);

    fn pte_offset_map_helper(pmd: *const bindings::pmd_t, va: c_types::c_ulong) -> *mut bindings::pte_t;
    fn pte_unmap_helper(pte: *const bindings::pte_t);
    fn pte_none_helper(pmd: bindings::pte_t) -> c_types::c_int;
    fn pte_val_helper(pmd: bindings::pte_t) -> bindings::pteval_t;
    fn pte_set_helper(pmd: *mut bindings::pte_t, value: bindings::pteval_t);
}

#[cfg(kernel_4_11_0_or_greater)]
extern "C" {
    fn p4d_offset_helper(pgd: *const bindings::pgd_t, va: c_types::c_ulong) -> *mut bindings::p4d_t;
    fn p4d_none_helper(p4d: bindings::p4d_t) -> c_types::c_int;
    fn p4d_bad_helper(p4d: bindings::p4d_t) -> c_types::c_int;
    fn p4d_val_helper(p4d: bindings::p4d_t) -> bindings::p4dval_t;
    fn p4d_set_helper(p4d: *mut bindings::p4d_t, value: bindings::p4dval_t);

    fn pud_offset_helper(p4d: *const bindings::p4d_t, va: c_types::c_ulong) -> *mut bindings::pud_t;
}

#[cfg(not(kernel_4_11_0_or_greater))]
extern "C" {
    fn pud_offset_helper(pgd: *const bindings::pgd_t, va: c_types::c_ulong) -> *mut bindings::pud_t;
}

pub struct PGD {
    pub(crate) raw: *mut bindings::pgd_t,
}

impl PGD {
    pub fn is_none(&self) -> bool {
        unsafe {
            pgd_none_helper(*self.raw) != 0
        }
    }

    pub fn is_bad(&self) -> bool {
        unsafe {
            pgd_bad_helper(*self.raw) != 0
        }
    }

    pub fn value(&self) -> bindings::pgdval_t {
        unsafe {
            pgd_val_helper(*self.raw)
        }
    }

    pub fn set_value(&mut self, value: bindings::pgdval_t) {
        unsafe {
            pgd_set_helper(self.raw, value);
        }
    }

    #[cfg(kernel_4_11_0_or_greater)]
    pub fn map_offset(&self, va: c_types::c_ulong) -> P4D {
        P4D {
            raw: unsafe {
                p4d_offset_helper(self.raw, va)
            },
        }
    }

    #[cfg(not(kernel_4_11_0_or_greater))]
    pub fn map_offset(&self, va: c_types::c_ulong) -> P4D {
        P4D {
            raw: self.raw,
        }
    }
}

#[cfg(kernel_4_11_0_or_greater)]
pub struct P4D {
    raw: *mut bindings::p4d_t,
}

#[cfg(kernel_4_11_0_or_greater)]
impl P4D {
    pub fn is_none(&self) -> bool {
        unsafe {
            p4d_none_helper(*self.raw) != 0
        }
    }

    pub fn is_bad(&self) -> bool {
        unsafe {
            p4d_bad_helper(*self.raw) != 0
        }
    }

    pub fn value(&self) -> bindings::p4dval_t {
        unsafe {
            p4d_val_helper(*self.raw)
        }
    }

    pub fn set_value(&mut self, value: bindings::p4dval_t) {
        unsafe {
            p4d_set_helper(self.raw, value);
        }
    }

    pub fn map_offset(&self, va: c_types::c_ulong) -> PUD {
        PUD {
            raw: unsafe {
                pud_offset_helper(self.raw, va)
            },
        }
    }
}

#[cfg(not(kernel_4_11_0_or_greater))]
pub struct P4D {
    raw: *mut bindings::pgd_t,
}

#[cfg(not(kernel_4_11_0_or_greater))]
impl P4D {
    pub fn is_none(&self) -> bool {
        unsafe {
            pgd_none_helper(*self.raw) != 0
        }
    }

    pub fn is_bad(&self) -> bool {
        unsafe {
            pgd_bad_helper(*self.raw) != 0
        }
    }

    pub fn value(&self) -> bindings::p4dval_t {
        unsafe {
            pgd_val_helper(*self.raw)
        }
    }

    pub fn set_value(&mut self, value: bindings::p4dval_t) {
    }

    pub fn map_offset(&self, va: c_types::c_ulong) -> PUD {
        PUD {
            raw: unsafe {
                pud_offset_helper(self.raw, va)
            },
        }
    }
}

pub struct PUD {
    raw: *mut bindings::pud_t,
}

impl PUD {
    pub fn is_none(&self) -> bool {
        unsafe {
            pud_none_helper(*self.raw) != 0
        }
    }

    pub fn is_bad(&self) -> bool {
        unsafe {
            pud_bad_helper(*self.raw) != 0
        }
    }

    pub fn value(&self) -> bindings::pudval_t {
        unsafe {
            pud_val_helper(*self.raw)
        }
    }

    pub fn set_value(&mut self, value: bindings::pudval_t) {
        unsafe {
            pud_set_helper(self.raw, value);
        }
    }

    pub fn map_offset(&self, va: c_types::c_ulong) -> PMD {
        PMD {
            raw: unsafe {
                pmd_offset_helper(self.raw, va)
            },
        }
    }
}

pub struct PMD {
    raw: *mut bindings::pmd_t,
}

impl PMD {
    pub fn is_none(&self) -> bool {
        unsafe {
            pmd_none_helper(*self.raw) != 0
        }
    }

    pub fn is_bad(&self) -> bool {
        unsafe {
            pmd_bad_helper(*self.raw) != 0
        }
    }

    pub fn value(&self) -> bindings::pmdval_t {
        unsafe {
            pmd_val_helper(*self.raw)
        }
    }

    pub fn set_value(&mut self, value: bindings::pmdval_t) {
        unsafe {
            pmd_set_helper(self.raw, value);
        }
    }

    pub fn map_offset(&self, va: c_types::c_ulong) -> PTE {
        PTE {
            raw: unsafe {
                pte_offset_map_helper(self.raw, va)
            },
        }
    }
}

pub struct PTE {
    raw: *mut bindings::pte_t,
}

impl PTE {
    pub fn is_none(&self) -> bool {
        unsafe {
            pte_none_helper(*self.raw) != 0
        }
    }

    pub fn value(&self) -> bindings::pteval_t {
        unsafe {
            pte_val_helper(*self.raw)
        }
    }

    pub fn set_value(&mut self, value: bindings::pteval_t) {
        unsafe {
            pte_set_helper(self.raw, value);
        }
    }
}

impl Drop for PTE {
    fn drop(&mut self) {
        unsafe {
            pte_unmap_helper(self.raw);
        }
    }
}
