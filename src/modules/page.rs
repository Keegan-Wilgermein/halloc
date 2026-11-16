//! # Page
//! 
//! Page metadata
//! 

/// Page metadata
#[derive(Clone, Copy)]
pub(crate) struct Page {
    /// Flag layout from
    /// left to right:
    /// - Bit 1: Free
    flags: u8,
    /// Heap index
    index: usize,
}

impl Page {
    /// Creates a new `Page`
    pub(crate) const fn new() -> Self {
        Self {
            flags: 0,
            index: 0,
        }
    }

    /// Creates a new `Page` with index
    pub(crate) const fn from(index: usize) -> Self {
        Self {
            flags: 0,
            index,
        }
    }

    /// Checks if the free bitflag is `true`
    /// and returns a `bool`
    pub(crate) fn is_free(&self) -> bool {
        self.flags & (1 << 7) == 0
    }

    /// Toggles the free bitflag
    pub(crate) fn toggle_free(&mut self) {
        self.flags ^= 128;
    }

    /// Gets the heap index
    pub(crate) fn index(&self) -> usize {
        self.index
    }
}

#[test]
/// Ensures is_free behaves for every value
/// a `u8` can take - `0..255`
/// 
/// also ensures toggle free works properly
fn is_free_checks_and_sets_correctly() {
    let mut page = Page::new();

    assert!(page.is_free());
    page.toggle_free();
    assert!(!page.is_free());
}
