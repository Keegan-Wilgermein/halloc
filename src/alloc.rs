//! # Alloc
//!
//! Global allocator
//!

use crate::{constants::{HEAP_SIZE, PAGE_SIZE, PAGES}, modules::page::Page};

/// Global allocator
pub(crate) static mut ALLOC: Alloc = Alloc::new();

/// Global allocator struct
pub(crate) struct Alloc {
    pub(crate) pages: [Page; PAGES],
    pub(crate) heap: [u8; HEAP_SIZE],
}

impl Alloc {
    /// Creates a new `Alloc`
    const fn new() -> Self {
        Self {
            pages: {
                let mut array = [Page::new(); PAGES];
                let mut i = 0;

                while i < array.len() {
                    array[i] = Page::from(i * PAGE_SIZE);
                    i += 1;
                }
                
                array
            },
            heap: [0; HEAP_SIZE],
        }
    }
}
