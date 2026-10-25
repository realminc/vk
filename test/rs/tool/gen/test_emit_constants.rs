use super::*;
#[test]
fn complement_respects_width() {
	assert_eq!(
		constant_value("(~0U)", Some("uint32_t")).unwrap(),
		("u32", "4294967295".into())
	);
	assert_eq!(
		constant_value("(~0ULL)", Some("uint64_t")).unwrap(),
		("u64", "18446744073709551615".into())
	);
	assert_eq!(constant_value("(~2U)", None).unwrap().1, "4294967293");
}

#[test]
fn hexadecimal_suffix_is_not_float() {
	assert_eq!(constant_value("0xFF", None).unwrap(), ("u32", "255".into()));
	assert!(constant_value("UNSUPPORTED_MACRO(3)", None).is_err());
}
