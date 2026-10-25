//! Owned enabled feature records, independent of their input pointer chains.
use super::CallError;
use crate::core as vk;

/// One explicitly captured native feature member. Names use the registry's C
/// spelling, so `shaderInt64` remains recognizable in specification diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeatureValue {
	pub structure: vk::StructureType,
	pub member: &'static str,
	pub enabled: bool,
}

/// Snapshot of exact feature records used to create a device.
///
/// Individual extension and unified core records are distinct representations;
/// this snapshot does not invent equivalence between equal member names. Native
/// type aliases share the same sType. Legacy base features are represented by
/// `PHYSICAL_DEVICE_FEATURES_2`, including default-false base features.
#[derive(Debug)]
pub struct Features {
	values: Vec<FeatureValue>,
	structures: Vec<vk::StructureType>,
	unrecognized: Vec<vk::StructureType>,
}

impl Features {
	/// Input records follow chain order; legacy/default base features follow the
	/// chain. Members within each record follow official XML declaration order.
	pub fn values(&self) -> &[FeatureValue] {
		&self.values
	}

	/// Exact input records not recognized as feature records by this build.
	/// This includes ordinary non-feature device-create records. Their presence
	/// is not a feature claim, and their payload is never interpreted or retained.
	pub fn unrecognized_structures(&self) -> &[vk::StructureType] {
		&self.unrecognized
	}

	/// Read an explicitly recorded member by its exact native representation.
	/// None means unrecorded/unknown, including alternate unified representations.
	/// A false value is an explicitly captured disabled bit, not inferred absence.
	pub fn enabled(&self, structure: vk::StructureType, member: &str) -> Option<bool> {
		self
			.values
			.iter()
			.find(|value| value.structure == structure && value.member == member)
			.map(|value| value.enabled)
	}

	/// Admit a caller-declared set of exact feature-record requirements.
	///
	/// # Errors
	/// Returns FeatureNotEnabled for the first disabled or unrecorded member.
	/// This checks enabled bits only; API/extension support, limits, native object
	/// state and argument-dependent valid usage remain separate obligations.
	pub fn require(
		&self,
		requirements: &[(vk::StructureType, &'static str)],
	) -> Result<(), CallError> {
		for &(structure, member) in requirements {
			if self.enabled(structure, member) != Some(true) {
				return Err(CallError::FeatureNotEnabled { structure, member });
			}
		}
		Ok(())
	}

	pub(super) fn record(
		&mut self,
		structure: vk::StructureType,
		members: &[(&'static str, vk::Bool32)],
	) -> Result<(), CallError> {
		if self.structures.contains(&structure) {
			return Err(CallError::InvalidMetadata(
				"duplicate enabled-feature record",
			));
		}
		self
			.structures
			.try_reserve(1)
			.map_err(|_| CallError::HostAllocationFailed)?;
		self
			.values
			.try_reserve(members.len())
			.map_err(|_| CallError::HostAllocationFailed)?;
		self.structures.push(structure);
		self
			.values
			.extend(members.iter().map(|&(member, value)| FeatureValue {
				structure,
				member,
				enabled: value != vk::FALSE,
			}));
		Ok(())
	}

	/// Caller guarantees every reachable pointer is valid, aligned, readable and
	/// identifies the complete native record declared by its sType.
	pub(super) unsafe fn capture(info: &vk::DeviceCreateInfo) -> Result<Self, CallError> {
		const MAX_NODES: usize = 1024;
		let mut result = Self {
			values: Vec::new(),
			structures: Vec::new(),
			unrecognized: Vec::new(),
		};
		let mut visited = Vec::new();
		let mut node = info.p_next.cast::<vk::BaseInStructure>();
		let mut features2 = false;
		while !node.is_null() {
			if visited.len() == MAX_NODES || visited.contains(&node) {
				return Err(CallError::InvalidMetadata(
					"cyclic or oversized device feature chain",
				));
			}
			visited
				.try_reserve(1)
				.map_err(|_| CallError::HostAllocationFailed)?;
			visited.push(node);
			// SAFETY: caller guarantees a readable header matching the complete record.
			let header = unsafe { &*node };
			if header.s_type == vk::StructureType::PHYSICAL_DEVICE_FEATURES_2 {
				if !info.p_enabled_features.is_null() {
					return Err(CallError::InvalidMetadata(
						"Features2 and p_enabled_features are mutually exclusive",
					));
				}
				features2 = true;
			}
			// SAFETY: complete record validity is the caller's explicit contract.
			if !unsafe { super::feature_generated::capture_node(node, &mut result) }? {
				result
					.unrecognized
					.try_reserve(1)
					.map_err(|_| CallError::HostAllocationFailed)?;
				result.unrecognized.push(header.s_type);
			}
			node = header.p_next;
		}
		if !features2 {
			// SAFETY: the generated base record consists entirely of VkBool32 fields.
			let defaults = unsafe { std::mem::zeroed::<vk::PhysicalDeviceFeatures>() };
			// SAFETY: caller supplies a valid optional base-feature record.
			let base = unsafe { info.p_enabled_features.as_ref() }.unwrap_or(&defaults);
			super::feature_generated::capture_base(base, &mut result)?;
		}
		Ok(result)
	}
}

#[cfg(test)]
#[path = "../../../test/rs/load/test_feature.rs"]
mod tests;
