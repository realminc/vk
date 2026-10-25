use super::*;

#[test]
fn dependencies_preserve_precedence_and_parentheses() {
	assert_eq!(
		clauses("A+B,C+(D,E)").unwrap(),
		vec![vec!["A", "B"], vec!["C", "D"], vec!["C", "E"]]
	);
	assert_eq!(
		clauses("(A,B)+(C,D)").unwrap(),
		vec![
			vec!["A", "C"],
			vec!["A", "D"],
			vec!["B", "C"],
			vec!["B", "D"]
		]
	);
	assert_eq!(clauses("A+A").unwrap(), vec![vec!["A"]]);
}

#[test]
fn malformed_and_explosive_dependencies_fail_generation() {
	for input in ["", "()", "A+", "+A", "A,,B", "(A", "A)", "A B", "A|B"] {
		assert!(clauses(input).is_err(), "{input}");
	}
	let expression = ["(A,B)"; 13].join("+");
	assert!(clauses(&expression).is_err());
	assert!(clauses(&format!("{}A{}", "(".repeat(65), ")".repeat(65))).is_err());
}
