//! # Node
//! 
//! A linked list node
//! 

use crate::modules::page::Page;

/// A linked list node
#[derive(Clone, Copy)]
pub(crate) struct Node {
    page: Page,
    prev: usize,
    next: usize,
}

impl From<(Page, usize, usize)> for Node {
    fn from(value: (Page, usize, usize)) -> Self {
        Self {
            page: value.0,
            prev: value.1,
            next: value.2,
        }
    }
}

impl Node {
    /// Creates a new `Node`
    /// with a `Page`
    pub(crate) const fn new() -> Self {
        Self {
            page: Page::new(),
            prev: 0,
            next: 0,
        }
    }

    /// Creates a new `Node` from
    /// a next link and a `Page`
    /// heap link
    pub(crate) const fn from(
        prev: usize,
        next: usize,
        heap_idx: usize,
    ) -> Self {
        Self {
            page: Page::from(heap_idx),
            prev,
            next,
        }
    }

    /// Sets `Node.next`
    pub(crate) const fn set_next(&mut self, index: usize) {
        self.next = index;
    }

    /// Returns the next `Node`
    pub(crate) fn next(&self) -> &usize {
        &self.next
    }

    /// Sets `Node.prev`
    pub(crate) const fn set_prev(&mut self, index: usize) {
        self.prev = index;
    }

    /// Returns the previous `Node`
    pub(crate) fn prev(&self) -> &usize {
        &self.prev
    }

    /// Return a reference to the
    /// `Page` contained in the `Node`
    pub(crate) fn page(&self) -> &Page {
        &self.page
    }
}
