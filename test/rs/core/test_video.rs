//! VP9 codec ABI and capability-query contracts.
//!
//! All size/align/offset assertions are derived from the pinned Khronos headers
//! (see `registry/ABI_SHA256SUMS` for `vk_video/vulkan_video_codec_vp9std*.h`
//! and `vulkan/vulkan_core.h`).  No GPU or driver is required.
//!
//! The `VULKAN_AVAILABLE=1`-gated tests at the bottom probe a real driver for
//! VP9 capability; they skip cleanly when no driver is present.
use ::vk::core as vk;
use ::vk::core::*;
use std::mem::{align_of, offset_of, size_of};

#[test]
fn codec_abi_has_one_owner_at_core_and_root() {
	let _: Option<&::vk::core::StdVideoDecodeVP9PictureInfo> =
		None::<&::vk::StdVideoDecodeVP9PictureInfo>;
	let _: ::vk::core::StdVideoVP9Profile = ::vk::StdVideoVP9Profile::_0;
	assert_eq!(
		::vk::core::STD_VIDEO_VP9_MAX_REF_FRAMES,
		::vk::STD_VIDEO_VP9_MAX_REF_FRAMES
	);
}

// ---------------------------------------------------------------------------
// StdVideo* struct layout (video.xml / vulkan_video_codec_vp9std*.h)
// ---------------------------------------------------------------------------

#[test]
fn std_video_vp9_constants_match_spec() {
	// §4.1 of vulkan_video_codec_vp9std.h
	assert_eq!(STD_VIDEO_VP9_NUM_REF_FRAMES, 8);
	assert_eq!(STD_VIDEO_VP9_REFS_PER_FRAME, 3);
	assert_eq!(STD_VIDEO_VP9_MAX_REF_FRAMES, 4);
	assert_eq!(STD_VIDEO_VP9_LOOP_FILTER_ADJUSTMENTS, 2);
	assert_eq!(STD_VIDEO_VP9_MAX_SEGMENTS, 8);
	assert_eq!(STD_VIDEO_VP9_SEG_LVL_MAX, 4);
	assert_eq!(STD_VIDEO_VP9_MAX_SEGMENTATION_TREE_PROBS, 7);
	assert_eq!(STD_VIDEO_VP9_MAX_SEGMENTATION_PRED_PROB, 3);
	// From vulkan_core.h / vk.xml
	assert_eq!(vk::MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR, 3);
}

#[test]
fn std_video_decode_vp9_picture_info_flags_layout() {
	// Packed bitfield; must be exactly one u32.
	assert_eq!(size_of::<StdVideoDecodeVP9PictureInfoFlags>(), 4);
	assert_eq!(align_of::<StdVideoDecodeVP9PictureInfoFlags>(), 4);
}

#[test]
fn std_video_decode_vp9_picture_info_layout() {
	// On 64-bit: flags(4) + profile(4) + frame_type(4) + u8×6 + filter(4) +
	//            q/delta×4 + tile×2 + pad(6) + [u16;3]=6 ... + 3 pointers(24) = 56
	if cfg!(target_pointer_width = "64") {
		assert_eq!(size_of::<StdVideoDecodeVP9PictureInfo>(), 56);
		assert_eq!(align_of::<StdVideoDecodeVP9PictureInfo>(), 8);
		assert_eq!(offset_of!(StdVideoDecodeVP9PictureInfo, flags), 0);
		assert_eq!(offset_of!(StdVideoDecodeVP9PictureInfo, profile), 4);
		assert_eq!(offset_of!(StdVideoDecodeVP9PictureInfo, p_color_config), 32);
	}
}

#[test]
fn std_video_vp9_color_config_layout() {
	// flags(4) + bit_depth(1) + subsampling_x(1) + subsampling_y(1) +
	// reserved1(1) + color_primaries(4) + transfer_characteristics(4) +
	// matrix_coefficients(4) + color_space(4) = 20... actual=12
	// (enum fields are 4 bytes each; flags=4; total with packing)
	assert_eq!(size_of::<StdVideoVP9ColorConfig>(), 12);
	assert_eq!(align_of::<StdVideoVP9ColorConfig>(), 4);
	assert_eq!(offset_of!(StdVideoVP9ColorConfig, flags), 0);
}

