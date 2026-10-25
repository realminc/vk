use super::*;
#[test]
fn mixed_declarators_and_api_members() {
	let reg=parse(r#"<registry><types><type category="struct" name="VkFixture"><member><type>float</type><name>matrix</name>[3][4]<comment>not [an extent]</comment></member><member>const <type>char</type>* const*<name>names</name></member><member api="vulkansc"><type>uint32_t</type><name>scOnly</name></member></type><type category="basetype">typedef <type>uint64_t</type><name>VkFlags64</name>;</type></types></registry>"#,false).unwrap();
	let TypeKind::Struct { members, .. } = &reg.types[0].kind else {
		panic!()
	};
	assert_eq!(members.len(), 2);
	assert_eq!(members[0].ty.arrays, ["3", "4"]);
	assert_eq!(members[1].ty.pointers, [true, true]);
	let TypeKind::BaseType { ty } = &reg.types[1].kind else {
		panic!()
	};
	assert_eq!(ty.name, "uint64_t");
}

#[test]
fn core_additions_and_extension_offsets() {
	let reg=parse(r#"<registry><enums name="VkFixture" type="enum"><enum name="VK_FIXTURE_A" value="0"/></enums><feature api="vulkan" name="VK_VERSION_1_1" number="1.1"><require><enum extends="VkFixture" name="VK_FIXTURE_B" value="1"/></require></feature><extensions><extension name="VK_EXT_fixture" number="2" supported="vulkan" type="device"><require><enum extends="VkFixture" name="VK_FIXTURE_C" offset="3" dir="-"/></require></extension></extensions></registry>"#,false).unwrap();
	assert_eq!(reg.enums[0].values.len(), 3);
	assert_eq!(reg.enums[0].values[2].value, "-1000001003");
}

#[test]
fn malformed_xml_is_rejected() {
	assert!(parse("<registry><types>", false).is_err());
}

#[test]
fn success_statuses_survive_command_alias_resolution() {
	let mut reg = parse(r#"<registry>
<types><type category="handle"><type>VK_DEFINE_HANDLE</type>(<name>VkDevice</name>)</type></types>
<commands>
<command successcodes="VK_SUCCESS,VK_PIPELINE_COMPILE_REQUIRED_EXT"><proto><type>VkResult</type><name>vkFixture</name></proto><param><type>VkDevice</type><name>device</name></param></command>
<command name="vkFixtureEXT" alias="vkFixture"/>
</commands>
<feature api="vulkan" name="VK_VERSION_1_0" number="1.0"><require><command name="vkFixture"/><command name="vkFixtureEXT"/></require></feature>
</registry>"#, false).unwrap();
	crate::resolve::resolve(&mut reg).unwrap();
	assert_eq!(reg.commands.len(), 2);
	for command in &reg.commands {
		assert_eq!(
			command.success_codes,
			["VK_SUCCESS", "VK_PIPELINE_COMPILE_REQUIRED_EXT"]
		);
		assert_eq!(command.dispatch, DispatchLevel::Device);
	}
}

#[test]
fn capability_providers_keep_scope_and_conditional_requirements() {
	let registry = parse(r#"<registry>
<feature api="vulkan" name="VK_VERSION_1_1" number="1.1"><require><command name="vkFixture"/></require></feature>
<extensions><extension name="VK_EXT_fixture" number="1" supported="vulkan" type="device" depends="VK_VERSION_1_0,VK_KHR_fixture">
<require depends="VK_KHR_other"><command name="vkFixtureEXT"/></require>
<require api="vulkansc"><command name="vkScOnly"/></require>
</extension></extensions></registry>"#, false).unwrap();
	assert_eq!(
		registry.command_requirements["vkFixture"],
		["(VK_VERSION_1_1)"]
	);
	assert_eq!(
		registry.command_requirements["vkFixtureEXT"],
		["(VK_EXT_fixture)+(VK_VERSION_1_0,VK_KHR_fixture)+(VK_KHR_other)"]
	);
	assert!(!registry.command_requirements.contains_key("vkScOnly"));
}
