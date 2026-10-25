use super::*;
#[test]
fn unprotected_provider_wins_over_guarded_alternatives() {
	let mut p = Provisions::new();
	provide(&mut p, "T", vec!["platform-win32".into()]);
	provide(&mut p, "T", vec![]);
	assert_eq!(p["T"], vec![Vec::<String>::new()]);
	assert!(!provide(&mut p, "T", vec!["provisional".into()]));
}