#[test]
fn std_video_vp9_loop_filter_layout() {
	assert_eq!(size_of::<StdVideoVP9LoopFilter>(), 16);
	assert_eq!(align_of::<StdVideoVP9LoopFilter>(), 4);
	assert_eq!(offset_of!(StdVideoVP9LoopFilter, flags), 0);
}

#[test]
fn std_video_vp9_segmentation_layout() {
	// flags(4) + 7+3+8+8*4 bytes of arrays + padding = 88
	assert_eq!(size_of::<StdVideoVP9Segmentation>(), 88);
	assert_eq!(align_of::<StdVideoVP9Segmentation>(), 4);
	assert_eq!(offset_of!(StdVideoVP9Segmentation, flags), 0);
}

// ---------------------------------------------------------------------------
// Vulkan VP9 structs (vk.xml / vulkan_core.h)
// ---------------------------------------------------------------------------

#[test]
fn vk_video_decode_vp9_profile_info_layout() {
	// s_type(4) + pad(4) + p_next(8) + std_profile(4) + pad(4) = 24
	if cfg!(target_pointer_width = "64") {
		assert_eq!(size_of::<vk::VideoDecodeVP9ProfileInfoKHR>(), 24);
		assert_eq!(align_of::<vk::VideoDecodeVP9ProfileInfoKHR>(), 8);
		assert_eq!(offset_of!(vk::VideoDecodeVP9ProfileInfoKHR, s_type), 0);
		assert_eq!(offset_of!(vk::VideoDecodeVP9ProfileInfoKHR, p_next), 8);
		assert_eq!(
			offset_of!(vk::VideoDecodeVP9ProfileInfoKHR, std_profile),
			16
		);
	}
}

#[test]
fn vk_video_decode_vp9_capabilities_layout() {
	// s_type(4) + pad(4) + p_next(8) + max_level(4) + pad(4) = 24
	if cfg!(target_pointer_width = "64") {
		assert_eq!(size_of::<vk::VideoDecodeVP9CapabilitiesKHR>(), 24);
		assert_eq!(align_of::<vk::VideoDecodeVP9CapabilitiesKHR>(), 8);
		assert_eq!(offset_of!(vk::VideoDecodeVP9CapabilitiesKHR, s_type), 0);
		assert_eq!(offset_of!(vk::VideoDecodeVP9CapabilitiesKHR, p_next), 8);
		assert_eq!(offset_of!(vk::VideoDecodeVP9CapabilitiesKHR, max_level), 16);
	}
}

#[test]
fn vk_video_decode_vp9_picture_info_layout() {
	// s_type(4)+pad(4)+p_next(8)+p_std_picture_info(8)+reference_name_slot_indices([i32;3]=12)
	// +uncompressed_header_offset(4)+compressed_header_offset(4)+tiles_offset(4)+pad(4) = 56... actual=48
	if cfg!(target_pointer_width = "64") {
		assert_eq!(size_of::<vk::VideoDecodeVP9PictureInfoKHR>(), 48);
		assert_eq!(align_of::<vk::VideoDecodeVP9PictureInfoKHR>(), 8);
		assert_eq!(offset_of!(vk::VideoDecodeVP9PictureInfoKHR, s_type), 0);
		assert_eq!(offset_of!(vk::VideoDecodeVP9PictureInfoKHR, p_next), 8);
		assert_eq!(
			offset_of!(vk::VideoDecodeVP9PictureInfoKHR, p_std_picture_info),
			16
		);
		assert_eq!(
			offset_of!(
				vk::VideoDecodeVP9PictureInfoKHR,
				reference_name_slot_indices
			),
			24
		);
	}
}

#[test]
fn vk_physical_device_video_decode_vp9_features_layout() {
	// s_type(4)+pad(4)+p_next(8)+video_decode_vp9(4)+pad(4) = 24
	if cfg!(target_pointer_width = "64") {
		assert_eq!(size_of::<vk::PhysicalDeviceVideoDecodeVP9FeaturesKHR>(), 24);
		assert_eq!(align_of::<vk::PhysicalDeviceVideoDecodeVP9FeaturesKHR>(), 8);
		assert_eq!(
			offset_of!(vk::PhysicalDeviceVideoDecodeVP9FeaturesKHR, s_type),
			0
		);
		assert_eq!(
			offset_of!(
				vk::PhysicalDeviceVideoDecodeVP9FeaturesKHR,
				video_decode_vp9
			),
			16
		);
	}
}

