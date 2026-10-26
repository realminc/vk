//! Registry parser. Mixed declaration text is preserved in a small XML tree;
//! comments and API variants never become part of C declarators.

use crate::model::*;
use anyhow::{Context, Result, anyhow, bail, ensure};
use quick_xml::{Reader, events::Event};
use std::{collections::BTreeMap, path::Path};

#[derive(Debug, Default)]
struct Node {
	name: String,
	attrs: BTreeMap<String, String>,
	parts: Vec<Part>,
}

#[derive(Debug)]
enum Part {
	Text(String),
	Child(Node),
}

impl Node {
	fn attr(&self, key: &str) -> Option<&str> {
		self.attrs.get(key).map(String::as_str)
	}

	fn children(&self) -> impl Iterator<Item = &Node> {
		self.parts.iter().filter_map(|p| {
			if let Part::Child(n) = p {
				Some(n)
			} else {
				None
			}
		})
	}

	fn child(&self, name: &str) -> Option<&Node> {
		self.children().find(|n| n.name == name)
	}

	fn text(&self) -> String {
		self
			.parts
			.iter()
			.map(|p| match p {
				Part::Text(t) => t.clone(),
				Part::Child(n) if n.name != "comment" => n.text(),
				_ => String::new(),
			})
			.collect()
	}

	fn child_text(&self, name: &str) -> Option<String> {
		self.child(name).map(Node::text)
	}

	fn admitted(&self) -> bool {
		self
			.attr("api")
			.is_none_or(|a| a.split(',').any(|a| a == "vulkan"))
	}

	fn guards(&self) -> Vec<String> {
		self
			.attr("protect")
			.map(|s| vec![s.to_owned()])
			.unwrap_or_default()
	}
}

fn start_node(e: quick_xml::events::BytesStart<'_>, reader: &Reader<&[u8]>) -> Result<Node> {
	let mut n = Node {
		name: std::str::from_utf8(e.name().as_ref())?.to_owned(),
		..Node::default()
	};
	for a in e.attributes() {
		let a = a?;
		n.attrs.insert(
			std::str::from_utf8(a.key.as_ref())?.to_owned(),
			a.decode_and_unescape_value(reader.decoder())?.into_owned(),
		);
	}
	Ok(n)
}

fn document(input: &str) -> Result<Node> {
	let mut reader = Reader::from_str(input);
	let mut stack = vec![Node::default()];
	loop {
		match reader.read_event()? {
			Event::Start(e) => stack.push(start_node(e, &reader)?),
			Event::Empty(e) => stack
				.last_mut()
				.unwrap()
				.parts
				.push(Part::Child(start_node(e, &reader)?)),
			Event::End(_) => {
				ensure!(stack.len() > 1, "unexpected closing XML element");
				let n = stack.pop().unwrap();
				stack.last_mut().unwrap().parts.push(Part::Child(n));
			}
			Event::Text(t) => stack
				.last_mut()
				.unwrap()
				.parts
				.push(Part::Text(t.unescape()?.into_owned())),
			Event::CData(t) => stack
				.last_mut()
				.unwrap()
				.parts
				.push(Part::Text(std::str::from_utf8(t.as_ref())?.to_owned())),
			Event::Eof => break,
			_ => {}
		}
	}
	ensure!(stack.len() == 1, "unterminated XML document");
	let mut root = stack.pop().unwrap();
	ensure!(root.children().count() == 1, "expected one XML root");
	match root.parts.remove(
		root
			.parts
			.iter()
			.position(|p| matches!(p, Part::Child(_)))
			.unwrap(),
	) {
		Part::Child(n) => Ok(n),
		_ => unreachable!(),
	}
}

pub fn parse_registry(path: &Path, from_video: bool) -> Result<Registry> {
	let xml = std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
	parse(&xml, from_video).with_context(|| format!("parsing {}", path.display()))
}

