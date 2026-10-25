//! External contracts for the memory module.

#![cfg(feature = "allocator")]

#[path = "memory/test_selector.rs"]
mod selector;

#[path = "memory/test_allocator.rs"]
mod allocator;

#[path = "memory/test_virtual_block.rs"]
mod virtual_block;
