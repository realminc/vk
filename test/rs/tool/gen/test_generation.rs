//! End-to-end freshness and failure checks for the actual generator binary.
use std::{
	path::{Path, PathBuf},
	process::{Command, Output},
};

// Compile the actual emitted metadata, without the generator's parser/model or
// runtime capability evaluator, for an independent pinned-XML differential test.
#[allow(dead_code)]
#[path = "../../../../src/rs/extension.gen.rs"]
mod emitted;

#[derive(Debug)]
enum Dependency {
	Token(String),
	All(Vec<Self>),
	Any(Vec<Self>),
}

impl Dependency {
	fn parse(source: &str) -> Self {
		// Independent reference: split at top-level operators, never expand DNF.
		for (operator, conjunction) in [(',', false), ('+', true)] {
			let mut depth = 0;
			let mut start = 0;
			let mut parts = Vec::new();
			for (index, character) in source.char_indices() {
				match character {
					'(' => depth += 1,
					')' => depth -= 1,
					_ => {}
				}
				if depth == 0 && character == operator {
					parts.push(Self::parse(&source[start..index]));
					start = index + 1;
				}
			}
			if !parts.is_empty() {
				parts.push(Self::parse(&source[start..]));
				return if conjunction {
					Self::All(parts)
				} else {
					Self::Any(parts)
				};
			}
		}
		if source.starts_with('(') {
			return Self::parse(&source[1..source.len() - 1]);
		}
		Self::Token(source.to_owned())
	}

	fn evaluate(&self, enabled: &impl Fn(&str) -> bool) -> bool {
		match self {
			Self::Token(token) => enabled(token),
			Self::All(parts) => parts.iter().all(|part| part.evaluate(enabled)),
			Self::Any(parts) => parts.iter().any(|part| part.evaluate(enabled)),
		}
	}
}

#[test]
fn command_requirements_match_independent_xml_boolean_oracle() {
	use quick_xml::{Reader, events::Event};
	use std::collections::BTreeMap;
	let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
	let xml = std::fs::read_to_string(root.join("registry/vk.xml")).unwrap();
	let mut reader = Reader::from_str(&xml);
	let mut versions = BTreeMap::<String, u32>::new();
	let mut scopes = BTreeMap::<String, String>::new();
	let mut providers = BTreeMap::<String, Vec<Dependency>>::new();
	let mut provider: Option<(String, Option<String>)> = None;
	let mut requirement: Option<Option<String>> = None;
	loop {
		let node = match reader.read_event().unwrap() {
			Event::Start(node) | Event::Empty(node) => node,
			Event::End(node) => {
				match node.name().as_ref() {
					b"feature" | b"extension" => provider = None,
					b"require" => requirement = None,
					_ => {}
				}
				continue;
			}
			Event::Eof => break,
			_ => continue,
		};
		let attributes: BTreeMap<_, _> = node
			.attributes()
			.map(|attribute| {
				let attribute = attribute.unwrap();
				(
					String::from_utf8(attribute.key.as_ref().to_vec()).unwrap(),
					attribute
						.decode_and_unescape_value(reader.decoder())
						.unwrap()
						.into_owned(),
				)
			})
			.collect();
		let vulkan = |key: &str| {
			attributes
				.get(key)
				.is_none_or(|value| value.split(',').any(|api| api == "vulkan"))
		};
		match node.name().as_ref() {
			b"feature" if attributes.contains_key("number") && vulkan("api") => {
				let name = attributes["name"].clone();
				let (major, minor) = attributes["number"].split_once('.').unwrap();
				versions.insert(
					name.clone(),
					(major.parse::<u32>().unwrap() << 22) | (minor.parse::<u32>().unwrap() << 12),
				);
				provider = Some((name, attributes.get("depends").cloned()));
			}
			b"extension" if vulkan("supported") => {
				let name = attributes["name"].clone();
				scopes.insert(name.clone(), attributes["type"].clone());
				provider = Some((name, attributes.get("depends").cloned()));
			}
			b"require" if provider.is_some() && vulkan("api") => {
				requirement = Some(attributes.get("depends").cloned())
			}
			b"command" if vulkan("api") => {
				if let (Some((name, dependency)), Some(conditional)) = (&provider, &requirement) {
					let mut parts = vec![Dependency::Token(name.clone())];
					parts.extend(
						dependency
							.iter()
							.chain(conditional.iter())
							.map(|source| Dependency::parse(source)),
					);
					providers
						.entry(attributes["name"].clone())
						.or_default()
						.push(Dependency::All(parts));
				}
			}
			_ => {}
		}
	}
	for api_minor in 0..=4 {
		let api_version = (1 << 22) | (api_minor << 12);
		for seed in 0..32_u64 {
			let enabled = |name: &str| {
				if let Some(version) = versions.get(name) {
					return api_version >= *version;
				}
				assert!(scopes.contains_key(name), "unknown XML requirement {name}");
				// Mix independent instance/device names into several reproducible
				// enablement sets; include none/all for boundary coverage.
				let hash = name.bytes().fold(seed, |hash, byte| {
					hash.wrapping_mul(131).wrapping_add(u64::from(byte))
				});
				seed == 1 || (seed != 0 && hash % 5 < 2)
			};
			for command in emitted::COMMANDS {
				let expected = providers.get(command.name).is_some_and(|alternatives| {
					alternatives
						.iter()
						.any(|expression| expression.evaluate(&enabled))
				});
				let actual = command.requirements.iter().any(|clause| {
					clause.iter().all(|atom| match atom {
						emitted::CapabilityRequirement::Core(version) => api_version >= *version,
						emitted::CapabilityRequirement::InstanceExtension(name) => {
							assert_eq!(scopes[name.to_str().unwrap()], "instance");
							enabled(name.to_str().unwrap())
						}
						emitted::CapabilityRequirement::DeviceExtension(name) => {
							assert_eq!(scopes[name.to_str().unwrap()], "device");
							enabled(name.to_str().unwrap())
						}
					})
				});
				assert_eq!(
					actual, expected,
					"{} API1.{api_minor}, seed {seed}",
					command.name
				);
			}
		}
	}
}
struct Temp(PathBuf);

