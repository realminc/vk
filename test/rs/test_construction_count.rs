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
