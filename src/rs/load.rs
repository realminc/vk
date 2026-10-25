//! Open the OS Vulkan library and initialize cached entry-point dispatch.
mod capability;
mod entry;
mod feature;
#[path = "load/feature.gen.rs"]
mod feature_generated;
pub use capability::{Capabilities, CommandSupport};
pub use feature::{FeatureValue, Features};
mod failure;
pub use failure::{CreatedHandle, InitializationFailure};
pub(crate) mod enumerate;
pub use entry::{
	CallError, Entry, LoadError, enumerate_instance_extension_properties,
	enumerate_instance_layer_properties, enumerate_instance_version,
};
