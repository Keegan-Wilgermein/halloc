//! # halloc
//!
//! A no_std compatible global
//! heap allocator for Rust
//!

#![no_std]

// Mod

pub mod api;
pub(crate) mod alloc;
pub(crate) mod modules;
pub(crate) mod constants;

// Re-exports

pub use api::functions::free;
pub use api::functions::malloc;

pub use constants::HEAP_SIZE;
pub use constants::PAGE_SIZE;
pub use constants::PAGES;