// ---------------------------------------------------------------------------
// VK_KHR_video_decode_vp9 discoverability — no driver required
// ---------------------------------------------------------------------------

#[test]
fn vk_khr_video_decode_vp9_extension_name_is_correct() {
	assert_eq!(
		vk::KHR_VIDEO_DECODE_VP9_EXTENSION_NAME,
		c"VK_KHR_video_decode_vp9"
	);
	assert_eq!(vk::KHR_VIDEO_DECODE_VP9_SPEC_VERSION, 1);
}

#[test]
fn vk_video_decode_vp9_defaults_have_correct_stype() {
	let profile = vk::VideoDecodeVP9ProfileInfoKHR::default();
	assert_eq!(
		profile.s_type,
		vk::StructureType::VIDEO_DECODE_VP9_PROFILE_INFO_KHR
	);
	assert!(profile.p_next.is_null());

	let caps = vk::VideoDecodeVP9CapabilitiesKHR::default();
	assert_eq!(
		caps.s_type,
		vk::StructureType::VIDEO_DECODE_VP9_CAPABILITIES_KHR
	);

	let pic = vk::VideoDecodeVP9PictureInfoKHR::default();
	assert_eq!(
		pic.s_type,
		vk::StructureType::VIDEO_DECODE_VP9_PICTURE_INFO_KHR
	);
	assert_eq!(
		pic.reference_name_slot_indices.len(),
		vk::MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR as usize
	);

	let feats = vk::PhysicalDeviceVideoDecodeVP9FeaturesKHR::default();
	assert_eq!(
		feats.s_type,
		vk::StructureType::PHYSICAL_DEVICE_VIDEO_DECODE_VP9_FEATURES_KHR
	);
}

#[test]
fn std_video_vp9_profile_enum_has_expected_variants() {
	// Values from vulkan_video_codec_vp9std.h §StdVideoVP9Profile
	// Generator emits numeric-leading-digit constants with underscore prefix.
	assert_eq!(StdVideoVP9Profile::_0.as_raw(), 0);
	assert_eq!(StdVideoVP9Profile::_1.as_raw(), 1);
	assert_eq!(StdVideoVP9Profile::_2.as_raw(), 2);
	assert_eq!(StdVideoVP9Profile::_3.as_raw(), 3);
}

#[test]
fn std_video_vp9_level_enum_has_expected_variants() {
	// Values from vulkan_video_codec_vp9std.h §StdVideoVP9Level
	assert_eq!(StdVideoVP9Level::_1_0.as_raw(), 0);
	assert_eq!(StdVideoVP9Level::_2_0.as_raw(), 2);
	assert_eq!(StdVideoVP9Level::_5_0.as_raw(), 8);
}

#[test]
fn std_video_decode_vp9_picture_info_flags_bitfield_accessors() {
	let mut flags = StdVideoDecodeVP9PictureInfoFlags { _bitfield_1: 0 };
	// Each bit accessor should read/write its field without clobbering others.
	flags.set_error_resilient_mode(1);
	assert_eq!(flags.error_resilient_mode(), 1);
	assert_eq!(flags.intra_only(), 0);
	flags.set_intra_only(1);
	assert_eq!(flags.error_resilient_mode(), 1);
	assert_eq!(flags.intra_only(), 1);
	// Clearing one must not touch the other.
	flags.set_error_resilient_mode(0);
	assert_eq!(flags.error_resilient_mode(), 0);
	assert_eq!(flags.intra_only(), 1);
}

// ---------------------------------------------------------------------------
// Runtime capability probe — skipped unless VULKAN_AVAILABLE=1
// ---------------------------------------------------------------------------

