//! Host-only construction contracts. Raw fields and native calls remain unsafe boundaries.
use crate::StructureType;

/// A checked host record could not be constructed. No native operation occurred.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConstructionError {
	/// A borrowed array exceeds its native count field's integer width.
	CountOverflow { field: &'static str, count: usize },
	/// Parallel arrays do not have the same native count.
	LengthMismatch {
		field: &'static str,
		expected: usize,
		actual: usize,
	},
	/// A single-node insertion received an already-linked node.
	AlreadyLinked,
	/// A public native header no longer has its canonical structure tag.
	UnexpectedStructureType {
		expected: StructureType,
		actual: StructureType,
	},
}

impl std::fmt::Display for ConstructionError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::CountOverflow { field, count } => {
				write!(f, "Vulkan {field} count {count} exceeds its native width")
			}
			Self::LengthMismatch {
				field,
				expected,
				actual,
			} => write!(f, "Vulkan {field} length {actual}, expected {expected}"),
			Self::AlreadyLinked => f.write_str("Vulkan extension node already has a p_next link"),
			Self::UnexpectedStructureType { expected, actual } => {
				write!(f, "Vulkan structure tag {actual:?}, expected {expected:?}")
			}
		}
	}
}

impl std::error::Error for ConstructionError {}

pub(crate) fn count(field: &'static str, count: usize) -> Result<u32, ConstructionError> {
	u32::try_from(count).map_err(|_| ConstructionError::CountOverflow { field, count })
}

/// A registry-declared extension relationship. Implementations are sealed.
///
/// This proves a native chain type relationship, not enabled features, valid raw
/// pointers, tag uniqueness or GPU resource lifetimes.
pub trait Extends<Root>: sealed::ChainNode {}

pub(crate) mod sealed {
	pub trait ChainNode {
		const S_TYPE: crate::StructureType;
		fn s_type(&self) -> crate::StructureType;
		fn p_next(&self) -> *const std::ffi::c_void;
		fn set_p_next(&mut self, p_next: *mut std::ffi::c_void);
		fn address(&mut self) -> *mut std::ffi::c_void;
	}
}

pub(crate) fn prepend<R: sealed::ChainNode, N: Extends<R>>(
	root: &mut R,
	next: &mut N,
) -> Result<(), ConstructionError> {
	for (expected, actual) in [(R::S_TYPE, root.s_type()), (N::S_TYPE, next.s_type())] {
		if actual != expected {
			return Err(ConstructionError::UnexpectedStructureType { expected, actual });
		}
	}
	if !next.p_next().is_null() {
		return Err(ConstructionError::AlreadyLinked);
	}
	next.set_p_next(root.p_next().cast_mut());
	root.set_p_next(next.address());
	Ok(())
}

#[cfg(test)]
#[path = "../../test/rs/test_construction_count.rs"]
mod tests;
