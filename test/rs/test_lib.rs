//! External contracts for the Vulkan binding API.

#[path = "test_abi.rs"]
mod abi;

#[path = "test_construction.rs"]
mod construction;

#[path = "test_construction.gen.rs"]
mod generated_construction;

#[path = "test_version.rs"]
mod version;

#[cfg(feature = "loader")]
#[path = "test_command.rs"]
mod command;

#[cfg(feature = "loader")]
#[path = "test_native.rs"]
mod native;

#[cfg(feature = "loader")]
#[path = "test_command.gen.rs"]
mod generated_command;

#[path = "test_video.rs"]
mod video;
