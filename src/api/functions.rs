//! # Functions
//!
//! Public standalone functions
//!

use core::ptr::null_mut;

use crate::{PAGE_SIZE, alloc::ALLOC, api::pointer::Pointer, modules::ops::{RawPtrOpsRead}};

/// Allocate memory to the heap
///
/// ## Allocation
/// Allocates memory at least equivalent
/// to `size_of::<T>()`
pub fn malloc<T>() -> Pointer<T> {
    let bytes = size_of::<T>();

    let link_ptr = unsafe {
        (&raw mut ALLOC.free)
    };

    let link = link_ptr.get();

    let page_idx = match link.next_free() {
        Some(idx) => idx,
        None => return null_mut::<T>().into(),
    };

    let heap_idx = link[page_idx]
        .page()
        .index();

    unsafe {
        &mut *link_ptr
    }.unlink(page_idx);

    return unsafe {
        ALLOC.heap[heap_idx..(heap_idx + bytes)]
            .as_mut_ptr()
    }
    .cast::<T>()
    .into();
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
        (&raw mut ALLOC.free)
            .as_mut_unchecked()
    }.link(offset);
}

pub fn get_between(index1: usize, index2: usize) -> &'static [u8] {
    unsafe {
        &ALLOC.heap[index1..index2]
    }
}
