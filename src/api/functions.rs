//! # Functions
//!
//! Public standalone functions
//!

use core::ptr::null_mut;

use crate::{RawPtrOps, alloc::ALLOC, constants::PAGES, modules::page::Page};

/// Allocate memory to the heap
///
/// ## Allocation
/// Allocates memory at least equivalent
/// to `bytes`, aligned to the nearest
/// page size greater than or equal to `bytes`
///
/// Page sizes are either
/// 
/// - 16
/// - 32
/// - 64
/// - 128
/// 
/// bytes large
/// 
/// ## Notes
/// `T` is irrelevant to the allocated size,
/// and is only useful to the caller so they
/// can perform operations on the data
/// 
/// It does not guarantee that `T` will
/// fit in the requested `bytes`
pub fn malloc<T>(bytes: usize) -> *mut T {
    let pages = unsafe {
        ALLOC.pages[0..PAGES]
            .as_mut_ptr()
            .cast::<[Page; PAGES]>()
    };

    for (idx, page) in pages.get().iter().enumerate() {
        if page.is_free() {
            let index = page.index();
            unsafe {
                (*ALLOC.pages[idx..idx].as_mut_ptr())
                    .toggle_free();
            }

            return unsafe {
                ALLOC.heap[index..(index + bytes)]
                .as_mut_ptr()
                .cast()
            }
        }
    }

    null_mut::<T>().cast()
}

/// Deallocate memory at the
/// pointer location, taking
/// ownership of the value so it
/// can't be used again
pub fn free<T>(ptr: T) {
    let _ptr = &ptr as *const T;
}

#[cfg(test)]
pub fn get_between(index1: usize, index2: usize) -> &'static [u8] {
    unsafe {
        &ALLOC.heap[index1..index2]
    }
}
