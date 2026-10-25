//! Vulkan ABI types, cached dispatch and native function/method calls.

#[path = "core/constant.gen.rs"]
mod constants;
#[path = "core/dispatch.gen.rs"]
mod dispatch;
#[path = "core/enum.gen.rs"]
mod enums;
#[path = "core/export.gen.rs"]
mod exports;
#[path = "core/extension.gen.rs"]
mod extensions;
#[path = "core/pfn.gen.rs"]
mod pfn;
#[path = "core/type.gen.rs"]
mod types;
pub use exports::*;
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
#[path = "core/command.gen.rs"]
mod operations;
#[cfg(feature = "loader")]
pub use operations::*;

#[cfg(feature = "loader")]
pub use crate::load::{
	enumerate_instance_extension_properties, enumerate_instance_layer_properties,
	enumerate_instance_version,
};
