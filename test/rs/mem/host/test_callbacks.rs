//! Fixed-capacity callback ledger: tracking performs no Rust global allocation.
use super::vk;
use std::{
	alloc::{GlobalAlloc, Layout, System},
	ffi::c_void,
	sync::Mutex,
};

#[derive(Clone, Copy)]
struct Record {
	pointer: usize,
	layout: Layout,
}

struct State {
	attempts: usize,
	successes: usize,
	frees: usize,
	fail_at: Option<usize>,
	unexpected: usize,
	records: [Option<Record>; 256],
}

pub(crate) struct Tracker(Mutex<State>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Snapshot {
	pub attempts: usize,
	pub successes: usize,
	pub frees: usize,
	pub live: usize,
	pub bytes: usize,
	pub unexpected: usize,
}

impl Tracker {
	pub fn new() -> Self {
		let tracker = Self(Mutex::new(State {
			attempts: 0,
			successes: 0,
			frees: 0,
			fail_at: None,
			unexpected: 0,
			records: [None; 256],
		}));
		// Darwin's pthread Mutex allocates its native lock on first use. Initialize
		// fixture synchronization before tests arm the global allocation fault hook;
		// callback tracking itself must not acquire that runtime storage later.
		drop(tracker.0.lock().unwrap());
		tracker
	}

	pub fn callbacks(&self) -> vk::AllocationCallbacks {
		vk::AllocationCallbacks {
			p_user_data: std::ptr::from_ref(self).cast_mut().cast(),
			pfn_allocation: Some(allocate),
			pfn_reallocation: Some(reallocate),
			pfn_free: Some(free),
			..Default::default()
		}
	}

	pub fn fail_at(&self, attempt: usize) {
		self.0.lock().unwrap().fail_at = Some(attempt);
	}

	pub fn snapshot(&self) -> Snapshot {
		let state = self.0.lock().unwrap();
		Snapshot {
			attempts: state.attempts,
			successes: state.successes,
			frees: state.frees,
			live: state.records.iter().flatten().count(),
			bytes: state
				.records
				.iter()
				.flatten()
				.map(|record| record.layout.size())
				.sum(),
			unexpected: state.unexpected,
		}
	}
}

unsafe extern "system" fn allocate(
	user: *mut c_void,
	size: usize,
	alignment: usize,
	scope: vk::SystemAllocationScope,
) -> *mut c_void {
	// SAFETY: each test retains its Tracker through every matching owner Drop.
	let tracker = unsafe { &*user.cast::<Tracker>() };
	let mut state = tracker.0.lock().unwrap();
	state.attempts += 1;
	if size == 0 || scope != vk::SystemAllocationScope::OBJECT {
		state.unexpected += 1;
		return std::ptr::null_mut();
	}
	if state.fail_at == Some(state.attempts) {
		return std::ptr::null_mut();
	}
	let Ok(layout) = Layout::from_size_align(size, alignment) else {
		state.unexpected += 1;
		return std::ptr::null_mut();
	};
	let Some(index) = state.records.iter().position(Option::is_none) else {
		state.unexpected += 1;
		return std::ptr::null_mut();
	};
	// SAFETY: nonzero validated layout; exact allocation family/layout is retained.
	let pointer = unsafe { System.alloc(layout) };
	if pointer.is_null() {
		return std::ptr::null_mut();
	}
	state.records[index] = Some(Record {
		pointer: pointer as usize,
		layout,
	});
	state.successes += 1;
	pointer.cast()
}

unsafe extern "system" fn free(user: *mut c_void, pointer: *mut c_void) {
	// SAFETY: callback user data is the live shared Tracker, protected by its mutex.
	let tracker = unsafe { &*user.cast::<Tracker>() };
	let mut state = tracker.0.lock().unwrap();
	let Some(index) = state
		.records
		.iter()
		.position(|record| record.is_some_and(|record| record.pointer == pointer as usize))
	else {
		state.unexpected += 1;
		return;
	};
	let record = state.records[index].take().unwrap();
	state.frees += 1;
	// SAFETY: ledger proves the live System allocation and exact original layout.
	unsafe { System.dealloc(pointer.cast(), record.layout) };
}

unsafe extern "system" fn reallocate(
	user: *mut c_void,
	_: *mut c_void,
	_: usize,
	_: usize,
	_: vk::SystemAllocationScope,
) -> *mut c_void {
	// SAFETY: the test deliberately marks any use of reallocation unexpected.
	let tracker = unsafe { &*user.cast::<Tracker>() };
	tracker.0.lock().unwrap().unexpected += 1;
	std::ptr::null_mut()
}