#[test]
fn every_command_and_alias_through_vulkan_1_4_has_a_public_operation() {
	use quick_xml::{Reader, events::Event};
	use std::collections::{BTreeMap, BTreeSet};
	let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
	let xml = std::fs::read_to_string(root.join("registry/vk.xml")).unwrap();
	let mut reader = Reader::from_str(&xml);
	let mut in_core = false;
	let mut commands = BTreeSet::new();
	let mut aliases = BTreeMap::new();
	loop {
		let node = match reader.read_event().unwrap() {
			Event::Start(node) | Event::Empty(node) => node,
			Event::End(node) if node.name().as_ref() == b"feature" => {
				in_core = false;
				continue;
			}
			Event::Eof => break,
			_ => continue,
		};
		let attributes: BTreeMap<_, _> = node
			.attributes()
			.map(|attribute| {
				let attribute = attribute.unwrap();
				(
					String::from_utf8(attribute.key.as_ref().to_vec()).unwrap(),
					attribute
						.decode_and_unescape_value(reader.decoder())
						.unwrap()
						.into_owned(),
				)
			})
			.collect();
		if node.name().as_ref() == b"command"
			&& let Some(alias) = attributes.get("alias")
		{
			aliases.insert(attributes["name"].clone(), alias.clone());
		}
		if node.name().as_ref() == b"feature" {
			in_core = attributes
				.get("number")
				.is_some_and(|number| matches!(number.as_str(), "1.0" | "1.1" | "1.2" | "1.3" | "1.4"))
				&& attributes
					.get("api")
					.is_some_and(|api| api.split(',').any(|api| api == "vulkan"));
		} else if in_core
			&& node.name().as_ref() == b"command"
			&& attributes
				.get("api")
				.is_none_or(|api| api.split(',').any(|api| api == "vulkan"))
		{
			commands.insert(attributes["name"].clone());
		}
	}
	assert!(
		commands.len() == 234,
		"pinned Vulkan 1.0/1.1/1.2/1.3/1.4 command inventory changed"
	);
	let promoted: Vec<_> = aliases
		.into_iter()
		.filter(|(_, target)| commands.contains(target))
		.map(|(name, _)| name)
		.collect();
	assert_eq!(
		promoted.len(),
		100,
		"pinned promoted through-1.4 aliases changed"
	);
	commands.extend(promoted);
	// Read actual nullable PFN fields to obtain native/Rust spelling pairs;
	// do not reuse the generator's naming converter or recipe inventory.
	let dispatch = std::fs::read_to_string(root.join("src/rs/dispatch.gen.rs")).unwrap();
	let words: Vec<_> = dispatch.split_whitespace().collect();
	let fields: BTreeMap<_, _> = words
		.windows(3)
		.filter_map(|words| {
			if words[0] != "pub" {
				return None;
			}
			Some((
				words[2].strip_prefix("PFN_")?.trim_end_matches(','),
				words[1].strip_suffix(':')?,
			))
		})
		.collect();
	let mut operations = BTreeSet::new();
	for source in [
		"load/entry.rs",
		"instance.rs",
		"device.rs",
		"command.gen.rs",
	] {
		let source = std::fs::read_to_string(root.join("src/rs").join(source)).unwrap();
		for line in source.lines() {
			let line = line.trim_start();
			if let Some(declaration) = line
				.strip_prefix("pub unsafe fn ")
				.or_else(|| line.strip_prefix("pub fn "))
			{
				operations.insert(declaration.split('(').next().unwrap().to_owned());
			}
		}
	}
	let missing: Vec<_> = commands
		.iter()
		.filter(|command| {
			!fields
				.get(command.as_str())
				.is_some_and(|field| operations.contains(*field))
		})
		.collect();
	assert!(
		missing.is_empty(),
		"Vulkan 1.0/1.1/1.2/1.3/1.4 operations missing: {missing:?}"
	);
	assert_eq!(
		fields.len(),
		842,
		"pinned callable command inventory changed"
	);
	let missing: Vec<_> = fields
		.iter()
		.filter(|(_, field)| !operations.contains(**field))
		.collect();
	assert!(
		missing.is_empty(),
		"registry command operations missing: {missing:?}"
	);
}

