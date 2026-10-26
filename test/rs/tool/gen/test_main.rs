use super::*;
#[test]
fn invalid_arguments_fail() {
	for args in [
		vec!["--typo"],
		vec!["--registry"],
		vec!["--registry", "--check"],
	] {
		assert!(parse_args(args.into_iter().map(str::to_owned)).is_err());
	}
}

#[test]
fn generated_generic_impls_and_methods_keep_single_blank_lines() {
	let source = "struct Header<'a>(&'a ());\nimpl<'a> Header<'a> {\nconst S_TYPE: u32 = 1;\n\nfn s_type(&self) -> u32 { Self::S_TYPE }\nfn p_next(&self) {}\n}\nimpl<'a> Default for Header<'a> { fn default() -> Self { Self(&()) } }\n";
	let formatted = format_generated(source).unwrap();
	assert!(formatted.contains("}\n\nimpl<'a> Default"));
	assert!(formatted.contains("const S_TYPE: u32 = 1;\n\n\tfn s_type"));
	assert!(formatted.contains("}\n\n\tfn p_next"));
	assert!(!formatted.contains("\n\n\n"));
	assert_eq!(format_generated(&formatted).unwrap(), formatted);
}
