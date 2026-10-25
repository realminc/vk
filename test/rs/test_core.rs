//! External contracts for the core module.

#[path = "core/test_abi.rs"]
mod abi;

#[path = "core/test_version.rs"]
mod version;

#[cfg(feature = "loader")]
#[path = "core/test_command.rs"]
mod command;

#[cfg(feature = "loader")]
#[path = "core/test_native.rs"]
mod native;

#[cfg(feature = "loader")]
#[path = "core/test_command.gen.rs"]
mod generated_command;

#[path = "core/test_video.rs"]
mod video;
