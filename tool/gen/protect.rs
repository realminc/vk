//! Requirement alternatives and transitive feature protection.

use crate::model::{Registry, TypeKind};
use anyhow::{Result, anyhow};
use std::collections::BTreeMap;

type Provisions = BTreeMap<String, Vec<Vec<String>>>;

pub fn provide(provisions: &mut Provisions, name: &str, mut clause: Vec<String>) -> bool {
	clause.sort();
	clause.dedup();
	let alternatives = provisions.entry(name.into()).or_default();
	if alternatives.iter().any(Vec::is_empty) || alternatives.contains(&clause) {
		return false;
	}
	if clause.is_empty() {
		alternatives.clear();
	}
	alternatives.push(clause);
	alternatives.sort();
	true
}

pub fn resolve(reg: &mut Registry) -> Result<()> {
	for ty in &reg.types {
		if ty.from_video {
			provide(&mut reg.provisions, &ty.c_name, vec![]);
		}
	}
	loop {
		let sources = reg.provisions.clone();
		let mut changed = false;
		for cmd in &reg.commands {
			if let Some(clauses) = sources.get(&cmd.c_name) {
				let deps = cmd
					.parameters
					.iter()
					.map(|p| p.ty.name.as_str())
					.chain(std::iter::once(cmd.return_type.name.as_str()))
					.chain(cmd.alias_of.as_deref());
				for dep in deps {
					for clause in clauses {
						changed |= provide(&mut reg.provisions, dep, clause.clone());
					}
				}
			}
		}
		for ty in &reg.types {
			let Some(clauses) = sources.get(&ty.c_name) else {
				continue;
			};
			let deps: Vec<&str> = match &ty.kind {
				TypeKind::Alias { target } => vec![target],
				TypeKind::Bitmask { base, requires } => std::iter::once(base.as_str())
					.chain(requires.as_deref())
					.collect(),
				TypeKind::BaseType { ty } => vec![&ty.name],
				TypeKind::Struct { members, .. } | TypeKind::Union { members } => {
					members.iter().map(|m| m.ty.name.as_str()).collect()
				}
				TypeKind::FuncPointer {
					return_type,
					parameters,
				} => std::iter::once(return_type.name.as_str())
					.chain(parameters.iter().map(|p| p.ty.name.as_str()))
					.collect(),
				_ => vec![],
			};
			for dep in deps {
				for clause in clauses {
					changed |= provide(&mut reg.provisions, dep, clause.clone());
				}
			}
		}
		if !changed {
			break;
		}
	}
	for ty in &mut reg.types {
		ty.guards = expressions(&reg.provisions, &reg.protections, &ty.c_name, &ty.guards)?;
	}
	for cmd in &mut reg.commands {
		cmd.guards = expressions(&reg.provisions, &reg.protections, &cmd.c_name, &cmd.guards)?;
	}
	for en in &mut reg.enums {
		en.guards = expressions(&reg.provisions, &reg.protections, &en.c_name, &en.guards)?;
		for value in &mut en.values {
			value.guards = expressions(
				&reg.provisions,
				&reg.protections,
				&value.name,
				&value.guards,
			)?;
		}
	}
	for c in &mut reg.constants {
		c.guards = expressions(&reg.provisions, &reg.protections, &c.name, &c.guards)?;
	}
	Ok(())
}

fn expressions(
	provisions: &Provisions,
	protections: &BTreeMap<String, String>,
	name: &str,
	direct: &[String],
) -> Result<Vec<String>> {
	let mut clauses = provisions
		.get(name)
		.cloned()
		.unwrap_or_else(|| vec![vec![]]);
	for clause in &mut clauses {
		for protect in direct {
			clause.push(
				protections
					.get(protect)
					.ok_or_else(|| anyhow!("unknown protection {protect}"))?
					.clone(),
			);
		}
		clause.sort();
		clause.dedup();
	}
	if clauses.iter().any(Vec::is_empty) {
		return Ok(vec![]);
	}
	Ok(
		clauses
			.iter()
			.map(|clause| {
				if clause.len() == 1 {
					return format!("feature = {:?}", clause[0]);
				}
				format!(
					"all({})",
					clause
						.iter()
						.map(|f| format!("feature = {f:?}"))
						.collect::<Vec<_>>()
						.join(", ")
				)
			})
			.collect(),
	)
}

pub fn cfg(guards: &[String]) -> String {
	if guards.is_empty() {
		String::new()
	} else if guards.len() == 1 {
		format!("#[cfg({})]\n", guards[0])
	} else {
		format!("#[cfg(any({}))]\n", guards.join(", "))
	}
}

#[cfg(test)]
#[path = "../../test/rs/tool/gen/test_protect.rs"]
mod tests;
