//! # Alloc
//!
//! Global allocator
//!

use crate::{constants::{HEAP_SIZE, PAGES}, modules::linked_array::linked_array::LinkedArray};

/// Global allocator
pub(crate) static mut ALLOC: Alloc = Alloc::new();

/// Global allocator struct
#[repr(align(8))]
pub(crate) struct Alloc {
    pub(crate) free: LinkedArray<PAGES>,
    pub(crate) heap: [u8; HEAP_SIZE],
}

impl Alloc {
    /// Creates a new `Alloc`
    const fn new() -> Self {
        Self {
            free: LinkedArray::new(),
            heap: [0; HEAP_SIZE],
        }
    }
}
