use core::ops::{Deref, DerefMut};
use crate::bindings;

extern "C" {
    fn spin_unlock_helper(lock: *const bindings::spinlock_t);
}

pub struct SpinlockGuard<T> {
    pub(crate) inner: T,
    pub(crate) lock: *mut bindings::spinlock,
}

impl<T> Deref for SpinlockGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for SpinlockGuard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> Drop for SpinlockGuard<T> {
    fn drop(&mut self) {
        unsafe {
            spin_unlock_helper(self.lock);
        }
    }
}


