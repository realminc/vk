//! Compare canonical Rust policy with vectors emitted by actual pinned VMA code.
use super::*;

#[test]
fn automatic_preferences_match_the_pinned_vma_oracle() {
	let cases = include_str!("fixtures/policy.gen.tsv");
	let mut count = 0;
	for line in cases.lines().filter(|line| !line.starts_with('#')) {
		let values: Vec<u32> = line
			.split_whitespace()
			.map(|word| word.parse().unwrap())
			.collect();
		assert_eq!(values.len(), 10);
		let info = AllocationCreateInfo {
			usage: [
				MemoryUsage::Explicit,
				MemoryUsage::Auto,
				MemoryUsage::PreferDevice,
				MemoryUsage::PreferHost,
			][values[0] as usize],
			host_access: [
				HostAccess::None,
				HostAccess::SequentialWrite,
				HostAccess::Random,
			][values[1] as usize],
			allow_transfer_instead: values[2] != 0,
			required_flags: vk::MemoryPropertyFlags::from_raw(values[5]),
			preferred_flags: vk::MemoryPropertyFlags::from_raw(values[6]),
			..Default::default()
		};
		let (required, preferred, avoided) =
			preferences(&info, values[3] != 0, Some(values[4] != 0)).unwrap();
		assert_eq!(
			[required.as_raw(), preferred.as_raw(), avoided.as_raw()],
			values[7..],
			"{line}"
		);
		count += 1;
	}
	assert_eq!(count, 1600);
}

#[test]
fn persistent_mapping_is_a_hard_visibility_requirement_and_transfer_fallback_is_explicit() {
	let mut info = AllocationCreateInfo {
		usage: MemoryUsage::Auto,
		persistent_map: true,
		..Default::default()
	};
	let (required, _, _) = preferences(&info, true, None).unwrap();
	assert_ne!(
		required.as_raw() & vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT.as_raw(),
		0
	);
	info.allow_transfer_instead = true;
	assert!(preferences(&info, true, None).is_err());
	info.host_access = HostAccess::SequentialWrite;
	assert!(preferences(&info, true, None).is_err());
	assert!(preferences(&info, true, Some(false)).is_ok());
}