fn parse(xml: &str, from_video: bool) -> Result<Registry> {
	let root = document(xml)?;
	ensure!(root.name == "registry", "expected registry root");
	let mut reg = Registry::default();
	if let Some(platforms) = root.child("platforms") {
		for platform in platforms.children().filter(|p| p.name == "platform") {
			reg.protections.insert(
				required(platform, "protect")?.into(),
				format!("platform-{}", required(platform, "name")?.replace('_', "-")),
			);
		}
	}
	reg
		.protections
		.insert("VK_ENABLE_BETA_EXTENSIONS".into(), "provisional".into());

	for section in root.children() {
		match section.name.as_str() {
			"types" => {
				for n in section
					.children()
					.filter(|n| n.name == "type" && n.admitted())
				{
					if n.attr("category") == Some("define")
						&& let Some(name) = n.child_text("name")
						&& (name == "VK_HEADER_VERSION"
							|| name == "VK_HEADER_VERSION_COMPLETE"
							|| name.starts_with("VK_API_VERSION_1_")
							|| name.starts_with("VK_STD_VULKAN_VIDEO_CODEC_"))
					{
						let text = n.text();
						let value = text
							.split_once(&name)
							.context("define missing name")?
							.1
							.split("//")
							.next()
							.unwrap()
							.trim()
							.to_owned();
						reg.constants.push(ConstantDecl {
							name,
							value,
							alias_of: None,
							ty: Some("uint32_t".into()),
							guards: vec![],
						});
					}
					if let Some(t) = parse_type(n, from_video)? {
						reg.types.push(t);
					}
				}
			}
			"enums" => {
				let name = required(section, "name")?.to_owned();
				if let Some(kind) = section.attr("type").filter(|k| *k != "constants") {
					ensure!(
						matches!(kind, "enum" | "bitmask"),
						"unsupported enum kind {kind}"
					);
					let backing = if kind == "bitmask" {
						if section.attr("bitwidth") == Some("64") {
							"u64"
						} else {
							"u32"
						}
					} else {
						"i32"
					};
					let mut decl = EnumDecl {
						c_name: name,
						backing: backing.into(),
						values: vec![],
						guards: section.guards(),
					};
					for n in section
						.children()
						.filter(|n| n.name == "enum" && n.admitted())
					{
						decl.values.push(enum_value(n, None)?);
					}
					reg.enums.push(decl);
				} else {
					for n in section
						.children()
						.filter(|n| n.name == "enum" && n.admitted())
					{
						reg.constants.push(constant(n)?);
					}
				}
			}
			"commands" => {
				for n in section
					.children()
					.filter(|n| n.name == "command" && n.admitted())
				{
					reg.commands.push(command(n)?);
				}
			}
			_ => {}
		}
	}
	// Core requirements and extensions contribute to the same enum symbol graph.
	for n in root
		.children()
		.filter(|n| n.name == "feature" && n.admitted())
	{
		let (types, commands) = requirements(n, None, &[], &mut reg)?;
		reg.features.push(FeatureDecl {
			name: required(n, "name")?.into(),
			number: required(n, "number")?.into(),
			requires_types: types,
			requires_commands: commands,
		});
	}
	if let Some(exts) = root.child("extensions") {
		for n in exts.children().filter(|n| n.name == "extension") {
			if !n
				.attr("supported")
				.is_some_and(|s| s.split(',').any(|a| a == "vulkan"))
			{
				continue;
			}
			let number = required(n, "number")?.parse()?;
			let mut guard = Vec::new();
			if let Some(platform) = n.attr("platform") {
				guard.push(if platform == "provisional" {
					"provisional".into()
				} else {
					format!("platform-{}", platform.replace('_', "-"))
				});
			}
			if n.attr("provisional") == Some("true") {
				guard.push("provisional".into());
			}
			let (types, commands) = requirements(n, Some(number), &guard, &mut reg)?;
			reg.extensions.push(ExtensionDecl {
				platform: n.attr("platform").map(str::to_owned),
				provisional: n.attr("provisional") == Some("true"),
				promoted_to: n.attr("promotedto").map(str::to_owned),
				depends: n.attr("depends").map(str::to_owned),
				from_video,
				name: required(n, "name")?.into(),
				number,
				kind: n.attr("type").unwrap_or("instance").into(),
				guards: n.guards(),
				requires_types: types,
				requires_commands: commands,
			});
		}
	}
	Ok(reg)
}

fn required<'a>(n: &'a Node, key: &str) -> Result<&'a str> {
	n.attr(key)
		.ok_or_else(|| anyhow!("{} missing {key}", n.name))
}