impl Temp {
	fn new() -> Self {
		static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
		let p = std::env::temp_dir().join(format!(
			"vk-gen-test-{}-{}",
			std::process::id(),
			NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
		));
		std::fs::create_dir_all(&p).unwrap();
		Self(p)
	}
}

impl Drop for Temp {
	fn drop(&mut self) {
		let _ = std::fs::remove_dir_all(&self.0);
	}
}

fn run(output: &Path, check: bool, registry: &Path) -> Output {
	let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
	let mut cmd = Command::new(env!("CARGO_BIN_EXE_vk-gen"));
	cmd
		.arg("--registry")
		.arg(registry)
		.arg("--video-registry")
		.arg(root.join("registry/video.xml"))
		.arg("--output")
		.arg(output);
	if check {
		cmd.arg("--check");
	}
	cmd.output().unwrap()
}

fn snapshot(root: &Path) -> Vec<(PathBuf, Vec<u8>, std::time::SystemTime)> {
	let mut files = Vec::new();
	for entry in std::fs::read_dir(root).unwrap() {
		let path = entry.unwrap().path();
		if path.is_dir() {
			files.extend(snapshot(&path));
		} else {
			files.push((
				path.clone(),
				std::fs::read(&path).unwrap(),
				path.metadata().unwrap().modified().unwrap(),
			));
		}
	}
	files.sort_by(|a, b| a.0.cmp(&b.0));
	files
}

