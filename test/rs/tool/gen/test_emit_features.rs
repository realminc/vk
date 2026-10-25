use super::*;

fn registry() -> Registry {
	let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../registry/vk.xml");
	let mut registry = crate::parse::parse_registry(&path, false).unwrap();
	crate::resolve::resolve(&mut registry).unwrap();
	registry
}

#[test]
fn unsupported_feature_member_or_missing_structure_tag_fails_generation() {
	let mut reg = registry();
	let index = reg
		.types
		.iter()
		.position(|ty| ty.c_name == "VkPhysicalDeviceTimelineSemaphoreFeatures")
		.unwrap();
	let TypeKind::Struct { members, .. } = &mut reg.types[index].kind else {
		panic!("feature struct")
	};
	members
		.iter_mut()
		.find(|member| member.name == "timelineSemaphore")
		.unwrap()
		.ty
		.name = "uint64_t".into();
	assert!(emit_features(&reg).is_err());
	let TypeKind::Struct { members, .. } = &mut reg.types[index].kind else {
		unreachable!()
	};
	members
		.iter_mut()
		.find(|member| member.name == "timelineSemaphore")
		.unwrap()
		.ty
		.name = "VkBool32".into();
	members
		.iter_mut()
		.find(|member| member.name == "sType")
		.unwrap()
		.values = None;
	assert!(emit_features(&reg).is_err());
}

#[test]
fn missing_base_or_unsupported_base_layout_fails_generation() {
	assert!(emit_features(&Registry::default()).is_err());
	let mut reg = registry();
	let base = reg
		.types
		.iter_mut()
		.find(|ty| ty.c_name == "VkPhysicalDeviceFeatures")
		.unwrap();
	let TypeKind::Struct { members, .. } = &mut base.kind else {
		panic!("base struct")
	};
	members[0].ty.arrays.push("2".into());
	assert!(emit_features(&reg).is_err());
}

#[test]
fn incompatible_chain_header_is_rejected_before_emission() {
	let mut reg = registry();
	let ty = reg
		.types
		.iter_mut()
		.find(|ty| ty.c_name == "VkPhysicalDeviceTimelineSemaphoreFeatures")
		.unwrap();
	let TypeKind::Struct { members, .. } = &mut ty.kind else {
		panic!("feature struct")
	};
	members.swap(0, 1);
	assert!(emit_features(&reg).is_err());
}