fn parse_type(n: &Node, from_video: bool) -> Result<Option<TypeDecl>> {
	let category = n.attr("category");
	if matches!(category, Some("include" | "define")) {
		return Ok(None);
	}
	let name = n
		.attr("name")
		.map(str::to_owned)
		.or_else(|| n.child_text("name"))
		.or_else(|| n.child("proto").and_then(|p| p.child_text("name")))
		.ok_or_else(|| anyhow!("type missing name: {}", n.text()))?;
	let alias = n.attr("alias").map(str::to_owned);
	let kind = if let Some(target) = &alias {
		TypeKind::Alias {
			target: target.clone(),
		}
	} else {
		match category {
			Some("handle") => {
				if n.text().contains("VK_DEFINE_NON_DISPATCHABLE_HANDLE") {
					TypeKind::NonDispatchableHandle
				} else {
					TypeKind::DispatchableHandle
				}
			}
			Some("enum") => TypeKind::Enum,
			Some("bitmask") => TypeKind::Bitmask {
				requires: n
					.attr("requires")
					.or(n.attr("bitvalues"))
					.map(str::to_owned),
				base: n
					.child_text("type")
					.ok_or_else(|| anyhow!("bitmask {name} missing base"))?,
			},
			Some("struct") => TypeKind::Struct {
				members: members(n)?,
				extends: n
					.attr("structextends")
					.map(|s| s.split(',').map(str::to_owned).collect())
					.unwrap_or_default(),
				returned_only: n.attr("returnedonly") == Some("true"),
			},
			Some("union") => TypeKind::Union {
				members: members(n)?,
			},
			Some("funcpointer") => {
				let proto = n
					.child("proto")
					.ok_or_else(|| anyhow!("callback {name} missing proto"))?;
				TypeKind::FuncPointer {
					return_type: declaration(proto)?.1,
					parameters: parameters(n)?,
				}
			}
			Some("basetype") => {
				if n.child("type").is_some() {
					TypeKind::BaseType {
						ty: declaration(n)?.1,
					}
				} else if n.text().contains("void*") || n.text().contains("__IOSurface*") {
					TypeKind::BaseType {
						ty: TypeRef {
							name: "void".into(),
							pointers: vec![false],
							arrays: vec![],
							array_element_const: false,
						},
					}
				} else {
					ensure!(
						n.text().contains("struct") || name == "CAMetalLayer",
						"unsupported basetype {name}"
					);
					TypeKind::ExternalType { requires: None }
				}
			}
			None => TypeKind::ExternalType {
				requires: n.attr("requires").map(str::to_owned),
			},
			Some(other) => bail!("unsupported type category {other} for {name}"),
		}
	};
	Ok(Some(TypeDecl {
		parent: n.attr("parent").map(str::to_owned),
		c_name: name,
		kind,
		guards: n.guards(),
		alias_of: alias,
		feature: None,
		from_video,
	}))
}

fn declaration(n: &Node) -> Result<(String, TypeRef)> {
	let base = n
		.child_text("type")
		.ok_or_else(|| anyhow!("{} has no declaration type: {}", n.name, n.text()))?;
	let name = n.child_text("name").unwrap_or_default();
	let mut before = String::new();
	let mut after = String::new();
	let mut past_name = false;
	for p in &n.parts {
		match p {
			Part::Child(c) if c.name == "name" => past_name = true,
			Part::Child(c) if c.name == "comment" => {}
			p => {
				let text = match p {
					Part::Text(t) => t.clone(),
					Part::Child(c) => c.text(),
				};
				if past_name {
					after.push_str(&text)
				} else {
					before.push_str(&text)
				}
			}
		}
	}
	let chunks: Vec<_> = before.split('*').collect();
	let pointers = chunks[..chunks.len() - 1]
		.iter()
		.map(|c| c.split_whitespace().any(|w| w == "const"))
		.collect();
	let mut arrays = vec![];
	let mut rest = after.trim();
	while let Some(next) = rest.strip_prefix('[') {
		let (extent, tail) = next
			.split_once(']')
			.ok_or_else(|| anyhow!("unclosed array on {name}"))?;
		ensure!(!extent.trim().is_empty(), "unsized array on {name}");
		arrays.push(extent.trim().to_owned());
		rest = tail.trim();
	}
	Ok((
		name,
		TypeRef {
			name: base,
			pointers,
			arrays,
			array_element_const: chunks
				.last()
				.unwrap()
				.split_whitespace()
				.any(|s| s == "const"),
		},
	))
}

fn members(n: &Node) -> Result<Vec<StructMember>> {
	n.children()
		.filter(|m| m.name == "member" && m.admitted())
		.map(|m| {
			let (name, ty) = declaration(m)?;
			let bitfield = m
				.text()
				.split_once(':')
				.map(|(_, s)| s.trim().parse())
				.transpose()
				.with_context(|| format!("bitfield {name}"))?;
			Ok(StructMember {
				name,
				ty,
				values: m.attr("values").map(str::to_owned),
				len: m.attr("len").map(str::to_owned),
				optional: m.attr("optional").and_then(|s| s.split(',').next()) == Some("true"),
				alt_len: m.attr("altlen").map(str::to_owned),
				selector: m.attr("selector").map(str::to_owned),
				selection: m.attr("selection").map(str::to_owned),
				no_auto_validity: m.attr("noautovalidity").is_some(),
				bitfield,
			})
		})
		.collect()
}

fn parameters(n: &Node) -> Result<Vec<Parameter>> {
	n.children()
		.filter(|p| p.name == "param" && p.admitted())
		.map(|p| {
			let (name, ty) = declaration(p)?;
			Ok(Parameter {
				name,
				ty,
				len: p.attr("len").map(str::to_owned),
				optional: p.attr("optional") == Some("true"),
				extern_sync: p.attr("externsync").is_some(),
			})
		})
		.collect()
}

