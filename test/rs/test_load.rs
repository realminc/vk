//! External contracts for the load module.

#![cfg(feature = "loader")]

#[path = "load/test_entry.rs"]
mod entry;

#[path = "load/test_dispatch.rs"]
mod dispatch;
