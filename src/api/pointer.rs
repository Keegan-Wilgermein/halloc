//! # Pointer
//! 
//! Pointer wrapper for safe
//! reads and writes
//! 

use core::{fmt::Debug, ops::{Deref, DerefMut}};

/// Wrapper around a raw pointer
pub struct Pointer<T>(*mut T);

impl<T> Debug for Pointer<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<T> Deref for Pointer<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl<T> DerefMut for Pointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}

impl<T> From<*mut T> for Pointer<T> {
    fn from(value: *mut T) -> Self {
        Self(value)
    }
}

impl<T> From<Pointer<T>> for *mut T {
    fn from(value: Pointer<T>) -> Self {
        value.0
    }
}

impl<T> Pointer<T> {
    /// Gets the address of the
    /// pointer as a `usize`
    pub fn addr(&self) -> usize {
        self.0.addr()
    }

    /// Returns `true` if
    /// `Pointer<T>` is null
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }
}