fn command(n: &Node) -> Result<CommandDecl> {
	let alias = n.attr("alias").map(str::to_owned);
	let (name, return_type) = if alias.is_some() {
		(
			required(n, "name")?.into(),
			TypeRef {
				name: "void".into(),
				pointers: vec![],
				arrays: vec![],
				array_element_const: false,
			},
		)
	} else {
		declaration(
			n.child("proto")
				.ok_or_else(|| anyhow!("command missing proto"))?,
		)?
	};
	Ok(CommandDecl {
		c_name: name,
		rust_name: String::new(),
		return_type,
		success_codes: n
			.attr("successcodes")
			.map(|codes| codes.split(',').map(str::to_owned).collect())
			.unwrap_or_default(),
		parameters: parameters(n)?,
		dispatch: DispatchLevel::Global,
		alias_of: alias,
		guards: n.guards(),
		feature: None,
	})
}

fn constant(n: &Node) -> Result<ConstantDecl> {
	Ok(ConstantDecl {
		name: required(n, "name")?.into(),
		value: n.attr("value").unwrap_or("").into(),
		alias_of: n.attr("alias").map(str::to_owned),
		ty: n.attr("type").map(str::to_owned),
		guards: n.guards(),
	})
}

fn enum_value(n: &Node, ext: Option<u32>) -> Result<EnumValue> {
	let alias = n.attr("alias").map(str::to_owned);
	let value = if let Some(v) = n.attr("value") {
		v.to_owned()
	} else if let Some(bit) = n.attr("bitpos") {
		1u64
			.checked_shl(bit.parse()?)
			.ok_or_else(|| anyhow!("bit position out of range"))?
			.to_string()
	} else if let Some(offset) = n.attr("offset") {
		let ext = n
			.attr("extnumber")
			.map(str::parse)
			.transpose()?
			.or(ext)
			.ok_or_else(|| anyhow!("enum offset missing extension number"))?;
		ensure!(ext > 0, "extension number must be positive");
		let value = 1_000_000_000i64 + (i64::from(ext) - 1) * 1000 + offset.parse::<i64>()?;
		(if n.attr("dir") == Some("-") {
			-value
		} else {
			value
		})
		.to_string()
	} else {
		ensure!(
			alias.is_some(),
			"enum {} missing value",
			required(n, "name")?
		);
		String::new()
	};
	Ok(EnumValue {
		name: required(n, "name")?.into(),
		value,
		alias_of: alias,
		guards: n.guards(),
		comment: n.attr("comment").map(str::to_owned),
		extension_number: ext,
	})
}

fn requirements(
	n: &Node,
	ext: Option<u32>,
	base: &[String],
	reg: &mut Registry,
) -> Result<(Vec<String>, Vec<String>)> {
	let mut types = vec![];
	let mut commands = vec![];
	for req in n.children().filter(|n| n.name == "require" && n.admitted()) {
		for e in req.children().filter(|e| e.admitted()) {
			if let Some(name) = e.attr("name") {
				let mut guard = base.to_vec();
				for node in [req, e] {
					if let Some(protect) = node.attr("protect") {
						guard.push(
							reg
								.protections
								.get(protect)
								.ok_or_else(|| anyhow!("unknown protection {protect}"))?
								.clone(),
						);
					}
				}
				crate::protect::provide(&mut reg.provisions, name, guard);
			}
			match e.name.as_str() {
				"type" => types.push(required(e, "name")?.into()),
				"command" => {
					let name = required(e, "name")?;
					commands.push(name.into());
					let mut expression = format!("({})", required(n, "name")?);
					for dependency in [n.attr("depends"), req.attr("depends")]
						.into_iter()
						.flatten()
					{
						expression.push_str(&format!("+({dependency})"));
					}
					reg
						.command_requirements
						.entry(name.into())
						.or_default()
						.push(expression);
				}
				"enum" if e.attr("extends").is_some() => {
					let name = required(e, "extends")?;
					let i = reg
						.enums
						.iter()
						.position(|d| d.c_name == name)
						.ok_or_else(|| anyhow!("enum contribution to undefined {name}"))?;
					reg.enums[i].values.push(enum_value(e, ext)?);
				}
				"enum" if e.attr("value").is_some() || e.attr("alias").is_some() => {
					reg.constants.push(constant(e)?)
				}
				_ => {}
			}
		}
	}
	Ok((types, commands))
}

#[cfg(test)]
#[path = "../../test/rs/tool/gen/test_parse.rs"]
mod tests;
