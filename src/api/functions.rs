//! # Functions
//!
//! Public standalone functions
//!

use core::ptr::null_mut;

use crate::{PAGE_SIZE, RawPtrOps, alloc::ALLOC, api::pointer::Pointer, constants::PAGES, modules::page::Page};

/// Allocate memory to the heap
///
/// ## Allocation
/// Allocates memory equivalent
/// to `bytes`
/// 
/// ## Optional
/// If `bytes` is `None` then `bytes`
/// will become `size_of::<T>()`, otherwise
/// it will be `bytes` regardless of `T`,
/// in which case T becomes useful only for
/// performing operations on the data
/// 
/// It does not guarantee that `T` will
/// fit in the requested `bytes`
pub fn malloc<T>(bytes: Option<usize>) -> Pointer<T> {
    let bytes = bytes.unwrap_or_else(|| size_of::<T>());

    let pages = unsafe {
        ALLOC.pages[0..PAGES]
            .as_mut_ptr()
            .cast::<[Page; PAGES]>()
    };

    for (idx, page) in pages.get()
        .iter()
        .enumerate() {
        if page.is_free() {
            let index = page.index();

            unsafe {
                (&raw mut ALLOC.pages)
                    .as_mut_unchecked()
                    [idx]
                    .toggle_free();
            }

            return unsafe {
                ALLOC.heap[index..(index + bytes)]
                    .as_mut_ptr()
            }
            .cast::<T>()
            .into()
        }
    }

    null_mut::<T>()
    .cast::<T>()
    .into()
}

/// Deallocate memory at the
/// pointer location, taking
/// ownership of the value so it
/// can't be used again
pub fn free<T>(ptr: impl Into<*mut T>) {
    let ptr_addr = ptr.into().addr();
    let start = unsafe {
        (&raw const ALLOC.heap).addr()
    };

    let offset = (ptr_addr - start) / PAGE_SIZE;
    unsafe {
        (&raw mut ALLOC.pages)
        .as_mut_unchecked()
        [offset]
        .toggle_free();
    }
}

pub fn get_between(index1: usize, index2: usize) -> &'static [u8] {
    unsafe {
        &ALLOC.heap[index1..index2]
    }
}
