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
	let mut reg = registry(
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
	assert!(setter(&reg, ty, &members(ty)[1]).is_none());
	assert_eq!(setter(&reg, ty, &members(ty)[3]), Some(Setter::Raw));
	assert_eq!(setter(&reg, ty, &members(ty)[4]), Some(Setter::Raw));
	assert!(
		resolve(&mut reg).is_err(),
		"unreviewed shared counts must fail generation"
	);
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

#[test]
fn mutable_storage_propagates_exclusivity_through_by_value_unions_and_aliases() {
	let mut reg = registry(
		r#"<registry><types>
<type category="struct" name="VkOutput"><member><type>size_t</type><name>dataSize</name></member><member len="dataSize"><type>void</type>*<name>pData</name></member></type>
<type category="union" name="VkUnion"><member><type>VkOutput</type><name>output</name></member><member><type>uint64_t</type><name>number</name></member></type>
<type category="struct" name="VkContainer"><member><type>VkUnion</type><name>value</name></member></type>
<type category="struct" name="VkAlias" alias="VkContainer"/>
</types></registry>"#,
	);
	resolve(&mut reg).unwrap();
	assert_eq!(reg.borrowed.len(), 4);
	assert_eq!(reg.exclusive.len(), 4);
	let source = crate::emit_types::emit_types(&reg).unwrap();
	assert!(source.contains("ManuallyDrop<Output<'a>>"));
	assert!(source.contains("ManuallyDrop::new(value)"));
	assert!(source.contains("&'a mut [u8]"));
	assert!(!source.contains("derive(Copy, Clone)"));
}

#[test]
fn formula_metadata_is_preserved_and_changed_native_shapes_are_rejected() {
	let mut reg = registry(
		r#"<registry><types><type category="struct" name="VkShaderModuleCreateInfo">
<member><type>size_t</type><name>codeSize</name></member>
<member len="latexmath:[codeSize / 4]" altlen="codeSize / 4">const <type>uint32_t</type>*<name>pCode</name></member>
</type></types></registry>"#,
	);
	resolve(&mut reg).unwrap();
	let ty = &reg.types[0];
	assert_eq!(members(ty)[1].alt_len.as_deref(), Some("codeSize / 4"));
	assert_eq!(setter(&reg, ty, &members(ty)[1]), Some(Setter::Words));
	let TypeKind::Struct { members, .. } = &mut reg.types[0].kind else {
		unreachable!()
	};
	members[1].ty.name = "uint64_t".into();
	assert!(resolve(&mut reg).is_err());
}

#[test]
fn unknown_math_extents_fail_instead_of_publishing_raw_fallbacks() {
	let mut reg = registry(
		r#"<registry><types><type category="struct" name="VkFixture">
<member len="latexmath:[N+7]">const <type>uint32_t</type>*<name>pData</name></member>
</type></types></registry>"#,
	);
	assert!(resolve(&mut reg).is_err());
}

#[test]
fn native_pointer_optional_qualifier_is_independent_of_pointee_optional_qualifiers() {
	let reg = registry(
		r#"<registry><types><type category="struct" name="VkFixture">
<member optional="true,false">const <type>uint32_t</type>* const*<name>ppData</name></member>
<member optional="false,true">const <type>uint32_t</type>*<name>pOther</name></member>
</type></types></registry>"#,
	);
	assert!(members(&reg.types[0])[0].optional);
	assert!(!members(&reg.types[0])[1].optional);
}
