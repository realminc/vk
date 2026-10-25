use super::*;
#[test]
fn arrays_decay_only_in_parameters() {
	let t = TypeRef {
		name: "float".into(),
		pointers: vec![],
		arrays: vec!["4".into()],
		array_element_const: true,
	};
	assert_eq!(type_str(&t), "[f32; 4]");
	assert_eq!(parameter_type(&t), "*const f32");
}

#[test]
fn nested_pointer_qualifiers() {
	let t = TypeRef {
		name: "char".into(),
		pointers: vec![true, false],
		arrays: vec![],
		array_element_const: false,
	};
	assert_eq!(type_str(&t), "*mut *const std::ffi::c_char");
}

#[test]
fn qualified_prototypes_preserve_pointer_constness_array_decay_and_native_types() {
	let mut ty = TypeRef {
		name: "VkBuffer".into(),
		pointers: vec![true, false],
		arrays: vec![],
		array_element_const: false,
	};
	assert_eq!(qualified_parameter_type(&ty), "*mut *const vk::Buffer");
	ty.pointers.clear();
	ty.arrays = vec!["4".into(), "VK_UUID_SIZE".into()];
	ty.array_element_const = true;
	assert_eq!(
		qualified_parameter_type(&ty),
		"*const [vk::Buffer; vk::UUID_SIZE as usize]"
	);
	ty.name = "xcb_window_t".into();
	ty.arrays.clear();
	assert_eq!(qualified_type(&ty), "vk::xcb_window_t");
}
