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
