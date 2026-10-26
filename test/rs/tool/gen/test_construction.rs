use super::*;

fn registry(xml: &str) -> Registry {
	let nonce = std::time::SystemTime::now()
		.duration_since(std::time::UNIX_EPOCH)
		.unwrap()
		.as_nanos();
	let path = std::env::temp_dir().join(format!(
		"vk-construction-{}-{nonce}.xml",
		std::process::id()
	));
	std::fs::write(&path, xml).unwrap();
	let result = crate::parse::parse_registry(&path, false);
	std::fs::remove_file(path).unwrap();
	result.unwrap()
}

#[test]
fn borrowed_classification_and_aliases_are_transitive() {
	let mut reg = registry(
		r#"<registry><types>
<type category="struct" name="VkBorrowed"><member len="null-terminated">const <type>char</type>*<name>pName</name></member></type>
<type category="struct" name="VkOuter"><member>const <type>VkBorrowed</type>*<name>pInfo</name></member></type>
<type category="struct" name="VkAlias" alias="VkOuter"/>
</types></registry>"#,
	);
	resolve(&mut reg).unwrap();
	assert_eq!(
		reg.borrowed.into_iter().collect::<Vec<_>>(),
		["VkAlias", "VkBorrowed", "VkOuter"]
	);
	assert!(reg.exclusive.is_empty());
}

#[test]
fn ambiguous_pointer_counts_and_opaque_payloads_stay_raw() {
	let reg = registry(
		r#"<registry><types>
<type category="struct" name="VkFixture">
<member><type>uint32_t</type><name>count</name></member>
<member len="count">const <type>uint32_t</type>*<name>pFirst</name></member>
<member len="count">const <type>uint32_t</type>*<name>pSecond</name></member>
<member>const <type>void</type>*<name>pOpaque</name></member>
<member len="latexmath:[2N]">const <type>uint32_t</type>*<name>pUnknown</name></member>
</type></types></registry>"#,
	);
	let ty = &reg.types[0];
	assert!(members(ty).iter().all(|m| setter(&reg, ty, m).is_none()));
}

#[test]
fn chain_helpers_keep_native_field_names_and_exclusive_propagation() {
	let mut reg = registry(
		r#"<registry><types>
<type category="struct" name="VkRoot">
<member values="VK_STRUCTURE_TYPE_APPLICATION_INFO"><type>VkStructureType</type><name>sType</name></member>
<member>const <type>void</type>*<name>pNext</name></member></type>
<type category="struct" name="VkNode" structextends="VkRoot">
<member values="VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO"><type>VkStructureType</type><name>sType</name></member>
<member>const <type>void</type>*<name>pNext</name></member></type>
<type category="struct" name="VkContainer"><member><type>VkNode</type><name>node</name></member></type>
<type category="struct" name="VkAlias" alias="VkContainer"/>
</types></registry>"#,
	);
	resolve(&mut reg).unwrap();
	assert_eq!(reg.exclusive.len(), 4);
	let source = emit_chains(&reg).unwrap();
	for expected in [
		"fn s_type(",
		"fn p_next(",
		"fn set_p_next(",
		"const S_TYPE:",
		"crate::Extends<vk::Root<'root>>",
	] {
		assert!(
			source.contains(expected),
			"missing native chain contract: {expected}"
		);
	}
	for old in ["fn tag(", "fn next(", "fn set_next("] {
		assert!(!source.contains(old));
	}
	let emitted = crate::emit_types::emit_types(&reg).unwrap();
	assert!(emitted.contains("PhantomData<&'a mut ()>"));
	assert!(!emitted.contains("derive(Copy, Clone)"));
}
