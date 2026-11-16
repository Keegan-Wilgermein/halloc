//! # Ops
//! 
//! Raw pointer operations
//! 

/// Raw pointer operations
pub trait RawPtrOps<T> {
    /// Reads a pointers value
    fn get(&self) -> T;

    /// Sets a pointers value
    fn set(&self, value: T);
}

impl<T> RawPtrOps<T> for *mut T {
    fn get(&self) -> T {
        unsafe { self.read() }
    }

    fn set(&self, value: T) {
        unsafe { self.write(value) }
    }
}