#[test]
fn generated_output_is_checked_without_writes() {
	let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
	let temp = Temp::new();
	let out = temp.0.join("generated");
	let xml = root.join("registry/vk.xml");
	assert!(!run(&out, true, &xml).status.success());
	assert!(!out.exists());
	let result = run(&out, false, &xml);
	assert!(
		result.status.success(),
		"{}",
		String::from_utf8_lossy(&result.stderr)
	);
	assert!(run(&out, true, &xml).status.success());
	let before = snapshot(&out);
	assert!(run(&out, false, &xml).status.success());
	assert_eq!(
		snapshot(&out),
		before,
		"repeat generation must preserve bytes and mtimes"
	);
	for name in [
		"src/rs/command.gen.rs",
		"src/rs/construction.gen.rs",
		"test/rs/test_construction.gen.rs",
		"tool/gen/construction.gen.tsv",
		"src/rs/load/feature.gen.rs",
		"test/rs/test_command.gen.rs",
	] {
		let path = out.join(name);
		let bytes = std::fs::read(&path).unwrap();
		std::fs::write(&path, b"stale generated API artifact").unwrap();
		assert!(!run(&out, true, &xml).status.success());
		assert_eq!(
			std::fs::read(&path).unwrap(),
			b"stale generated API artifact"
		);
		std::fs::write(&path, bytes).unwrap();
	}
	let generated_test = out.join("test/rs/test_command.gen.rs");
	let test_bytes = std::fs::read(&generated_test).unwrap();
	std::fs::remove_file(&generated_test).unwrap();
	assert!(!run(&out, true, &xml).status.success());
	assert!(!generated_test.exists());
	std::fs::write(&generated_test, test_bytes).unwrap();
	let file = out.join("src/rs/pfn.gen.rs");
	let bytes = std::fs::read(&file).unwrap();
	std::fs::write(&file, b"stale output").unwrap();
	assert!(!run(&out, true, &xml).status.success());
	assert_eq!(std::fs::read(&file).unwrap(), b"stale output");
	std::fs::write(&file, bytes).unwrap();
	std::fs::remove_file(&file).unwrap();
	assert!(!run(&out, true, &xml).status.success());
	assert!(!file.exists());
	let bad = temp.0.join("missing-command.xml");
	std::fs::write(&bad, "<registry/>").unwrap();
	assert!(!run(&out, true, &bad).status.success());
}

#[derive(Default)]
struct RegistryOrder {
	types: Vec<String>,
	enums: Vec<String>,
	constants: Vec<String>,
	commands: Vec<String>,
	extensions: Vec<String>,
}

fn registry_order(path: &Path) -> RegistryOrder {
	use quick_xml::{Reader, events::Event};
	let xml = std::fs::read_to_string(path).unwrap();
	let mut reader = Reader::from_str(&xml);
	let mut stack = Vec::<String>::new();
	let mut order = RegistryOrder::default();
	loop {
		let (node, push) = match reader.read_event().unwrap() {
			Event::Start(node) => (node, true),
			Event::Empty(node) => (node, false),
			Event::End(_) => {
				stack.pop().unwrap();
				continue;
			}
			Event::Eof => break,
			_ => continue,
		};
		let name = std::str::from_utf8(node.name().as_ref())
			.unwrap()
			.to_owned();
		let attribute = |key: &[u8]| {
			node
				.attributes()
				.map(|a| a.unwrap())
				.find(|a| a.key.as_ref() == key)
				.map(|a| String::from_utf8(a.value.into_owned()).unwrap())
		};
		let context: Vec<_> = stack.iter().map(String::as_str).collect();
		if context == ["registry", "types"] && name == "type" {
			if let Some(name) = attribute(b"name") {
				order.types.push(name);
			}
		} else if context == ["registry", "types", "type"] && name == "name" {
			let value = reader
				.read_text(quick_xml::name::QName(b"name"))
				.unwrap()
				.into_owned();
			if value.starts_with("VK_") {
				order.constants.push(value.clone());
			}
			order.types.push(value);
			continue;
		} else if context == ["registry"] && name == "enums" {
			if attribute(b"type").is_some_and(|kind| kind == "enum" || kind == "bitmask") {
				order.enums.push(attribute(b"name").unwrap());
			}
		} else if name == "enum" {
			if let Some(name) = attribute(b"name") {
				order.constants.push(name);
			}
		} else if context == ["registry", "extensions"] && name == "extension" {
			if let Some(name) = attribute(b"name") {
				order.extensions.push(name);
			}
		} else if context == ["registry", "commands"] && name == "command" {
			if let Some(name) = attribute(b"name") {
				order.commands.push(name);
			}
		} else if context == ["registry", "commands", "command", "proto"] && name == "name" {
			order.commands.push(
				reader
					.read_text(quick_xml::name::QName(b"name"))
					.unwrap()
					.into_owned(),
			);
			continue;
		}
		if push {
			stack.push(name);
		}
	}
	order
}

