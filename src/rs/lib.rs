//! Vulkan ABI and cached native dispatch, with one implementation per operation.
//!
//! Cargo package: `vkrust`. Rust import namespace: `vk`.
//! Core ABI exports, including StdVideo codec definitions, are available at the crate root.
//!
//! ```
//! let info = vk::InstanceCreateInfo::default();
//! assert_eq!(info.s_type, vk::StructureType::INSTANCE_CREATE_INFO);
//! let _: vk::core::InstanceHandle = vk::InstanceHandle::NULL;
//! ```
pub mod core;
pub use core::*;

#[cfg(feature = "loader")]
pub mod load;
#[cfg(feature = "loader")]
pub use load::{
	CallError, Capabilities, CommandSupport, CreatedHandle, Entry, FeatureValue, Features,
	InitializationFailure, LoadError,
};
#[cfg(feature = "allocator")]
pub mod memory;
#[cfg(feature = "allocator")]
pub use memory::{
	AllocError, AllocRequest, Allocation, Allocator, VirtualAllocation, VirtualBlock,
};
