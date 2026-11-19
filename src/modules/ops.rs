//! # Ops
//! 
//! Raw pointer operations
//! 

/// Raw pointer read operations
pub trait RawPtrOpsRead<T> {
    /// Reads a pointers value
    fn get(&self) -> T;
}

impl<T> RawPtrOpsRead<T> for *mut T {
    fn get(&self) -> T {
        unsafe { self.read() }
    }
}

impl<T> RawPtrOpsRead<T> for *const T {
    fn get(&self) -> T {
        unsafe { self.read() }
    }
}
