//! External contracts for the memory module.

#![cfg(feature = "allocator")]

#[path = "mem/test_selector.rs"]
mod selector;

#[path = "mem/test_allocator.rs"]
mod allocator;

#[path = "mem/test_virtual_block.rs"]
mod virtual_block;

#[path = "mem/test_image_native.rs"]
mod image;

#[path = "mem/test_virtual_linear.rs"]
mod virtual_linear;

#[path = "mem/test_host_callbacks.rs"]
mod host_callbacks;

#[path = "mem/test_synchronized_native.rs"]
mod synchronized;

#[path = "mem/test_import_native.rs"]
mod import;
