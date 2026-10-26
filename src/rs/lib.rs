#![cfg_attr(feature = "loader", doc = include_str!("../../README.md"))]
#![cfg_attr(
	not(feature = "loader"),
	doc = "Vulkan ABI declarations and cached nullable dispatch tables. Cargo package: `vkrust`; Rust namespace: `vk`. All binding definitions live at the crate root. Enable `loader` for library loading and the `Instance` / `Device` wrappers; enable `allocator` for the work-in-progress Rust-native allocator based on VMA."
)]

#[path = "constant.gen.rs"]
mod constants;
#[path = "dispatch.gen.rs"]
mod dispatch;
#[path = "enum.gen.rs"]
mod enums;
#[path = "export.gen.rs"]
mod exports;
#[path = "extension.gen.rs"]
mod extensions;
#[path = "pfn.gen.rs"]
mod pfn;
#[path = "type.gen.rs"]
mod types;
#[doc(inline)]
pub use exports::*;
mod construction;
pub use construction::{ConstructionError, Extends};
#[path = "construction.gen.rs"]
mod construction_generated;
mod version;
pub use version::*;

#[cfg(feature = "loader")]
mod device;
#[cfg(feature = "loader")]
mod instance;
#[cfg(feature = "loader")]
pub use device::*;
#[cfg(feature = "loader")]
pub use instance::*;

#[cfg(feature = "loader")]
pub(crate) mod command;

#[cfg(feature = "loader")]
#[path = "command.gen.rs"]
mod operations;
#[cfg(feature = "loader")]
pub use operations::*;

#[cfg(feature = "loader")]
pub use crate::load::{
	enumerate_instance_extension_properties, enumerate_instance_layer_properties,
	enumerate_instance_version,
};

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
