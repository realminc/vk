//! Shared test-binary-only host allocation fault hook.
use std::{
	alloc::{GlobalAlloc, Layout, System},
	cell::Cell,
};

// Test-binary-only allocator. A constant thread-local flag fails exactly one
// actual System allocation/reallocation on the requesting test thread. It never
// changes consumer allocation behavior, other test threads, or deallocation.
struct FailingHostAllocator;

thread_local! {
	pub(crate) static FAIL_NEXT_HOST_ALLOCATION: Cell<bool> = const { Cell::new(false) };
}

fn reject_host_allocation() -> bool {
	FAIL_NEXT_HOST_ALLOCATION
		.try_with(|flag| flag.replace(false))
		.unwrap_or(false)
}

// SAFETY: all non-injected operations preserve System's layout, alignment,
// provenance and allocation/deallocation pairing. Injected requests return NULL
// without acquiring storage, as allowed by GlobalAlloc. The TLS Cell needs no
// allocation/destructor and affects only the calling thread, with no recursion.
unsafe impl GlobalAlloc for FailingHostAllocator {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		if reject_host_allocation() {
			return std::ptr::null_mut();
		}
		// SAFETY: forwards the caller's valid allocator layout unchanged.
		unsafe { System.alloc(layout) }
	}

	unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
		if reject_host_allocation() {
			return std::ptr::null_mut();
		}
		// SAFETY: forwards the caller's valid allocator layout unchanged.
		unsafe { System.alloc_zeroed(layout) }
	}

	unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
		if reject_host_allocation() {
			return std::ptr::null_mut();
		}
		// SAFETY: System acquired pointer under this layout; realloc preserves
		// its ownership on failure and follows the supplied valid new size.
		unsafe { System.realloc(pointer, layout, size) }
	}

	unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
		// SAFETY: System acquired every successful allocation; pairing is unchanged.
		unsafe { System.dealloc(pointer, layout) }
	}
}

#[global_allocator]
static HOST_ALLOCATOR: FailingHostAllocator = FailingHostAllocator;
