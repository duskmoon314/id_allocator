#![no_std]

extern crate alloc;

mod allocator;

pub use allocator::{Allocator, IncrementUsizeAllocator};
