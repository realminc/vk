//! Independent state-transition sequences for VMA's seven-event cache policy.
use super::*;

#[test]
fn repeated_map_unmap_retains_mapping_after_seventh_major_event() {
	let mut state = Hysteresis {
		enabled: true,
		..Default::default()
	};
	for _ in 0..3 {
		state.post_map();
		state.post_unmap();
		assert!(!state.cached);
	}
	state.post_map();
	assert!(state.cached);
	assert_eq!((state.major, state.minor), (0, 0));
}

#[test]
fn allocation_free_churn_releases_extra_mapping_without_user_holds() {
	let mut state = Hysteresis {
		enabled: true,
		cached: true,
		..Default::default()
	};
	for _ in 0..3 {
		state.post_alloc();
		assert!(!state.post_free());
	}
	state.post_alloc();
	assert!(state.post_free());
	assert!(!state.cached);
}

#[test]
fn opposing_work_cancels_hysteresis_evidence_and_disabled_policy_stays_idle() {
	let mut state = Hysteresis {
		enabled: true,
		..Default::default()
	};
	for _ in 0..100 {
		state.post_map();
		state.post_alloc();
		state.post_free();
		assert!(!state.cached);
	}
	let mut disabled = Hysteresis::default();
	for _ in 0..100 {
		disabled.post_map();
		disabled.post_unmap();
		disabled.post_alloc();
		assert!(!disabled.post_free());
	}
	assert!(!disabled.cached);
	assert_eq!((disabled.major, disabled.minor), (0, 0));
}
