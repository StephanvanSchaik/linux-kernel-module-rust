use core::ops::{Deref, DerefMut};
use crate::bindings;

pub struct RwLockRef<T> {
    pub(crate) lock: *mut bindings::rw_semaphore,
    pub(crate) inner: T,
}

impl<T> RwLockRef<T> {
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        unsafe {
            bindings::down_read(self.lock);
        }

        RwLockReadGuard {
            lock: self.lock,
            inner: &self.inner,
        }
    }

    pub fn write(&mut self) -> RwLockWriteGuard<'_, T> {
        unsafe {
            bindings::down_write(self.lock);
        }

        RwLockWriteGuard {
            lock: self.lock,
            inner: &mut self.inner,
        }
    }
}

pub struct RwLockReadGuard<'a, T> {
    pub(crate) lock: *mut bindings::rw_semaphore,
    pub(crate) inner: &'a T,
}

impl<'a, T> Deref for RwLockReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a, T> Drop for RwLockReadGuard<'a, T> {
    fn drop(&mut self) {
        unsafe {
            bindings::up_read(self.lock);
        }
    }
}

pub struct RwLockWriteGuard<'a, T> {
    pub(crate) lock: *mut bindings::rw_semaphore,
    pub(crate) inner: &'a mut T,
}

impl<'a, T> Deref for RwLockWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a, T> DerefMut for RwLockWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}

impl<'a, T> Drop for RwLockWriteGuard<'a, T> {
    fn drop(&mut self) {
        unsafe {
            bindings::up_write(self.lock);
        }
    }
}
