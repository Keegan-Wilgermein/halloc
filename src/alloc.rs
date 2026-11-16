//! # Alloc
//!
//! Global allocator
//!

#[cfg(not(feature = "std"))]
use crate::constants::HEAP_SIZE;

/// Global allocator
pub(crate) static mut ALLOC: Alloc = Alloc::new();

/// Global allocator struct
pub(crate) struct Alloc {
    #[cfg(not(feature = "std"))]
    pub(crate) heap: [u8; HEAP_SIZE],
}

impl Alloc {
    /// Creates a new `Alloc`
    const fn new() -> Self {
        Self {
            #[cfg(not(feature = "std"))]
            heap: [0; HEAP_SIZE],
        }
    }
}
