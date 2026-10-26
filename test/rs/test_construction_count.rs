use super::*;

#[test]
fn native_counts_accept_the_full_width_and_reject_overflow() {
	assert_eq!(count("pValues", 0), Ok(0));
	assert_eq!(count("pValues", u32::MAX as usize), Ok(u32::MAX));
	if usize::BITS > 32 {
		let overflow = u32::MAX as usize + 1;
		assert_eq!(
			count("pValues", overflow),
			Err(ConstructionError::CountOverflow {
				field: "pValues",
				count: overflow,
			})
		);
	}
}

#[test]
fn video_and_wide_counts_use_their_actual_native_widths() {
	assert_eq!(count_as::<u8>("video", 255), Ok(255));
	assert!(matches!(
		count_as::<u8>("video", 256),
		Err(ConstructionError::CountOverflow { count: 256, .. })
	));
	assert_eq!(count_as::<u16>("short", 65535), Ok(65535));
	assert!(count_as::<u16>("short", 65536).is_err());
	assert_eq!(count_as::<u64>("wide", usize::MAX), Ok(usize::MAX as u64));
}