fn rust_type_name(native: &str) -> String {
	match native {
		"VkInstance" => "InstanceHandle".into(),
		"VkDevice" => "DeviceHandle".into(),
		name => name.strip_prefix("Vk").unwrap_or(name).into(),
	}
}

fn assert_registry_order(
	actual: Vec<String>,
	native: impl IntoIterator<Item = String>,
	family: &str,
) {
	let admitted: std::collections::HashSet<_> = actual.iter().cloned().collect();
	let mut seen = std::collections::HashSet::new();
	let expected: Vec<_> = native
		.into_iter()
		.filter(|name| admitted.contains(name) && seen.insert(name.clone()))
		.collect();
	assert!(!actual.is_empty(), "empty {family} inventory");
	assert_eq!(
		actual, expected,
		"{family} must follow pinned XML declaration order"
	);
}

#[test]
fn generated_families_follow_pinned_xml_declaration_order() {
	let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
	let temp = Temp::new();
	let out = temp.0.join("ordered");
	let result = run(&out, false, &root.join("registry/vk.xml"));
	assert!(
		result.status.success(),
		"{}",
		String::from_utf8_lossy(&result.stderr)
	);
	assert_chain_relationships(
		&root,
		&source_text(&out, "src/rs/type.gen.rs"),
		&source_text(&out, "src/rs/construction.gen.rs"),
	);
	assert_construction_inventory(
		&root,
		&source_text(&out, "src/rs/type.gen.rs"),
		&source_text(&out, "tool/gen/construction.gen.tsv"),
	);
	let vk = registry_order(&root.join("registry/vk.xml"));
	let video = registry_order(&root.join("registry/video.xml"));
	let source = |name: &str| std::fs::read_to_string(out.join(name)).unwrap();
	let types = |text: &str| {
		text
			.lines()
			.filter_map(|line| {
				["pub struct ", "pub union ", "pub type "]
					.iter()
					.find_map(|prefix| line.strip_prefix(prefix))
			})
			.map(|line| {
				line
					.split([' ', '(', '{', '=', '<'])
					.next()
					.unwrap()
					.to_owned()
			})
			.collect::<Vec<_>>()
	};
	assert_registry_order(
		types(&source("src/rs/type.gen.rs")),
		vk.types
			.into_iter()
			.chain(video.types)
			.map(|name| rust_type_name(&name)),
		"types",
	);
	assert_registry_order(
		types(&source("src/rs/enum.gen.rs")),
		vk.enums
			.into_iter()
			.chain(video.enums)
			.map(|name| rust_type_name(&name)),
		"enums",
	);
	let constants = source("src/rs/constant.gen.rs")
		.lines()
		.filter_map(|line| line.strip_prefix("pub const "))
		.map(|line| line.split(':').next().unwrap().to_owned())
		.collect();
	assert_registry_order(
		constants,
		vk.constants
			.into_iter()
			.chain(video.constants)
			.map(|name| name.strip_prefix("VK_").unwrap_or(&name).to_owned()),
		"constants",
	);
	let pfn = source("src/rs/pfn.gen.rs")
		.lines()
		.filter_map(|line| line.strip_prefix("pub type PFN_"))
		.map(|line| line.split(' ').next().unwrap().to_owned())
		.filter(|name| vk.commands.contains(name))
		.collect();
	assert_registry_order(pfn, vk.commands.clone(), "command PFNs");
	let commands: Vec<_> = source("src/rs/command.gen.rs")
		.lines()
		.filter_map(|line| line.trim().strip_prefix("/// Call `"))
		.map(|line| line.split('`').next().unwrap().to_owned())
		.collect();
	let mut commands = commands;
	commands.dedup();
	assert_registry_order(commands.clone(), vk.commands.clone(), "native operations");
	let mut cases: Vec<_> = source("test/rs/test_command.gen.rs")
		.split("CallError::MissingCommand(")
		.skip(1)
		.map(|piece| {
			piece
				.trim_start()
				.strip_prefix('"')
				.unwrap()
				.split('"')
				.next()
				.unwrap()
				.to_owned()
		})
		.collect();
	cases.dedup();
	assert_eq!(
		cases, commands,
		"every generated operation must have its API contract case"
	);
	assert_registry_order(cases, vk.commands.clone(), "generated API tests");
	let tables = source("src/rs/dispatch.gen.rs");
	for table in ["FnEntry", "FnInstance", "FnDevice"] {
		let fields = tables
			.split(&format!("pub struct {table} {{"))
			.nth(1)
			.unwrap()
			.split('}')
			.next()
			.unwrap();
		let commands = fields
			.lines()
			.filter_map(|line| line.split(": PFN_").nth(1))
			.map(|line| line.trim_end_matches(',').to_owned())
			.collect();
		assert_registry_order(commands, vk.commands.clone(), table);
	}
	let metadata = source("src/rs/extension.gen.rs");
	let commands = metadata
		.split("CommandMetadata {")
		.filter_map(|record| {
			record
				.split("name: \"")
				.nth(1)
				.map(|value| value.split('"').next().unwrap().to_owned())
		})
		.collect();
	assert_registry_order(commands, vk.commands, "command metadata");
	let extensions = metadata
		.split("name: c\"")
		.skip(1)
		.map(|value| value.split('"').next().unwrap().to_owned())
		.collect();
	assert_registry_order(
		extensions,
		vk.extensions.into_iter().chain(video.extensions),
		"extension metadata",
	);
}

