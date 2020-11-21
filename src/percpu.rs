use crate::types::FromRaw;

pub struct PerCpu<U> {
    var: *mut U,
}

impl<U> PerCpu<U> {
    pub fn from_var(var: *mut U) -> Self {
        Self {
            var,
        }
    }

    pub fn read<T: FromRaw<U>>(&self) -> T {
        let value: u64;

        unsafe {
            asm!(
                "mov {0}, QWORD PTR gs:[{1}]",
                out(reg) value,
                in(reg) self.var,
            );

            T::from_raw(value as *mut U)
        }
    }
}
