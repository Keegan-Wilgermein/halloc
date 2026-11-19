//! # Page
//! 
//! Page metadata
//! 

/// Page metadata
#[derive(Clone, Copy)]
pub(crate) struct Page {
    /// Heap index
    index: usize,
}

impl Page {
    /// Creates a new `Page`
    pub(crate) const fn new() -> Self {
        Self {
            index: 0,
        }
    }

    /// Creates a new `Page` with index
    pub(crate) const fn from(index: usize) -> Self {
        Self {
            index,
        }
    }

    /// Gets the heap index
    pub(crate) fn index(&self) -> usize {
        self.index
    }
}
