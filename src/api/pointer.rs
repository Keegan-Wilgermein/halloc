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

impl<T> AsRef<T> for Pointer<T> {
    fn as_ref(&self) -> &T {
        &*self
    }
}

impl<T> AsMut<T> for Pointer<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut *self
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

    /// Casts a pointer into a
    /// pointer of another type
    /// 
    /// ## Returns
    /// Returns `Ok()` if the
    /// size of `U` is the same
    /// as `T`, otherwise it
    /// returns `Err()` containing 
    ///the original pointer
    pub fn cast<U>(self)-> Result<Pointer<U>, Self> {
        if size_of::<T>() == size_of::<U>() {
            Ok(self.0.cast::<U>().into())
        } else {
            Err(self)
        }
    }

    /// Casts a pointer into a
    /// pointer of another type
    /// without bounds checking
    pub fn cast_unchecked<U>(&self) -> Pointer<U> {
        self.0.cast::<U>().into()
    }
}
