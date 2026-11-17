//! # Constants
//!
//! Crate wide constants
//!

/// The size of the heap in bytes
/// 
/// Defaults to 64 kilobytes
pub const HEAP_SIZE: usize = PAGE_SIZE * PAGES;

/// The amount of pages in the heap
/// 
/// Default to 64
pub const PAGES: usize = match option_env!("PAGES") {
    Some(string) => match usize::from_str_radix(string, 10) {
        Ok(pages) => pages,
        Err(_) => panic!("PAGES must be a valid usize"),
    },
    None => 64,
};

/// The size of a page in bytes
/// 
/// Default to 1 kilobyte
pub const PAGE_SIZE: usize = match option_env!("PAGE_SIZE") {
    Some(string) => match usize::from_str_radix(string, 10) {
        Ok(size) => size.next_power_of_two(),
        Err(_) => panic!("PAGE_SIZE must be a usize"),
    },
    None => 1024,
};
