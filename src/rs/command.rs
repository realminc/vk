//! One operation body emits both module-function and method spellings.
//!
//! Input bodies are the canonical implementations. Methods only delegate;
//! neither spelling resolves symbols, owns another table, submits or waits
//! beyond what the documented operation itself explicitly requests.
macro_rules! commands {
	($owner:ident, $context:ident; $(
		$(#[$doc:meta])*
		pub fn $name:ident($($argument:ident: $ty:ty),* $(,)?) -> $result:ty $body:block
	)*) => { $(
		$(#[$doc])*
		pub fn $name($context: &$owner, $($argument: $ty),*) -> $result $body

		impl $owner {
			$(#[$doc])*
			#[inline]
			pub fn $name(&self, $($argument: $ty),*) -> $result {
				$name(self, $($argument),*)
			}
		}
	)* };
	($owner:ident, $context:ident; $(
		$(#[$doc:meta])*
		pub unsafe fn $name:ident($($argument:ident: $ty:ty),* $(,)?) -> $result:ty $body:block
	)*) => { $(
		$(#[$doc])*
		pub unsafe fn $name($context: &$owner, $($argument: $ty),*) -> $result $body

		impl $owner {
			$(#[$doc])*
			#[inline]
			pub unsafe fn $name(&self, $($argument: $ty),*) -> $result {
				// SAFETY: identical caller obligations to the canonical operation.
				unsafe { $name(self, $($argument),*) }
			}
		}
	)* };
}
pub(crate) use commands;