fn source_text(root: &Path, name: &str) -> String {
	std::fs::read_to_string(root.join(name)).unwrap()
}

// Read XML directly, without vk-gen's IR or lifetime/relationship resolver.
fn assert_chain_relationships(root: &Path, records: &str, chains: &str) {
	use quick_xml::{Reader, events::Event};
	use std::collections::BTreeSet;
	let admitted: BTreeSet<_> = records
		.lines()
		.filter_map(|line| line.strip_prefix("pub struct "))
		.map(|line| line.split(['<', ' ', '{']).next().unwrap().to_owned())
		.collect();
	let xml = std::fs::read_to_string(root.join("registry/vk.xml")).unwrap();
	let mut reader = Reader::from_str(&xml);
	let mut expected = Vec::new();
	loop {
		let node = match reader.read_event().unwrap() {
			Event::Start(node) | Event::Empty(node) => node,
			Event::Eof => break,
			_ => continue,
		};
		if node.name().as_ref() != b"type" {
			continue;
		}
		let attributes: std::collections::BTreeMap<_, _> = node
			.attributes()
			.map(|a| {
				let a = a.unwrap();
				(
					String::from_utf8(a.key.as_ref().to_vec()).unwrap(),
					a.decode_and_unescape_value(reader.decoder())
						.unwrap()
						.into_owned(),
				)
			})
			.collect();
		if let (Some(name), Some(extends)) = (attributes.get("name"), attributes.get("structextends")) {
			let name = rust_type_name(name);
			if admitted.contains(&name) {
				for base in extends.split(',') {
					expected.push((name.clone(), rust_type_name(base)));
				}
			}
		}
	}
	let compact: String = chains.chars().filter(|c| !c.is_whitespace()).collect();
	let actual: Vec<_> = compact
		.split("crate::Extends<vk::")
		.skip(1)
		.map(|entry| {
			let base = entry.split('<').next().unwrap().to_owned();
			let name = entry
				.split("forvk::")
				.nth(1)
				.unwrap()
				.split('<')
				.next()
				.unwrap()
				.to_owned();
			(name, base)
		})
		.collect();
	assert!(
		expected.len() > 500,
		"incomplete independent XML chain inventory"
	);
	assert_eq!(
		actual, expected,
		"chain relationships must match pinned XML and its declaration order"
	);
}

