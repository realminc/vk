//! Compile registry dependency expressions into bounded disjunctive clauses.
use anyhow::{Result, bail, ensure};

type Clauses = Vec<Vec<String>>;

pub fn clauses(input: &str) -> Result<Clauses> {
	ensure!(
		input.len() <= 65536,
		"dependency expression exceeds 65536 bytes"
	);
	let mut parser = Parser {
		input: input.as_bytes(),
		position: 0,
		depth: 0,
	};
	let result = parser.alternatives()?;
	ensure!(
		parser.position == parser.input.len(),
		"trailing dependency expression: {input}"
	);
	Ok(result)
}

struct Parser<'a> {
	input: &'a [u8],
	position: usize,
	depth: usize,
}

impl Parser<'_> {
	fn alternatives(&mut self) -> Result<Clauses> {
		let mut result = self.conjunction()?;
		while self.take(b',') {
			result.extend(self.conjunction()?);
			ensure!(
				result.len() <= 4096,
				"dependency expression exceeds 4096 clauses"
			);
		}
		Ok(result)
	}

	fn conjunction(&mut self) -> Result<Clauses> {
		let mut result = self.atom()?;
		while self.take(b'+') {
			let next = self.atom()?;
			ensure!(
				result
					.len()
					.checked_mul(next.len())
					.is_some_and(|n| n <= 4096),
				"dependency expression exceeds 4096 clauses"
			);
			result = result
				.iter()
				.flat_map(|left| {
					next.iter().map(move |right| {
						let mut clause = left.clone();
						for token in right {
							if !clause.contains(token) {
								clause.push(token.clone());
							}
						}
						clause
					})
				})
				.collect();
		}
		Ok(result)
	}

	fn atom(&mut self) -> Result<Clauses> {
		if self.take(b'(') {
			self.depth += 1;
			ensure!(self.depth <= 64, "dependency nesting exceeds 64 levels");
			let result = self.alternatives()?;
			ensure!(self.take(b')'), "unclosed dependency expression");
			self.depth -= 1;
			return Ok(result);
		}
		let start = self.position;
		while self
			.input
			.get(self.position)
			.is_some_and(|c| c.is_ascii_alphanumeric() || *c == b'_')
		{
			self.position += 1;
		}
		if start == self.position {
			bail!("expected dependency token at byte {}", self.position);
		}
		Ok(vec![vec![
			std::str::from_utf8(&self.input[start..self.position])?.into(),
		]])
	}

	fn take(&mut self, token: u8) -> bool {
		if self.input.get(self.position) == Some(&token) {
			self.position += 1;
			true
		} else {
			false
		}
	}
}

#[cfg(test)]
#[path = "../../test/rs/tool/gen/test_capability.rs"]
mod tests;