#[cfg(feature = "loader")]
#[test]
fn vp9_capability_probe() {
	if std::env::var("VULKAN_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	use ::vk::load::Entry;

	// SAFETY: platform Vulkan loader; trust is explicit via env var.
	let entry = unsafe { Entry::new() }.expect("load Vulkan");

	let version = entry.enumerate_instance_version().expect("loader version");
	if version < vk::API_VERSION_1_1 {
		eprintln!("VP9 query unqualified: loader predates Vulkan 1.1");
		return;
	}
	let app = vk::ApplicationInfo {
		api_version: vk::API_VERSION_1_1,
		..Default::default()
	};
	// SAFETY: live input values, compatible pNext chains and handles from this instance.
	unsafe {
		let instance = entry
			.create_instance(
				&vk::InstanceCreateInfo {
					p_application_info: &app,
					..Default::default()
				},
				None,
			)
			.unwrap();
		let mut queried = 0;
		for physical in instance.enumerate_physical_devices().unwrap() {
			let extensions = instance
				.enumerate_device_extension_properties(physical, None)
				.unwrap();
			let advertised = |name: &std::ffi::CStr| {
				extensions
					.iter()
					.any(|e| std::ffi::CStr::from_ptr(e.extension_name.as_ptr()) == name)
			};
			if !advertised(vk::KHR_VIDEO_DECODE_VP9_EXTENSION_NAME) {
				continue;
			}
			assert!(advertised(vk::KHR_VIDEO_QUEUE_EXTENSION_NAME));
			assert!(advertised(vk::KHR_VIDEO_DECODE_QUEUE_EXTENSION_NAME));
			let mut vp9_feature = vk::PhysicalDeviceVideoDecodeVP9FeaturesKHR::default();
			let mut features = vk::PhysicalDeviceFeatures2 {
				p_next: std::ptr::from_mut(&mut vp9_feature).cast(),
				..Default::default()
			};
			instance
				.fp()
				.get_physical_device_features2
				.expect("Vulkan 1.1 feature query")(physical, &mut features);
			if vp9_feature.video_decode_vp9 == vk::FALSE {
				continue;
			}
			// Probe a concrete baseline profile; this is not an exhaustive profile inventory.
			let vp9_profile = vk::VideoDecodeVP9ProfileInfoKHR {
				std_profile: StdVideoVP9Profile::_0,
				..Default::default()
			};
			let profile = vk::VideoProfileInfoKHR {
				p_next: std::ptr::from_ref(&vp9_profile).cast(),
				video_codec_operation: vk::VideoCodecOperationFlagBitsKHR::DECODE_VP9_BIT_KHR,
				chroma_subsampling: vk::VideoChromaSubsamplingFlagBitsKHR::_420_BIT_KHR,
				luma_bit_depth: vk::VideoComponentBitDepthFlagBitsKHR::_8_BIT_KHR,
				chroma_bit_depth: vk::VideoComponentBitDepthFlagBitsKHR::_8_BIT_KHR,
				..Default::default()
			};
			// Both output structures are mandatory for VP9 decode (VUID 07183/10792).
			let mut vp9_caps = vk::VideoDecodeVP9CapabilitiesKHR::default();
			let mut decode_caps = vk::VideoDecodeCapabilitiesKHR {
				p_next: std::ptr::from_mut(&mut vp9_caps).cast(),
				..Default::default()
			};
			let mut caps = vk::VideoCapabilitiesKHR {
				p_next: std::ptr::from_mut(&mut decode_caps).cast(),
				..Default::default()
			};
			let result = instance
				.fp()
				.get_physical_device_video_capabilities_khr
				.expect("VP9 capabilities query")(physical, &profile, &mut caps);
			queried += 1;
			match result {
				vk::Result::SUCCESS => {
					assert!(caps.max_coded_extent.width >= caps.min_coded_extent.width);
					assert!(caps.max_coded_extent.height >= caps.min_coded_extent.height);
					assert!(caps.min_bitstream_buffer_offset_alignment.is_power_of_two());
					eprintln!("VP9 profile 0 capability query succeeded");
				}
				vk::Result::ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR
				| vk::Result::ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR
				| vk::Result::ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR
				| vk::Result::ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR => {
					eprintln!("VP9 profile 0 unsupported: {result:?}")
				}
				other => panic!("VP9 query failed: {other:?}"),
			}
		}
		instance.destroy(None);
		if queried == 0 {
			eprintln!(
				"VP9 hardware query unqualified: no device exposes the required extension and feature"
			);
		}
	}
}