/// Read native fields independently from the generator IR. The emitted Rust
/// declarations select admitted records; every member and length expression of
/// those records must occur once, in XML order, in the construction audit.
fn assert_construction_inventory(root: &Path, types: &str, inventory: &str) {
	assert!(inventory.lines().all(|line| line.trim_end() == line));
	use quick_xml::{Reader, events::Event};
	use std::collections::BTreeSet;
	let admitted: BTreeSet<_> = types
		.lines()
		.filter_map(|line| {
			line
				.strip_prefix("pub struct ")
				.or_else(|| line.strip_prefix("pub union "))
		})
		.map(|line| line.split(['<', ' ', '{']).next().unwrap().to_owned())
		.collect();
	let mut expected = Vec::new();
	let mut seen = BTreeSet::new();
	for file in ["vk.xml", "video.xml"] {
		let xml = std::fs::read_to_string(root.join("registry").join(file)).unwrap();
		let mut reader = Reader::from_str(&xml);
		let mut record = None;
		let mut member = None;
		let mut depth = 0;
		let mut record_depth = 0;
		loop {
			match reader.read_event().unwrap() {
				Event::Start(node) => {
					depth += 1;
					let attributes: std::collections::BTreeMap<_, _> = node
						.attributes()
						.map(|a| {
							let a = a.unwrap();
							(
								a.key.as_ref().to_vec(),
								a.decode_and_unescape_value(reader.decoder())
									.unwrap()
									.into_owned(),
							)
						})
						.collect();
					let attr = |key: &[u8]| attributes.get(key).map(String::as_str);
					let vulkan = attr(b"api").is_none_or(|api| api.split(',').any(|api| api == "vulkan"));
					match node.name().as_ref() {
						b"type" if matches!(attr(b"category"), Some("struct" | "union")) && vulkan => {
							let name = attr(b"name").unwrap();
							if admitted.contains(&rust_type_name(name)) && seen.insert(name.to_owned()) {
								record = Some(name.to_owned());
								record_depth = depth;
							}
						}
						b"member" if record.is_some() && depth == record_depth + 1 && vulkan => {
							member = Some((
								attr(b"len").unwrap_or("-").to_owned(),
								attr(b"altlen").unwrap_or("-").to_owned(),
							));
						}
						b"name" if member.is_some() && depth == record_depth + 2 => {
							let field = reader
								.read_text(quick_xml::name::QName(b"name"))
								.unwrap()
								.into_owned();
							let (len, altlen) = member.as_ref().unwrap();
							expected.push((
								record.as_ref().unwrap().clone(),
								field,
								len.clone(),
								altlen.clone(),
							));
							depth -= 1;
						}
						_ => {}
					}
				}
				Event::End(node) => {
					if node.name().as_ref() == b"member" {
						member = None;
					}
					if depth == record_depth {
						record = None;
					}
					depth -= 1;
				}
				Event::Eof => break,
				_ => {}
			}
		}
	}
	// Paired video definitions replace vk.xml header placeholders in place.
	// Retain those positions while taking their members from video.xml.
	let mut by_record = std::collections::BTreeMap::<String, Vec<_>>::new();
	for row in expected {
		by_record.entry(row.0.clone()).or_default().push(row);
	}
	let expected: Vec<_> = registry_order(&root.join("registry/vk.xml"))
		.types
		.into_iter()
		.chain(registry_order(&root.join("registry/video.xml")).types)
		.flat_map(|record| by_record.remove(&record).unwrap_or_default())
		.collect();
	assert!(by_record.is_empty());
	let actual: Vec<_> = inventory
		.lines()
		.filter(|line| !line.starts_with('#'))
		.map(|line| {
			let fields: Vec<_> = line.split('\t').collect();
			assert_eq!(fields.len(), 5);
			assert!(!fields[2].is_empty(), "unclassified field: {line}");
			assert!(
				!fields[2].starts_with("unsafe-") || fields[3] == "-",
				"unreviewed sized raw field: {line}"
			);
			(
				fields[0].to_owned(),
				fields[1].to_owned(),
				fields[3].to_owned(),
				fields[4].to_owned(),
			)
		})
		.collect();
	assert_eq!(
		actual, expected,
		"every admitted record field and extent must be audited in pinned XML order"
	);
}
