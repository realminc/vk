// @generated — do not edit by hand.
#![allow(non_camel_case_types, non_upper_case_globals)]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ImageLayout(pub i32);
impl ImageLayout {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const UNDEFINED: Self = Self(0);
	pub const GENERAL: Self = Self(1);
	pub const COLOR_ATTACHMENT_OPTIMAL: Self = Self(2);
	pub const DEPTH_STENCIL_ATTACHMENT_OPTIMAL: Self = Self(3);
	pub const DEPTH_STENCIL_READ_ONLY_OPTIMAL: Self = Self(4);
	pub const SHADER_READ_ONLY_OPTIMAL: Self = Self(5);
	pub const TRANSFER_SRC_OPTIMAL: Self = Self(6);
	pub const TRANSFER_DST_OPTIMAL: Self = Self(7);
	pub const PREINITIALIZED: Self = Self(8);
	pub const DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL: Self = Self(1000117000);
	pub const DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL: Self = Self(1000117001);
	pub const DEPTH_ATTACHMENT_OPTIMAL: Self = Self(1000241000);
	pub const DEPTH_READ_ONLY_OPTIMAL: Self = Self(1000241001);
	pub const STENCIL_ATTACHMENT_OPTIMAL: Self = Self(1000241002);
	pub const STENCIL_READ_ONLY_OPTIMAL: Self = Self(1000241003);
	pub const READ_ONLY_OPTIMAL: Self = Self(1000314000);
	pub const ATTACHMENT_OPTIMAL: Self = Self(1000314001);
	pub const RENDERING_LOCAL_READ: Self = Self(1000232000);
	pub const PRESENT_SRC_KHR: Self = Self(1000001002);
	pub const VIDEO_DECODE_DST_KHR: Self = Self(1000024000);
	pub const VIDEO_DECODE_SRC_KHR: Self = Self(1000024001);
	pub const VIDEO_DECODE_DPB_KHR: Self = Self(1000024002);
	pub const SHARED_PRESENT_KHR: Self = Self(1000111000);
	pub const DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL_KHR: Self = Self(1000117000);
	pub const DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL_KHR: Self = Self(1000117001);
	pub const SHADING_RATE_OPTIMAL_NV: Self = Self(1000164003);
	pub const FRAGMENT_DENSITY_MAP_OPTIMAL_EXT: Self = Self(1000218000);
	pub const FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR: Self = Self(1000164003);
	pub const RENDERING_LOCAL_READ_KHR: Self = Self(1000232000);
	pub const DEPTH_ATTACHMENT_OPTIMAL_KHR: Self = Self(1000241000);
	pub const DEPTH_READ_ONLY_OPTIMAL_KHR: Self = Self(1000241001);
	pub const STENCIL_ATTACHMENT_OPTIMAL_KHR: Self = Self(1000241002);
	pub const STENCIL_READ_ONLY_OPTIMAL_KHR: Self = Self(1000241003);
	pub const VIDEO_ENCODE_DST_KHR: Self = Self(1000299000);
	pub const VIDEO_ENCODE_SRC_KHR: Self = Self(1000299001);
	pub const VIDEO_ENCODE_DPB_KHR: Self = Self(1000299002);
	pub const READ_ONLY_OPTIMAL_KHR: Self = Self(1000314000);
	pub const ATTACHMENT_OPTIMAL_KHR: Self = Self(1000314001);
	pub const ATTACHMENT_FEEDBACK_LOOP_OPTIMAL_EXT: Self = Self(1000339000);
	pub const TENSOR_ALIASING_ARM: Self = Self(1000460000);
	pub const VIDEO_ENCODE_QUANTIZATION_MAP_KHR: Self = Self(1000553000);
	pub const ZERO_INITIALIZED_EXT: Self = Self(1000620000);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AttachmentLoadOp(pub i32);
impl AttachmentLoadOp {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const LOAD: Self = Self(0);
	pub const CLEAR: Self = Self(1);
	pub const DONT_CARE: Self = Self(2);
	pub const NONE: Self = Self(1000400000);
	pub const NONE_EXT: Self = Self(1000400000);
	pub const NONE_KHR: Self = Self(1000400000);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AttachmentStoreOp(pub i32);
impl AttachmentStoreOp {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const STORE: Self = Self(0);
	pub const DONT_CARE: Self = Self(1);
	pub const NONE: Self = Self(1000301000);
	pub const NONE_KHR: Self = Self(1000301000);
	pub const NONE_QCOM: Self = Self(1000301000);
	pub const NONE_EXT: Self = Self(1000301000);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ImageType(pub i32);
impl ImageType {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const _1D: Self = Self(0);
	pub const _2D: Self = Self(1);
	pub const _3D: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ImageTiling(pub i32);
impl ImageTiling {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const OPTIMAL: Self = Self(0);
	pub const LINEAR: Self = Self(1);
	pub const DRM_FORMAT_MODIFIER_EXT: Self = Self(1000158000);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ImageViewType(pub i32);
impl ImageViewType {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const _1D: Self = Self(0);
	pub const _2D: Self = Self(1);
	pub const _3D: Self = Self(2);
	pub const CUBE: Self = Self(3);
	pub const _1D_ARRAY: Self = Self(4);
	pub const _2D_ARRAY: Self = Self(5);
	pub const CUBE_ARRAY: Self = Self(6);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CommandBufferLevel(pub i32);
impl CommandBufferLevel {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const PRIMARY: Self = Self(0);
	pub const SECONDARY: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ComponentSwizzle(pub i32);
impl ComponentSwizzle {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const IDENTITY: Self = Self(0);
	pub const ZERO: Self = Self(1);
	pub const ONE: Self = Self(2);
	pub const R: Self = Self(3);
	pub const G: Self = Self(4);
	pub const B: Self = Self(5);
	pub const A: Self = Self(6);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DescriptorType(pub i32);
impl DescriptorType {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const SAMPLER: Self = Self(0);
	pub const COMBINED_IMAGE_SAMPLER: Self = Self(1);
	pub const SAMPLED_IMAGE: Self = Self(2);
	pub const STORAGE_IMAGE: Self = Self(3);
	pub const UNIFORM_TEXEL_BUFFER: Self = Self(4);
	pub const STORAGE_TEXEL_BUFFER: Self = Self(5);
	pub const UNIFORM_BUFFER: Self = Self(6);
	pub const STORAGE_BUFFER: Self = Self(7);
	pub const UNIFORM_BUFFER_DYNAMIC: Self = Self(8);
	pub const STORAGE_BUFFER_DYNAMIC: Self = Self(9);
	pub const INPUT_ATTACHMENT: Self = Self(10);
	pub const INLINE_UNIFORM_BLOCK: Self = Self(1000138000);
	pub const INLINE_UNIFORM_BLOCK_EXT: Self = Self(1000138000);
	pub const ACCELERATION_STRUCTURE_KHR: Self = Self(1000150000);
	pub const ACCELERATION_STRUCTURE_NV: Self = Self(1000165000);
	pub const MUTABLE_VALVE: Self = Self(1000351000);
	pub const SAMPLE_WEIGHT_IMAGE_QCOM: Self = Self(1000440000);
	pub const BLOCK_MATCH_IMAGE_QCOM: Self = Self(1000440001);
	pub const TENSOR_ARM: Self = Self(1000460000);
	pub const MUTABLE_EXT: Self = Self(1000351000);
	pub const PARTITIONED_ACCELERATION_STRUCTURE_NV: Self = Self(1000570000);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct QueryType(pub i32);
impl QueryType {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const OCCLUSION: Self = Self(0);
	pub const PIPELINE_STATISTICS: Self = Self(1);
	pub const TIMESTAMP: Self = Self(2);
	pub const RESULT_STATUS_ONLY_KHR: Self = Self(1000023000);
	pub const TRANSFORM_FEEDBACK_STREAM_EXT: Self = Self(1000028004);
	pub const PERFORMANCE_QUERY_KHR: Self = Self(1000116000);
	pub const ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR: Self = Self(1000150000);
	pub const ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR: Self = Self(1000150001);
	pub const ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV: Self = Self(1000165000);
	pub const TIME_ELAPSED_QCOM: Self = Self(1000173000);
	pub const PERFORMANCE_QUERY_INTEL: Self = Self(1000210000);
	pub const VIDEO_ENCODE_FEEDBACK_KHR: Self = Self(1000299000);
	pub const MESH_PRIMITIVES_GENERATED_EXT: Self = Self(1000328000);
	pub const PRIMITIVES_GENERATED_EXT: Self = Self(1000382000);
	pub const ACCELERATION_STRUCTURE_SERIALIZATION_BOTTOM_LEVEL_POINTERS_KHR: Self = Self(1000386000);
	pub const ACCELERATION_STRUCTURE_SIZE_KHR: Self = Self(1000386001);
	pub const MICROMAP_SERIALIZATION_SIZE_EXT: Self = Self(1000396000);
	pub const MICROMAP_COMPACTED_SIZE_EXT: Self = Self(1000396001);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BorderColor(pub i32);
impl BorderColor {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const FLOAT_TRANSPARENT_BLACK: Self = Self(0);
	pub const INT_TRANSPARENT_BLACK: Self = Self(1);
	pub const FLOAT_OPAQUE_BLACK: Self = Self(2);
	pub const INT_OPAQUE_BLACK: Self = Self(3);
	pub const FLOAT_OPAQUE_WHITE: Self = Self(4);
	pub const INT_OPAQUE_WHITE: Self = Self(5);
	pub const FLOAT_CUSTOM_EXT: Self = Self(1000287003);
	pub const INT_CUSTOM_EXT: Self = Self(1000287004);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PipelineBindPoint(pub i32);
impl PipelineBindPoint {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const GRAPHICS: Self = Self(0);
	pub const COMPUTE: Self = Self(1);
	#[cfg(feature = "provisional")]
	pub const EXECUTION_GRAPH_AMDX: Self = Self(1000134000);
	pub const RAY_TRACING_KHR: Self = Self(1000165000);
	pub const RAY_TRACING_NV: Self = Self(1000165000);
	pub const SUBPASS_SHADING_HUAWEI: Self = Self(1000369003);
	pub const DATA_GRAPH_ARM: Self = Self(1000507000);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PipelineCacheHeaderVersion(pub i32);
impl PipelineCacheHeaderVersion {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ONE: Self = Self(1);
	pub const DATA_GRAPH_QCOM: Self = Self(1000629000);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PipelineCacheCreateFlagBits(pub u32);
impl PipelineCacheCreateFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const EXTERNALLY_SYNCHRONIZED_BIT: Self = Self(1);
	pub const EXTERNALLY_SYNCHRONIZED_BIT_EXT: Self = Self(1);
	pub const INTERNALLY_SYNCHRONIZED_MERGE_BIT_KHR: Self = Self(8);
}

impl std::ops::BitOr for PipelineCacheCreateFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for PipelineCacheCreateFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for PipelineCacheCreateFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for PipelineCacheCreateFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PrimitiveTopology(pub i32);
impl PrimitiveTopology {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const POINT_LIST: Self = Self(0);
	pub const LINE_LIST: Self = Self(1);
	pub const LINE_STRIP: Self = Self(2);
	pub const TRIANGLE_LIST: Self = Self(3);
	pub const TRIANGLE_STRIP: Self = Self(4);
	pub const TRIANGLE_FAN: Self = Self(5);
	pub const LINE_LIST_WITH_ADJACENCY: Self = Self(6);
	pub const LINE_STRIP_WITH_ADJACENCY: Self = Self(7);
	pub const TRIANGLE_LIST_WITH_ADJACENCY: Self = Self(8);
	pub const TRIANGLE_STRIP_WITH_ADJACENCY: Self = Self(9);
	pub const PATCH_LIST: Self = Self(10);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SharingMode(pub i32);
impl SharingMode {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const EXCLUSIVE: Self = Self(0);
	pub const CONCURRENT: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct IndexType(pub i32);
impl IndexType {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const UINT16: Self = Self(0);
	pub const UINT32: Self = Self(1);
	pub const UINT8: Self = Self(1000265000);
	pub const NONE_KHR: Self = Self(1000165000);
	pub const NONE_NV: Self = Self(1000165000);
	pub const UINT8_EXT: Self = Self(1000265000);
	pub const UINT8_KHR: Self = Self(1000265000);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Filter(pub i32);
impl Filter {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const NEAREST: Self = Self(0);
	pub const LINEAR: Self = Self(1);
	pub const CUBIC_IMG: Self = Self(1000015000);
	pub const CUBIC_EXT: Self = Self(1000015000);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SamplerMipmapMode(pub i32);
impl SamplerMipmapMode {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const NEAREST: Self = Self(0);
	pub const LINEAR: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SamplerAddressMode(pub i32);
impl SamplerAddressMode {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const REPEAT: Self = Self(0);
	pub const MIRRORED_REPEAT: Self = Self(1);
	pub const CLAMP_TO_EDGE: Self = Self(2);
	pub const CLAMP_TO_BORDER: Self = Self(3);
	pub const MIRROR_CLAMP_TO_EDGE: Self = Self(4);
	pub const MIRROR_CLAMP_TO_EDGE_KHR: Self = Self(4);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CompareOp(pub i32);
impl CompareOp {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const NEVER: Self = Self(0);
	pub const LESS: Self = Self(1);
	pub const EQUAL: Self = Self(2);
	pub const LESS_OR_EQUAL: Self = Self(3);
	pub const GREATER: Self = Self(4);
	pub const NOT_EQUAL: Self = Self(5);
	pub const GREATER_OR_EQUAL: Self = Self(6);
	pub const ALWAYS: Self = Self(7);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PolygonMode(pub i32);
impl PolygonMode {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const FILL: Self = Self(0);
	pub const LINE: Self = Self(1);
	pub const POINT: Self = Self(2);
	pub const FILL_RECTANGLE_NV: Self = Self(1000153000);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FrontFace(pub i32);
impl FrontFace {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const COUNTER_CLOCKWISE: Self = Self(0);
	pub const CLOCKWISE: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BlendFactor(pub i32);
impl BlendFactor {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ZERO: Self = Self(0);
	pub const ONE: Self = Self(1);
	pub const SRC_COLOR: Self = Self(2);
	pub const ONE_MINUS_SRC_COLOR: Self = Self(3);
	pub const DST_COLOR: Self = Self(4);
	pub const ONE_MINUS_DST_COLOR: Self = Self(5);
	pub const SRC_ALPHA: Self = Self(6);
	pub const ONE_MINUS_SRC_ALPHA: Self = Self(7);
	pub const DST_ALPHA: Self = Self(8);
	pub const ONE_MINUS_DST_ALPHA: Self = Self(9);
	pub const CONSTANT_COLOR: Self = Self(10);
	pub const ONE_MINUS_CONSTANT_COLOR: Self = Self(11);
	pub const CONSTANT_ALPHA: Self = Self(12);
	pub const ONE_MINUS_CONSTANT_ALPHA: Self = Self(13);
	pub const SRC_ALPHA_SATURATE: Self = Self(14);
	pub const SRC1_COLOR: Self = Self(15);
	pub const ONE_MINUS_SRC1_COLOR: Self = Self(16);
	pub const SRC1_ALPHA: Self = Self(17);
	pub const ONE_MINUS_SRC1_ALPHA: Self = Self(18);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BlendOp(pub i32);
impl BlendOp {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ADD: Self = Self(0);
	pub const SUBTRACT: Self = Self(1);
	pub const REVERSE_SUBTRACT: Self = Self(2);
	pub const MIN: Self = Self(3);
	pub const MAX: Self = Self(4);
	pub const ZERO_EXT: Self = Self(1000148000);
	pub const SRC_EXT: Self = Self(1000148001);
	pub const DST_EXT: Self = Self(1000148002);
	pub const SRC_OVER_EXT: Self = Self(1000148003);
	pub const DST_OVER_EXT: Self = Self(1000148004);
	pub const SRC_IN_EXT: Self = Self(1000148005);
	pub const DST_IN_EXT: Self = Self(1000148006);
	pub const SRC_OUT_EXT: Self = Self(1000148007);
	pub const DST_OUT_EXT: Self = Self(1000148008);
	pub const SRC_ATOP_EXT: Self = Self(1000148009);
	pub const DST_ATOP_EXT: Self = Self(1000148010);
	pub const XOR_EXT: Self = Self(1000148011);
	pub const MULTIPLY_EXT: Self = Self(1000148012);
	pub const SCREEN_EXT: Self = Self(1000148013);
	pub const OVERLAY_EXT: Self = Self(1000148014);
	pub const DARKEN_EXT: Self = Self(1000148015);
	pub const LIGHTEN_EXT: Self = Self(1000148016);
	pub const COLORDODGE_EXT: Self = Self(1000148017);
	pub const COLORBURN_EXT: Self = Self(1000148018);
	pub const HARDLIGHT_EXT: Self = Self(1000148019);
	pub const SOFTLIGHT_EXT: Self = Self(1000148020);
	pub const DIFFERENCE_EXT: Self = Self(1000148021);
	pub const EXCLUSION_EXT: Self = Self(1000148022);
	pub const INVERT_EXT: Self = Self(1000148023);
	pub const INVERT_RGB_EXT: Self = Self(1000148024);
	pub const LINEARDODGE_EXT: Self = Self(1000148025);
	pub const LINEARBURN_EXT: Self = Self(1000148026);
	pub const VIVIDLIGHT_EXT: Self = Self(1000148027);
	pub const LINEARLIGHT_EXT: Self = Self(1000148028);
	pub const PINLIGHT_EXT: Self = Self(1000148029);
	pub const HARDMIX_EXT: Self = Self(1000148030);
	pub const HSL_HUE_EXT: Self = Self(1000148031);
	pub const HSL_SATURATION_EXT: Self = Self(1000148032);
	pub const HSL_COLOR_EXT: Self = Self(1000148033);
	pub const HSL_LUMINOSITY_EXT: Self = Self(1000148034);
	pub const PLUS_EXT: Self = Self(1000148035);
	pub const PLUS_CLAMPED_EXT: Self = Self(1000148036);
	pub const PLUS_CLAMPED_ALPHA_EXT: Self = Self(1000148037);
	pub const PLUS_DARKER_EXT: Self = Self(1000148038);
	pub const MINUS_EXT: Self = Self(1000148039);
	pub const MINUS_CLAMPED_EXT: Self = Self(1000148040);
	pub const CONTRAST_EXT: Self = Self(1000148041);
	pub const INVERT_OVG_EXT: Self = Self(1000148042);
	pub const RED_EXT: Self = Self(1000148043);
	pub const GREEN_EXT: Self = Self(1000148044);
	pub const BLUE_EXT: Self = Self(1000148045);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StencilOp(pub i32);
impl StencilOp {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const KEEP: Self = Self(0);
	pub const ZERO: Self = Self(1);
	pub const REPLACE: Self = Self(2);
	pub const INCREMENT_AND_CLAMP: Self = Self(3);
	pub const DECREMENT_AND_CLAMP: Self = Self(4);
	pub const INVERT: Self = Self(5);
	pub const INCREMENT_AND_WRAP: Self = Self(6);
	pub const DECREMENT_AND_WRAP: Self = Self(7);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct LogicOp(pub i32);
impl LogicOp {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const CLEAR: Self = Self(0);
	pub const AND: Self = Self(1);
	pub const AND_REVERSE: Self = Self(2);
	pub const COPY: Self = Self(3);
	pub const AND_INVERTED: Self = Self(4);
	pub const NO_OP: Self = Self(5);
	pub const XOR: Self = Self(6);
	pub const OR: Self = Self(7);
	pub const NOR: Self = Self(8);
	pub const EQUIVALENT: Self = Self(9);
	pub const INVERT: Self = Self(10);
	pub const OR_REVERSE: Self = Self(11);
	pub const COPY_INVERTED: Self = Self(12);
	pub const OR_INVERTED: Self = Self(13);
	pub const NAND: Self = Self(14);
	pub const SET: Self = Self(15);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct InternalAllocationType(pub i32);
impl InternalAllocationType {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const EXECUTABLE: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SystemAllocationScope(pub i32);
impl SystemAllocationScope {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const COMMAND: Self = Self(0);
	pub const OBJECT: Self = Self(1);
	pub const CACHE: Self = Self(2);
	pub const DEVICE: Self = Self(3);
	pub const INSTANCE: Self = Self(4);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PhysicalDeviceType(pub i32);
impl PhysicalDeviceType {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const OTHER: Self = Self(0);
	pub const INTEGRATED_GPU: Self = Self(1);
	pub const DISCRETE_GPU: Self = Self(2);
	pub const VIRTUAL_GPU: Self = Self(3);
	pub const CPU: Self = Self(4);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VertexInputRate(pub i32);
impl VertexInputRate {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const VERTEX: Self = Self(0);
	pub const INSTANCE: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Format(pub i32);
impl Format {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const UNDEFINED: Self = Self(0);
	pub const R4G4_UNORM_PACK8: Self = Self(1);
	pub const R4G4B4A4_UNORM_PACK16: Self = Self(2);
	pub const B4G4R4A4_UNORM_PACK16: Self = Self(3);
	pub const R5G6B5_UNORM_PACK16: Self = Self(4);
	pub const B5G6R5_UNORM_PACK16: Self = Self(5);
	pub const R5G5B5A1_UNORM_PACK16: Self = Self(6);
	pub const B5G5R5A1_UNORM_PACK16: Self = Self(7);
	pub const A1R5G5B5_UNORM_PACK16: Self = Self(8);
	pub const R8_UNORM: Self = Self(9);
	pub const R8_SNORM: Self = Self(10);
	pub const R8_USCALED: Self = Self(11);
	pub const R8_SSCALED: Self = Self(12);
	pub const R8_UINT: Self = Self(13);
	pub const R8_SINT: Self = Self(14);
	pub const R8_SRGB: Self = Self(15);
	pub const R8G8_UNORM: Self = Self(16);
	pub const R8G8_SNORM: Self = Self(17);
	pub const R8G8_USCALED: Self = Self(18);
	pub const R8G8_SSCALED: Self = Self(19);
	pub const R8G8_UINT: Self = Self(20);
	pub const R8G8_SINT: Self = Self(21);
	pub const R8G8_SRGB: Self = Self(22);
	pub const R8G8B8_UNORM: Self = Self(23);
	pub const R8G8B8_SNORM: Self = Self(24);
	pub const R8G8B8_USCALED: Self = Self(25);
	pub const R8G8B8_SSCALED: Self = Self(26);
	pub const R8G8B8_UINT: Self = Self(27);
	pub const R8G8B8_SINT: Self = Self(28);
	pub const R8G8B8_SRGB: Self = Self(29);
	pub const B8G8R8_UNORM: Self = Self(30);
	pub const B8G8R8_SNORM: Self = Self(31);
	pub const B8G8R8_USCALED: Self = Self(32);
	pub const B8G8R8_SSCALED: Self = Self(33);
	pub const B8G8R8_UINT: Self = Self(34);
	pub const B8G8R8_SINT: Self = Self(35);
	pub const B8G8R8_SRGB: Self = Self(36);
	pub const R8G8B8A8_UNORM: Self = Self(37);
	pub const R8G8B8A8_SNORM: Self = Self(38);
	pub const R8G8B8A8_USCALED: Self = Self(39);
	pub const R8G8B8A8_SSCALED: Self = Self(40);
	pub const R8G8B8A8_UINT: Self = Self(41);
	pub const R8G8B8A8_SINT: Self = Self(42);
	pub const R8G8B8A8_SRGB: Self = Self(43);
	pub const B8G8R8A8_UNORM: Self = Self(44);
	pub const B8G8R8A8_SNORM: Self = Self(45);
	pub const B8G8R8A8_USCALED: Self = Self(46);
	pub const B8G8R8A8_SSCALED: Self = Self(47);
	pub const B8G8R8A8_UINT: Self = Self(48);
	pub const B8G8R8A8_SINT: Self = Self(49);
	pub const B8G8R8A8_SRGB: Self = Self(50);
	pub const A8B8G8R8_UNORM_PACK32: Self = Self(51);
	pub const A8B8G8R8_SNORM_PACK32: Self = Self(52);
	pub const A8B8G8R8_USCALED_PACK32: Self = Self(53);
	pub const A8B8G8R8_SSCALED_PACK32: Self = Self(54);
	pub const A8B8G8R8_UINT_PACK32: Self = Self(55);
	pub const A8B8G8R8_SINT_PACK32: Self = Self(56);
	pub const A8B8G8R8_SRGB_PACK32: Self = Self(57);
	pub const A2R10G10B10_UNORM_PACK32: Self = Self(58);
	pub const A2R10G10B10_SNORM_PACK32: Self = Self(59);
	pub const A2R10G10B10_USCALED_PACK32: Self = Self(60);
	pub const A2R10G10B10_SSCALED_PACK32: Self = Self(61);
	pub const A2R10G10B10_UINT_PACK32: Self = Self(62);
	pub const A2R10G10B10_SINT_PACK32: Self = Self(63);
	pub const A2B10G10R10_UNORM_PACK32: Self = Self(64);
	pub const A2B10G10R10_SNORM_PACK32: Self = Self(65);
	pub const A2B10G10R10_USCALED_PACK32: Self = Self(66);
	pub const A2B10G10R10_SSCALED_PACK32: Self = Self(67);
	pub const A2B10G10R10_UINT_PACK32: Self = Self(68);
	pub const A2B10G10R10_SINT_PACK32: Self = Self(69);
	pub const R16_UNORM: Self = Self(70);
	pub const R16_SNORM: Self = Self(71);
	pub const R16_USCALED: Self = Self(72);
	pub const R16_SSCALED: Self = Self(73);
	pub const R16_UINT: Self = Self(74);
	pub const R16_SINT: Self = Self(75);
	pub const R16_SFLOAT: Self = Self(76);
	pub const R16G16_UNORM: Self = Self(77);
	pub const R16G16_SNORM: Self = Self(78);
	pub const R16G16_USCALED: Self = Self(79);
	pub const R16G16_SSCALED: Self = Self(80);
	pub const R16G16_UINT: Self = Self(81);
	pub const R16G16_SINT: Self = Self(82);
	pub const R16G16_SFLOAT: Self = Self(83);
	pub const R16G16B16_UNORM: Self = Self(84);
	pub const R16G16B16_SNORM: Self = Self(85);
	pub const R16G16B16_USCALED: Self = Self(86);
	pub const R16G16B16_SSCALED: Self = Self(87);
	pub const R16G16B16_UINT: Self = Self(88);
	pub const R16G16B16_SINT: Self = Self(89);
	pub const R16G16B16_SFLOAT: Self = Self(90);
	pub const R16G16B16A16_UNORM: Self = Self(91);
	pub const R16G16B16A16_SNORM: Self = Self(92);
	pub const R16G16B16A16_USCALED: Self = Self(93);
	pub const R16G16B16A16_SSCALED: Self = Self(94);
	pub const R16G16B16A16_UINT: Self = Self(95);
	pub const R16G16B16A16_SINT: Self = Self(96);
	pub const R16G16B16A16_SFLOAT: Self = Self(97);
	pub const R32_UINT: Self = Self(98);
	pub const R32_SINT: Self = Self(99);
	pub const R32_SFLOAT: Self = Self(100);
	pub const R32G32_UINT: Self = Self(101);
	pub const R32G32_SINT: Self = Self(102);
	pub const R32G32_SFLOAT: Self = Self(103);
	pub const R32G32B32_UINT: Self = Self(104);
	pub const R32G32B32_SINT: Self = Self(105);
	pub const R32G32B32_SFLOAT: Self = Self(106);
	pub const R32G32B32A32_UINT: Self = Self(107);
	pub const R32G32B32A32_SINT: Self = Self(108);
	pub const R32G32B32A32_SFLOAT: Self = Self(109);
	pub const R64_UINT: Self = Self(110);
	pub const R64_SINT: Self = Self(111);
	pub const R64_SFLOAT: Self = Self(112);
	pub const R64G64_UINT: Self = Self(113);
	pub const R64G64_SINT: Self = Self(114);
	pub const R64G64_SFLOAT: Self = Self(115);
	pub const R64G64B64_UINT: Self = Self(116);
	pub const R64G64B64_SINT: Self = Self(117);
	pub const R64G64B64_SFLOAT: Self = Self(118);
	pub const R64G64B64A64_UINT: Self = Self(119);
	pub const R64G64B64A64_SINT: Self = Self(120);
	pub const R64G64B64A64_SFLOAT: Self = Self(121);
	pub const B10G11R11_UFLOAT_PACK32: Self = Self(122);
	pub const E5B9G9R9_UFLOAT_PACK32: Self = Self(123);
	pub const D16_UNORM: Self = Self(124);
	pub const X8_D24_UNORM_PACK32: Self = Self(125);
	pub const D32_SFLOAT: Self = Self(126);
	pub const S8_UINT: Self = Self(127);
	pub const D16_UNORM_S8_UINT: Self = Self(128);
	pub const D24_UNORM_S8_UINT: Self = Self(129);
	pub const D32_SFLOAT_S8_UINT: Self = Self(130);
	pub const BC1_RGB_UNORM_BLOCK: Self = Self(131);
	pub const BC1_RGB_SRGB_BLOCK: Self = Self(132);
	pub const BC1_RGBA_UNORM_BLOCK: Self = Self(133);
	pub const BC1_RGBA_SRGB_BLOCK: Self = Self(134);
	pub const BC2_UNORM_BLOCK: Self = Self(135);
	pub const BC2_SRGB_BLOCK: Self = Self(136);
	pub const BC3_UNORM_BLOCK: Self = Self(137);
	pub const BC3_SRGB_BLOCK: Self = Self(138);
	pub const BC4_UNORM_BLOCK: Self = Self(139);
	pub const BC4_SNORM_BLOCK: Self = Self(140);
	pub const BC5_UNORM_BLOCK: Self = Self(141);
	pub const BC5_SNORM_BLOCK: Self = Self(142);
	pub const BC6H_UFLOAT_BLOCK: Self = Self(143);
	pub const BC6H_SFLOAT_BLOCK: Self = Self(144);
	pub const BC7_UNORM_BLOCK: Self = Self(145);
	pub const BC7_SRGB_BLOCK: Self = Self(146);
	pub const ETC2_R8G8B8_UNORM_BLOCK: Self = Self(147);
	pub const ETC2_R8G8B8_SRGB_BLOCK: Self = Self(148);
	pub const ETC2_R8G8B8A1_UNORM_BLOCK: Self = Self(149);
	pub const ETC2_R8G8B8A1_SRGB_BLOCK: Self = Self(150);
	pub const ETC2_R8G8B8A8_UNORM_BLOCK: Self = Self(151);
	pub const ETC2_R8G8B8A8_SRGB_BLOCK: Self = Self(152);
	pub const EAC_R11_UNORM_BLOCK: Self = Self(153);
	pub const EAC_R11_SNORM_BLOCK: Self = Self(154);
	pub const EAC_R11G11_UNORM_BLOCK: Self = Self(155);
	pub const EAC_R11G11_SNORM_BLOCK: Self = Self(156);
	pub const ASTC_4x4_UNORM_BLOCK: Self = Self(157);
	pub const ASTC_4x4_SRGB_BLOCK: Self = Self(158);
	pub const ASTC_5x4_UNORM_BLOCK: Self = Self(159);
	pub const ASTC_5x4_SRGB_BLOCK: Self = Self(160);
	pub const ASTC_5x5_UNORM_BLOCK: Self = Self(161);
	pub const ASTC_5x5_SRGB_BLOCK: Self = Self(162);
	pub const ASTC_6x5_UNORM_BLOCK: Self = Self(163);
	pub const ASTC_6x5_SRGB_BLOCK: Self = Self(164);
	pub const ASTC_6x6_UNORM_BLOCK: Self = Self(165);
	pub const ASTC_6x6_SRGB_BLOCK: Self = Self(166);
	pub const ASTC_8x5_UNORM_BLOCK: Self = Self(167);
	pub const ASTC_8x5_SRGB_BLOCK: Self = Self(168);
	pub const ASTC_8x6_UNORM_BLOCK: Self = Self(169);
	pub const ASTC_8x6_SRGB_BLOCK: Self = Self(170);
	pub const ASTC_8x8_UNORM_BLOCK: Self = Self(171);
	pub const ASTC_8x8_SRGB_BLOCK: Self = Self(172);
	pub const ASTC_10x5_UNORM_BLOCK: Self = Self(173);
	pub const ASTC_10x5_SRGB_BLOCK: Self = Self(174);
	pub const ASTC_10x6_UNORM_BLOCK: Self = Self(175);
	pub const ASTC_10x6_SRGB_BLOCK: Self = Self(176);
	pub const ASTC_10x8_UNORM_BLOCK: Self = Self(177);
	pub const ASTC_10x8_SRGB_BLOCK: Self = Self(178);
	pub const ASTC_10x10_UNORM_BLOCK: Self = Self(179);
	pub const ASTC_10x10_SRGB_BLOCK: Self = Self(180);
	pub const ASTC_12x10_UNORM_BLOCK: Self = Self(181);
	pub const ASTC_12x10_SRGB_BLOCK: Self = Self(182);
	pub const ASTC_12x12_UNORM_BLOCK: Self = Self(183);
	pub const ASTC_12x12_SRGB_BLOCK: Self = Self(184);
	pub const G8B8G8R8_422_UNORM: Self = Self(1000156000);
	pub const B8G8R8G8_422_UNORM: Self = Self(1000156001);
	pub const G8_B8_R8_3PLANE_420_UNORM: Self = Self(1000156002);
	pub const G8_B8R8_2PLANE_420_UNORM: Self = Self(1000156003);
	pub const G8_B8_R8_3PLANE_422_UNORM: Self = Self(1000156004);
	pub const G8_B8R8_2PLANE_422_UNORM: Self = Self(1000156005);
	pub const G8_B8_R8_3PLANE_444_UNORM: Self = Self(1000156006);
	pub const R10X6_UNORM_PACK16: Self = Self(1000156007);
	pub const R10X6G10X6_UNORM_2PACK16: Self = Self(1000156008);
	pub const R10X6G10X6B10X6A10X6_UNORM_4PACK16: Self = Self(1000156009);
	pub const G10X6B10X6G10X6R10X6_422_UNORM_4PACK16: Self = Self(1000156010);
	pub const B10X6G10X6R10X6G10X6_422_UNORM_4PACK16: Self = Self(1000156011);
	pub const G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16: Self = Self(1000156012);
	pub const G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16: Self = Self(1000156013);
	pub const G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16: Self = Self(1000156014);
	pub const G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16: Self = Self(1000156015);
	pub const G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16: Self = Self(1000156016);
	pub const R12X4_UNORM_PACK16: Self = Self(1000156017);
	pub const R12X4G12X4_UNORM_2PACK16: Self = Self(1000156018);
	pub const R12X4G12X4B12X4A12X4_UNORM_4PACK16: Self = Self(1000156019);
	pub const G12X4B12X4G12X4R12X4_422_UNORM_4PACK16: Self = Self(1000156020);
	pub const B12X4G12X4R12X4G12X4_422_UNORM_4PACK16: Self = Self(1000156021);
	pub const G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16: Self = Self(1000156022);
	pub const G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16: Self = Self(1000156023);
	pub const G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16: Self = Self(1000156024);
	pub const G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16: Self = Self(1000156025);
	pub const G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16: Self = Self(1000156026);
	pub const G16B16G16R16_422_UNORM: Self = Self(1000156027);
	pub const B16G16R16G16_422_UNORM: Self = Self(1000156028);
	pub const G16_B16_R16_3PLANE_420_UNORM: Self = Self(1000156029);
	pub const G16_B16R16_2PLANE_420_UNORM: Self = Self(1000156030);
	pub const G16_B16_R16_3PLANE_422_UNORM: Self = Self(1000156031);
	pub const G16_B16R16_2PLANE_422_UNORM: Self = Self(1000156032);
	pub const G16_B16_R16_3PLANE_444_UNORM: Self = Self(1000156033);
	pub const G8_B8R8_2PLANE_444_UNORM: Self = Self(1000330000);
	pub const G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16: Self = Self(1000330001);
	pub const G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16: Self = Self(1000330002);
	pub const G16_B16R16_2PLANE_444_UNORM: Self = Self(1000330003);
	pub const A4R4G4B4_UNORM_PACK16: Self = Self(1000340000);
	pub const A4B4G4R4_UNORM_PACK16: Self = Self(1000340001);
	pub const ASTC_4x4_SFLOAT_BLOCK: Self = Self(1000066000);
	pub const ASTC_5x4_SFLOAT_BLOCK: Self = Self(1000066001);
	pub const ASTC_5x5_SFLOAT_BLOCK: Self = Self(1000066002);
	pub const ASTC_6x5_SFLOAT_BLOCK: Self = Self(1000066003);
	pub const ASTC_6x6_SFLOAT_BLOCK: Self = Self(1000066004);
	pub const ASTC_8x5_SFLOAT_BLOCK: Self = Self(1000066005);
	pub const ASTC_8x6_SFLOAT_BLOCK: Self = Self(1000066006);
	pub const ASTC_8x8_SFLOAT_BLOCK: Self = Self(1000066007);
	pub const ASTC_10x5_SFLOAT_BLOCK: Self = Self(1000066008);
	pub const ASTC_10x6_SFLOAT_BLOCK: Self = Self(1000066009);
	pub const ASTC_10x8_SFLOAT_BLOCK: Self = Self(1000066010);
	pub const ASTC_10x10_SFLOAT_BLOCK: Self = Self(1000066011);
	pub const ASTC_12x10_SFLOAT_BLOCK: Self = Self(1000066012);
	pub const ASTC_12x12_SFLOAT_BLOCK: Self = Self(1000066013);
	pub const A1B5G5R5_UNORM_PACK16: Self = Self(1000470000);
	pub const A8_UNORM: Self = Self(1000470001);
	pub const PVRTC1_2BPP_UNORM_BLOCK_IMG: Self = Self(1000054000);
	pub const PVRTC1_4BPP_UNORM_BLOCK_IMG: Self = Self(1000054001);
	pub const PVRTC2_2BPP_UNORM_BLOCK_IMG: Self = Self(1000054002);
	pub const PVRTC2_4BPP_UNORM_BLOCK_IMG: Self = Self(1000054003);
	pub const PVRTC1_2BPP_SRGB_BLOCK_IMG: Self = Self(1000054004);
	pub const PVRTC1_4BPP_SRGB_BLOCK_IMG: Self = Self(1000054005);
	pub const PVRTC2_2BPP_SRGB_BLOCK_IMG: Self = Self(1000054006);
	pub const PVRTC2_4BPP_SRGB_BLOCK_IMG: Self = Self(1000054007);
	pub const ASTC_4x4_SFLOAT_BLOCK_EXT: Self = Self(1000066000);
	pub const ASTC_5x4_SFLOAT_BLOCK_EXT: Self = Self(1000066001);
	pub const ASTC_5x5_SFLOAT_BLOCK_EXT: Self = Self(1000066002);
	pub const ASTC_6x5_SFLOAT_BLOCK_EXT: Self = Self(1000066003);
	pub const ASTC_6x6_SFLOAT_BLOCK_EXT: Self = Self(1000066004);
	pub const ASTC_8x5_SFLOAT_BLOCK_EXT: Self = Self(1000066005);
	pub const ASTC_8x6_SFLOAT_BLOCK_EXT: Self = Self(1000066006);
	pub const ASTC_8x8_SFLOAT_BLOCK_EXT: Self = Self(1000066007);
	pub const ASTC_10x5_SFLOAT_BLOCK_EXT: Self = Self(1000066008);
	pub const ASTC_10x6_SFLOAT_BLOCK_EXT: Self = Self(1000066009);
	pub const ASTC_10x8_SFLOAT_BLOCK_EXT: Self = Self(1000066010);
	pub const ASTC_10x10_SFLOAT_BLOCK_EXT: Self = Self(1000066011);
	pub const ASTC_12x10_SFLOAT_BLOCK_EXT: Self = Self(1000066012);
	pub const ASTC_12x12_SFLOAT_BLOCK_EXT: Self = Self(1000066013);
	pub const G8B8G8R8_422_UNORM_KHR: Self = Self(1000156000);
	pub const B8G8R8G8_422_UNORM_KHR: Self = Self(1000156001);
	pub const G8_B8_R8_3PLANE_420_UNORM_KHR: Self = Self(1000156002);
	pub const G8_B8R8_2PLANE_420_UNORM_KHR: Self = Self(1000156003);
	pub const G8_B8_R8_3PLANE_422_UNORM_KHR: Self = Self(1000156004);
	pub const G8_B8R8_2PLANE_422_UNORM_KHR: Self = Self(1000156005);
	pub const G8_B8_R8_3PLANE_444_UNORM_KHR: Self = Self(1000156006);
	pub const R10X6_UNORM_PACK16_KHR: Self = Self(1000156007);
	pub const R10X6G10X6_UNORM_2PACK16_KHR: Self = Self(1000156008);
	pub const R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR: Self = Self(1000156009);
	pub const G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR: Self = Self(1000156010);
	pub const B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR: Self = Self(1000156011);
	pub const G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR: Self = Self(1000156012);
	pub const G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR: Self = Self(1000156013);
	pub const G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR: Self = Self(1000156014);
	pub const G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR: Self = Self(1000156015);
	pub const G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR: Self = Self(1000156016);
	pub const R12X4_UNORM_PACK16_KHR: Self = Self(1000156017);
	pub const R12X4G12X4_UNORM_2PACK16_KHR: Self = Self(1000156018);
	pub const R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR: Self = Self(1000156019);
	pub const G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR: Self = Self(1000156020);
	pub const B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR: Self = Self(1000156021);
	pub const G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR: Self = Self(1000156022);
	pub const G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR: Self = Self(1000156023);
	pub const G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR: Self = Self(1000156024);
	pub const G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR: Self = Self(1000156025);
	pub const G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR: Self = Self(1000156026);
	pub const G16B16G16R16_422_UNORM_KHR: Self = Self(1000156027);
	pub const B16G16R16G16_422_UNORM_KHR: Self = Self(1000156028);
	pub const G16_B16_R16_3PLANE_420_UNORM_KHR: Self = Self(1000156029);
	pub const G16_B16R16_2PLANE_420_UNORM_KHR: Self = Self(1000156030);
	pub const G16_B16_R16_3PLANE_422_UNORM_KHR: Self = Self(1000156031);
	pub const G16_B16R16_2PLANE_422_UNORM_KHR: Self = Self(1000156032);
	pub const G16_B16_R16_3PLANE_444_UNORM_KHR: Self = Self(1000156033);
	pub const ASTC_3x3x3_UNORM_BLOCK_EXT: Self = Self(1000288000);
	pub const ASTC_3x3x3_SRGB_BLOCK_EXT: Self = Self(1000288001);
	pub const ASTC_3x3x3_SFLOAT_BLOCK_EXT: Self = Self(1000288002);
	pub const ASTC_4x3x3_UNORM_BLOCK_EXT: Self = Self(1000288003);
	pub const ASTC_4x3x3_SRGB_BLOCK_EXT: Self = Self(1000288004);
	pub const ASTC_4x3x3_SFLOAT_BLOCK_EXT: Self = Self(1000288005);
	pub const ASTC_4x4x3_UNORM_BLOCK_EXT: Self = Self(1000288006);
	pub const ASTC_4x4x3_SRGB_BLOCK_EXT: Self = Self(1000288007);
	pub const ASTC_4x4x3_SFLOAT_BLOCK_EXT: Self = Self(1000288008);
	pub const ASTC_4x4x4_UNORM_BLOCK_EXT: Self = Self(1000288009);
	pub const ASTC_4x4x4_SRGB_BLOCK_EXT: Self = Self(1000288010);
	pub const ASTC_4x4x4_SFLOAT_BLOCK_EXT: Self = Self(1000288011);
	pub const ASTC_5x4x4_UNORM_BLOCK_EXT: Self = Self(1000288012);
	pub const ASTC_5x4x4_SRGB_BLOCK_EXT: Self = Self(1000288013);
	pub const ASTC_5x4x4_SFLOAT_BLOCK_EXT: Self = Self(1000288014);
	pub const ASTC_5x5x4_UNORM_BLOCK_EXT: Self = Self(1000288015);
	pub const ASTC_5x5x4_SRGB_BLOCK_EXT: Self = Self(1000288016);
	pub const ASTC_5x5x4_SFLOAT_BLOCK_EXT: Self = Self(1000288017);
	pub const ASTC_5x5x5_UNORM_BLOCK_EXT: Self = Self(1000288018);
	pub const ASTC_5x5x5_SRGB_BLOCK_EXT: Self = Self(1000288019);
	pub const ASTC_5x5x5_SFLOAT_BLOCK_EXT: Self = Self(1000288020);
	pub const ASTC_6x5x5_UNORM_BLOCK_EXT: Self = Self(1000288021);
	pub const ASTC_6x5x5_SRGB_BLOCK_EXT: Self = Self(1000288022);
	pub const ASTC_6x5x5_SFLOAT_BLOCK_EXT: Self = Self(1000288023);
	pub const ASTC_6x6x5_UNORM_BLOCK_EXT: Self = Self(1000288024);
	pub const ASTC_6x6x5_SRGB_BLOCK_EXT: Self = Self(1000288025);
	pub const ASTC_6x6x5_SFLOAT_BLOCK_EXT: Self = Self(1000288026);
	pub const ASTC_6x6x6_UNORM_BLOCK_EXT: Self = Self(1000288027);
	pub const ASTC_6x6x6_SRGB_BLOCK_EXT: Self = Self(1000288028);
	pub const ASTC_6x6x6_SFLOAT_BLOCK_EXT: Self = Self(1000288029);
	pub const G8_B8R8_2PLANE_444_UNORM_EXT: Self = Self(1000330000);
	pub const G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16_EXT: Self = Self(1000330001);
	pub const G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16_EXT: Self = Self(1000330002);
	pub const G16_B16R16_2PLANE_444_UNORM_EXT: Self = Self(1000330003);
	pub const A4R4G4B4_UNORM_PACK16_EXT: Self = Self(1000340000);
	pub const A4B4G4R4_UNORM_PACK16_EXT: Self = Self(1000340001);
	pub const R8_BOOL_ARM: Self = Self(1000460000);
	pub const R16_SFLOAT_FPENCODING_BFLOAT16_ARM: Self = Self(1000460001);
	pub const R8_SFLOAT_FPENCODING_FLOAT8E4M3_ARM: Self = Self(1000460002);
	pub const R8_SFLOAT_FPENCODING_FLOAT8E5M2_ARM: Self = Self(1000460003);
	pub const R16G16_SFIXED5_NV: Self = Self(1000464000);
	pub const R16G16_S10_5_NV: Self = Self(1000464000);
	pub const A1B5G5R5_UNORM_PACK16_KHR: Self = Self(1000470000);
	pub const A8_UNORM_KHR: Self = Self(1000470001);
	pub const R10X6_UINT_PACK16_ARM: Self = Self(1000609000);
	pub const R10X6G10X6_UINT_2PACK16_ARM: Self = Self(1000609001);
	pub const R10X6G10X6B10X6A10X6_UINT_4PACK16_ARM: Self = Self(1000609002);
	pub const R12X4_UINT_PACK16_ARM: Self = Self(1000609003);
	pub const R12X4G12X4_UINT_2PACK16_ARM: Self = Self(1000609004);
	pub const R12X4G12X4B12X4A12X4_UINT_4PACK16_ARM: Self = Self(1000609005);
	pub const R14X2_UINT_PACK16_ARM: Self = Self(1000609006);
	pub const R14X2G14X2_UINT_2PACK16_ARM: Self = Self(1000609007);
	pub const R14X2G14X2B14X2A14X2_UINT_4PACK16_ARM: Self = Self(1000609008);
	pub const R14X2_UNORM_PACK16_ARM: Self = Self(1000609009);
	pub const R14X2G14X2_UNORM_2PACK16_ARM: Self = Self(1000609010);
	pub const R14X2G14X2B14X2A14X2_UNORM_4PACK16_ARM: Self = Self(1000609011);
	pub const G14X2_B14X2R14X2_2PLANE_420_UNORM_3PACK16_ARM: Self = Self(1000609012);
	pub const G14X2_B14X2R14X2_2PLANE_422_UNORM_3PACK16_ARM: Self = Self(1000609013);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StructureType(pub i32);
impl StructureType {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const APPLICATION_INFO: Self = Self(0);
	pub const INSTANCE_CREATE_INFO: Self = Self(1);
	pub const DEVICE_QUEUE_CREATE_INFO: Self = Self(2);
	pub const DEVICE_CREATE_INFO: Self = Self(3);
	pub const SUBMIT_INFO: Self = Self(4);
	pub const MEMORY_ALLOCATE_INFO: Self = Self(5);
	pub const MAPPED_MEMORY_RANGE: Self = Self(6);
	pub const BIND_SPARSE_INFO: Self = Self(7);
	pub const FENCE_CREATE_INFO: Self = Self(8);
	pub const SEMAPHORE_CREATE_INFO: Self = Self(9);
	pub const EVENT_CREATE_INFO: Self = Self(10);
	pub const QUERY_POOL_CREATE_INFO: Self = Self(11);
	pub const BUFFER_CREATE_INFO: Self = Self(12);
	pub const BUFFER_VIEW_CREATE_INFO: Self = Self(13);
	pub const IMAGE_CREATE_INFO: Self = Self(14);
	pub const IMAGE_VIEW_CREATE_INFO: Self = Self(15);
	pub const SHADER_MODULE_CREATE_INFO: Self = Self(16);
	pub const PIPELINE_CACHE_CREATE_INFO: Self = Self(17);
	pub const PIPELINE_SHADER_STAGE_CREATE_INFO: Self = Self(18);
	pub const PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: Self = Self(19);
	pub const PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: Self = Self(20);
	pub const PIPELINE_TESSELLATION_STATE_CREATE_INFO: Self = Self(21);
	pub const PIPELINE_VIEWPORT_STATE_CREATE_INFO: Self = Self(22);
	pub const PIPELINE_RASTERIZATION_STATE_CREATE_INFO: Self = Self(23);
	pub const PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: Self = Self(24);
	pub const PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: Self = Self(25);
	pub const PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: Self = Self(26);
	pub const PIPELINE_DYNAMIC_STATE_CREATE_INFO: Self = Self(27);
	pub const GRAPHICS_PIPELINE_CREATE_INFO: Self = Self(28);
	pub const COMPUTE_PIPELINE_CREATE_INFO: Self = Self(29);
	pub const PIPELINE_LAYOUT_CREATE_INFO: Self = Self(30);
	pub const SAMPLER_CREATE_INFO: Self = Self(31);
	pub const DESCRIPTOR_SET_LAYOUT_CREATE_INFO: Self = Self(32);
	pub const DESCRIPTOR_POOL_CREATE_INFO: Self = Self(33);
	pub const DESCRIPTOR_SET_ALLOCATE_INFO: Self = Self(34);
	pub const WRITE_DESCRIPTOR_SET: Self = Self(35);
	pub const COPY_DESCRIPTOR_SET: Self = Self(36);
	pub const FRAMEBUFFER_CREATE_INFO: Self = Self(37);
	pub const RENDER_PASS_CREATE_INFO: Self = Self(38);
	pub const COMMAND_POOL_CREATE_INFO: Self = Self(39);
	pub const COMMAND_BUFFER_ALLOCATE_INFO: Self = Self(40);
	pub const COMMAND_BUFFER_INHERITANCE_INFO: Self = Self(41);
	pub const COMMAND_BUFFER_BEGIN_INFO: Self = Self(42);
	pub const RENDER_PASS_BEGIN_INFO: Self = Self(43);
	pub const BUFFER_MEMORY_BARRIER: Self = Self(44);
	pub const IMAGE_MEMORY_BARRIER: Self = Self(45);
	pub const MEMORY_BARRIER: Self = Self(46);
	pub const LOADER_INSTANCE_CREATE_INFO: Self = Self(47);
	pub const LOADER_DEVICE_CREATE_INFO: Self = Self(48);
	pub const BIND_BUFFER_MEMORY_INFO: Self = Self(1000157000);
	pub const BIND_IMAGE_MEMORY_INFO: Self = Self(1000157001);
	pub const MEMORY_DEDICATED_REQUIREMENTS: Self = Self(1000127000);
	pub const MEMORY_DEDICATED_ALLOCATE_INFO: Self = Self(1000127001);
	pub const MEMORY_ALLOCATE_FLAGS_INFO: Self = Self(1000060000);
	pub const DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO: Self = Self(1000060004);
	pub const DEVICE_GROUP_SUBMIT_INFO: Self = Self(1000060005);
	pub const DEVICE_GROUP_BIND_SPARSE_INFO: Self = Self(1000060006);
	pub const BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO: Self = Self(1000060013);
	pub const BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO: Self = Self(1000060014);
	pub const PHYSICAL_DEVICE_GROUP_PROPERTIES: Self = Self(1000070000);
	pub const DEVICE_GROUP_DEVICE_CREATE_INFO: Self = Self(1000070001);
	pub const BUFFER_MEMORY_REQUIREMENTS_INFO_2: Self = Self(1000146000);
	pub const IMAGE_MEMORY_REQUIREMENTS_INFO_2: Self = Self(1000146001);
	pub const IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2: Self = Self(1000146002);
	pub const MEMORY_REQUIREMENTS_2: Self = Self(1000146003);
	pub const SPARSE_IMAGE_MEMORY_REQUIREMENTS_2: Self = Self(1000146004);
	pub const PHYSICAL_DEVICE_FEATURES_2: Self = Self(1000059000);
	pub const PHYSICAL_DEVICE_PROPERTIES_2: Self = Self(1000059001);
	pub const FORMAT_PROPERTIES_2: Self = Self(1000059002);
	pub const IMAGE_FORMAT_PROPERTIES_2: Self = Self(1000059003);
	pub const PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2: Self = Self(1000059004);
	pub const QUEUE_FAMILY_PROPERTIES_2: Self = Self(1000059005);
	pub const PHYSICAL_DEVICE_MEMORY_PROPERTIES_2: Self = Self(1000059006);
	pub const SPARSE_IMAGE_FORMAT_PROPERTIES_2: Self = Self(1000059007);
	pub const PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2: Self = Self(1000059008);
	pub const IMAGE_VIEW_USAGE_CREATE_INFO: Self = Self(1000117002);
	pub const PROTECTED_SUBMIT_INFO: Self = Self(1000145000);
	pub const PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES: Self = Self(1000145001);
	pub const PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES: Self = Self(1000145002);
	pub const DEVICE_QUEUE_INFO_2: Self = Self(1000145003);
	pub const PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO: Self = Self(1000071000);
	pub const EXTERNAL_IMAGE_FORMAT_PROPERTIES: Self = Self(1000071001);
	pub const PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO: Self = Self(1000071002);
	pub const EXTERNAL_BUFFER_PROPERTIES: Self = Self(1000071003);
	pub const PHYSICAL_DEVICE_ID_PROPERTIES: Self = Self(1000071004);
	pub const EXTERNAL_MEMORY_BUFFER_CREATE_INFO: Self = Self(1000072000);
	pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO: Self = Self(1000072001);
	pub const EXPORT_MEMORY_ALLOCATE_INFO: Self = Self(1000072002);
	pub const PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO: Self = Self(1000112000);
	pub const EXTERNAL_FENCE_PROPERTIES: Self = Self(1000112001);
	pub const EXPORT_FENCE_CREATE_INFO: Self = Self(1000113000);
	pub const EXPORT_SEMAPHORE_CREATE_INFO: Self = Self(1000077000);
	pub const PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO: Self = Self(1000076000);
	pub const EXTERNAL_SEMAPHORE_PROPERTIES: Self = Self(1000076001);
	pub const PHYSICAL_DEVICE_SUBGROUP_PROPERTIES: Self = Self(1000094000);
	pub const PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES: Self = Self(1000083000);
	pub const PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES: Self = Self(1000120000);
	pub const PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES: Self = Self(1000120000);
	pub const DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO: Self = Self(1000085000);
	pub const PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES: Self = Self(1000168000);
	pub const DESCRIPTOR_SET_LAYOUT_SUPPORT: Self = Self(1000168001);
	pub const SAMPLER_YCBCR_CONVERSION_CREATE_INFO: Self = Self(1000156000);
	pub const SAMPLER_YCBCR_CONVERSION_INFO: Self = Self(1000156001);
	pub const BIND_IMAGE_PLANE_MEMORY_INFO: Self = Self(1000156002);
	pub const IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO: Self = Self(1000156003);
	pub const PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES: Self = Self(1000156004);
	pub const SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES: Self = Self(1000156005);
	pub const DEVICE_GROUP_RENDER_PASS_BEGIN_INFO: Self = Self(1000060003);
	pub const PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES: Self = Self(1000117000);
	pub const RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO: Self = Self(1000117001);
	pub const PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO: Self = Self(1000117003);
	pub const RENDER_PASS_MULTIVIEW_CREATE_INFO: Self = Self(1000053000);
	pub const PHYSICAL_DEVICE_MULTIVIEW_FEATURES: Self = Self(1000053001);
	pub const PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES: Self = Self(1000053002);
	pub const PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES: Self = Self(1000063000);
	pub const PHYSICAL_DEVICE_SHADER_DRAW_PARAMETER_FEATURES: Self = Self(1000063000);
	pub const PHYSICAL_DEVICE_DRIVER_PROPERTIES: Self = Self(1000196000);
	pub const PHYSICAL_DEVICE_VULKAN_1_1_FEATURES: Self = Self(49);
	pub const PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES: Self = Self(50);
	pub const PHYSICAL_DEVICE_VULKAN_1_2_FEATURES: Self = Self(51);
	pub const PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES: Self = Self(52);
	pub const IMAGE_FORMAT_LIST_CREATE_INFO: Self = Self(1000147000);
	pub const PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES: Self = Self(1000211000);
	pub const PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES: Self = Self(1000261000);
	pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES: Self = Self(1000207000);
	pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES: Self = Self(1000207001);
	pub const SEMAPHORE_TYPE_CREATE_INFO: Self = Self(1000207002);
	pub const TIMELINE_SEMAPHORE_SUBMIT_INFO: Self = Self(1000207003);
	pub const SEMAPHORE_WAIT_INFO: Self = Self(1000207004);
	pub const SEMAPHORE_SIGNAL_INFO: Self = Self(1000207005);
	pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES: Self = Self(1000257000);
	pub const BUFFER_DEVICE_ADDRESS_INFO: Self = Self(1000244001);
	pub const BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO: Self = Self(1000257002);
	pub const MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO: Self = Self(1000257003);
	pub const DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO: Self = Self(1000257004);
	pub const PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES: Self = Self(1000177000);
	pub const PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES: Self = Self(1000180000);
	pub const PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES: Self = Self(1000082000);
	pub const PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES: Self = Self(1000197000);
	pub const DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO: Self = Self(1000161000);
	pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES: Self = Self(1000161001);
	pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES: Self = Self(1000161002);
	pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO: Self = Self(1000161003);
	pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT: Self = Self(1000161004);
	pub const PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES: Self = Self(1000221000);
	pub const PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES: Self = Self(1000130000);
	pub const SAMPLER_REDUCTION_MODE_CREATE_INFO: Self = Self(1000130001);
	pub const PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES: Self = Self(1000253000);
	pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES: Self = Self(1000175000);
	pub const ATTACHMENT_DESCRIPTION_2: Self = Self(1000109000);
	pub const ATTACHMENT_REFERENCE_2: Self = Self(1000109001);
	pub const SUBPASS_DESCRIPTION_2: Self = Self(1000109002);
	pub const SUBPASS_DEPENDENCY_2: Self = Self(1000109003);
	pub const RENDER_PASS_CREATE_INFO_2: Self = Self(1000109004);
	pub const SUBPASS_BEGIN_INFO: Self = Self(1000109005);
	pub const SUBPASS_END_INFO: Self = Self(1000109006);
	pub const PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES: Self = Self(1000199000);
	pub const SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE: Self = Self(1000199001);
	pub const IMAGE_STENCIL_USAGE_CREATE_INFO: Self = Self(1000246000);
	pub const PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES: Self = Self(1000108000);
	pub const FRAMEBUFFER_ATTACHMENTS_CREATE_INFO: Self = Self(1000108001);
	pub const FRAMEBUFFER_ATTACHMENT_IMAGE_INFO: Self = Self(1000108002);
	pub const RENDER_PASS_ATTACHMENT_BEGIN_INFO: Self = Self(1000108003);
	pub const PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES: Self = Self(1000241000);
	pub const ATTACHMENT_REFERENCE_STENCIL_LAYOUT: Self = Self(1000241001);
	pub const ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT: Self = Self(1000241002);
	pub const PHYSICAL_DEVICE_VULKAN_1_3_FEATURES: Self = Self(53);
	pub const PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES: Self = Self(54);
	pub const PHYSICAL_DEVICE_TOOL_PROPERTIES: Self = Self(1000245000);
	pub const PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES: Self = Self(1000295000);
	pub const DEVICE_PRIVATE_DATA_CREATE_INFO: Self = Self(1000295001);
	pub const PRIVATE_DATA_SLOT_CREATE_INFO: Self = Self(1000295002);
	pub const MEMORY_BARRIER_2: Self = Self(1000314000);
	pub const BUFFER_MEMORY_BARRIER_2: Self = Self(1000314001);
	pub const IMAGE_MEMORY_BARRIER_2: Self = Self(1000314002);
	pub const DEPENDENCY_INFO: Self = Self(1000314003);
	pub const SUBMIT_INFO_2: Self = Self(1000314004);
	pub const SEMAPHORE_SUBMIT_INFO: Self = Self(1000314005);
	pub const COMMAND_BUFFER_SUBMIT_INFO: Self = Self(1000314006);
	pub const PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES: Self = Self(1000314007);
	pub const COPY_BUFFER_INFO_2: Self = Self(1000337000);
	pub const COPY_IMAGE_INFO_2: Self = Self(1000337001);
	pub const COPY_BUFFER_TO_IMAGE_INFO_2: Self = Self(1000337002);
	pub const COPY_IMAGE_TO_BUFFER_INFO_2: Self = Self(1000337003);
	pub const BUFFER_COPY_2: Self = Self(1000337006);
	pub const IMAGE_COPY_2: Self = Self(1000337007);
	pub const BUFFER_IMAGE_COPY_2: Self = Self(1000337009);
	pub const PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES: Self = Self(1000066000);
	pub const FORMAT_PROPERTIES_3: Self = Self(1000360000);
	pub const PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES: Self = Self(1000413000);
	pub const PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES: Self = Self(1000413001);
	pub const DEVICE_BUFFER_MEMORY_REQUIREMENTS: Self = Self(1000413002);
	pub const DEVICE_IMAGE_MEMORY_REQUIREMENTS: Self = Self(1000413003);
	pub const PIPELINE_CREATION_FEEDBACK_CREATE_INFO: Self = Self(1000192000);
	pub const PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES: Self = Self(1000215000);
	pub const PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES: Self = Self(1000276000);
	pub const PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES: Self = Self(1000297000);
	pub const PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES: Self = Self(1000325000);
	pub const PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES: Self = Self(1000335000);
	pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES: Self = Self(1000225000);
	pub const PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO: Self = Self(1000225001);
	pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES: Self = Self(1000225002);
	pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES: Self = Self(1000138000);
	pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES: Self = Self(1000138001);
	pub const WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK: Self = Self(1000138002);
	pub const DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO: Self = Self(1000138003);
	pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES: Self = Self(1000280000);
	pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES: Self = Self(1000280001);
	pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES: Self = Self(1000281001);
	pub const BLIT_IMAGE_INFO_2: Self = Self(1000337004);
	pub const RESOLVE_IMAGE_INFO_2: Self = Self(1000337005);
	pub const IMAGE_BLIT_2: Self = Self(1000337008);
	pub const IMAGE_RESOLVE_2: Self = Self(1000337010);
	pub const RENDERING_INFO: Self = Self(1000044000);
	pub const RENDERING_ATTACHMENT_INFO: Self = Self(1000044001);
	pub const PIPELINE_RENDERING_CREATE_INFO: Self = Self(1000044002);
	pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES: Self = Self(1000044003);
	pub const COMMAND_BUFFER_INHERITANCE_RENDERING_INFO: Self = Self(1000044004);
	pub const PHYSICAL_DEVICE_VULKAN_1_4_FEATURES: Self = Self(55);
	pub const PHYSICAL_DEVICE_VULKAN_1_4_PROPERTIES: Self = Self(56);
	pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO: Self = Self(1000174000);
	pub const PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES: Self = Self(1000388000);
	pub const QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES: Self = Self(1000388001);
	pub const PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES: Self = Self(1000265000);
	pub const MEMORY_MAP_INFO: Self = Self(1000271000);
	pub const MEMORY_UNMAP_INFO: Self = Self(1000271001);
	pub const PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES: Self = Self(1000470000);
	pub const PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES: Self = Self(1000470001);
	pub const DEVICE_IMAGE_SUBRESOURCE_INFO: Self = Self(1000470004);
	pub const SUBRESOURCE_LAYOUT_2: Self = Self(1000338002);
	pub const IMAGE_SUBRESOURCE_2: Self = Self(1000338003);
	pub const BUFFER_USAGE_FLAGS_2_CREATE_INFO: Self = Self(1000470006);
	pub const PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES: Self = Self(1000545000);
	pub const PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES: Self = Self(1000545001);
	pub const BIND_MEMORY_STATUS: Self = Self(1000545002);
	pub const PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES: Self = Self(1000270000);
	pub const PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES: Self = Self(1000270001);
	pub const MEMORY_TO_IMAGE_COPY: Self = Self(1000270002);
	pub const IMAGE_TO_MEMORY_COPY: Self = Self(1000270003);
	pub const COPY_IMAGE_TO_MEMORY_INFO: Self = Self(1000270004);
	pub const COPY_MEMORY_TO_IMAGE_INFO: Self = Self(1000270005);
	pub const HOST_IMAGE_LAYOUT_TRANSITION_INFO: Self = Self(1000270006);
	pub const COPY_IMAGE_TO_IMAGE_INFO: Self = Self(1000270007);
	pub const SUBRESOURCE_HOST_MEMCPY_SIZE: Self = Self(1000270008);
	pub const HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY: Self = Self(1000270009);
	pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES: Self = Self(1000416000);
	pub const PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES: Self = Self(1000528000);
	pub const PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES: Self = Self(1000544000);
	pub const PIPELINE_CREATE_FLAGS_2_CREATE_INFO: Self = Self(1000470005);
	pub const PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES: Self = Self(1000080000);
	pub const BIND_DESCRIPTOR_SETS_INFO: Self = Self(1000545003);
	pub const PUSH_CONSTANTS_INFO: Self = Self(1000545004);
	pub const PUSH_DESCRIPTOR_SET_INFO: Self = Self(1000545005);
	pub const PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO: Self = Self(1000545006);
	pub const PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES: Self = Self(1000466000);
	pub const PIPELINE_ROBUSTNESS_CREATE_INFO: Self = Self(1000068000);
	pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES: Self = Self(1000068001);
	pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES: Self = Self(1000068002);
	pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES: Self = Self(1000259000);
	pub const PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO: Self = Self(1000259001);
	pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES: Self = Self(1000259002);
	pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES: Self = Self(1000525000);
	pub const PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO: Self = Self(1000190001);
	pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES: Self = Self(1000190002);
	pub const RENDERING_AREA_INFO: Self = Self(1000470003);
	pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES: Self = Self(1000232000);
	pub const RENDERING_ATTACHMENT_LOCATION_INFO: Self = Self(1000232001);
	pub const RENDERING_INPUT_ATTACHMENT_INDEX_INFO: Self = Self(1000232002);
	pub const SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1000001000);
	pub const PRESENT_INFO_KHR: Self = Self(1000001001);
	pub const DEVICE_GROUP_PRESENT_CAPABILITIES_KHR: Self = Self(1000060007);
	pub const IMAGE_SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1000060008);
	pub const BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR: Self = Self(1000060009);
	pub const ACQUIRE_NEXT_IMAGE_INFO_KHR: Self = Self(1000060010);
	pub const DEVICE_GROUP_PRESENT_INFO_KHR: Self = Self(1000060011);
	pub const DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1000060012);
	pub const DISPLAY_MODE_CREATE_INFO_KHR: Self = Self(1000002000);
	pub const DISPLAY_SURFACE_CREATE_INFO_KHR: Self = Self(1000002001);
	pub const DISPLAY_PRESENT_INFO_KHR: Self = Self(1000003000);
	#[cfg(feature = "platform-xlib")]
	pub const XLIB_SURFACE_CREATE_INFO_KHR: Self = Self(1000004000);
	#[cfg(feature = "platform-xcb")]
	pub const XCB_SURFACE_CREATE_INFO_KHR: Self = Self(1000005000);
	#[cfg(feature = "platform-wayland")]
	pub const WAYLAND_SURFACE_CREATE_INFO_KHR: Self = Self(1000006000);
	#[cfg(feature = "platform-android")]
	pub const ANDROID_SURFACE_CREATE_INFO_KHR: Self = Self(1000008000);
	#[cfg(feature = "platform-win32")]
	pub const WIN32_SURFACE_CREATE_INFO_KHR: Self = Self(1000009000);
	pub const DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT: Self = Self(1000011000);
	pub const DEBUG_REPORT_CREATE_INFO_EXT: Self = Self(1000011000);
	pub const PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD: Self = Self(1000018000);
	pub const DEBUG_MARKER_OBJECT_NAME_INFO_EXT: Self = Self(1000022000);
	pub const DEBUG_MARKER_OBJECT_TAG_INFO_EXT: Self = Self(1000022001);
	pub const DEBUG_MARKER_MARKER_INFO_EXT: Self = Self(1000022002);
	pub const VIDEO_PROFILE_INFO_KHR: Self = Self(1000023000);
	pub const VIDEO_CAPABILITIES_KHR: Self = Self(1000023001);
	pub const VIDEO_PICTURE_RESOURCE_INFO_KHR: Self = Self(1000023002);
	pub const VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR: Self = Self(1000023003);
	pub const BIND_VIDEO_SESSION_MEMORY_INFO_KHR: Self = Self(1000023004);
	pub const VIDEO_SESSION_CREATE_INFO_KHR: Self = Self(1000023005);
	pub const VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000023006);
	pub const VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR: Self = Self(1000023007);
	pub const VIDEO_BEGIN_CODING_INFO_KHR: Self = Self(1000023008);
	pub const VIDEO_END_CODING_INFO_KHR: Self = Self(1000023009);
	pub const VIDEO_CODING_CONTROL_INFO_KHR: Self = Self(1000023010);
	pub const VIDEO_REFERENCE_SLOT_INFO_KHR: Self = Self(1000023011);
	pub const QUEUE_FAMILY_VIDEO_PROPERTIES_KHR: Self = Self(1000023012);
	pub const VIDEO_PROFILE_LIST_INFO_KHR: Self = Self(1000023013);
	pub const PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR: Self = Self(1000023014);
	pub const VIDEO_FORMAT_PROPERTIES_KHR: Self = Self(1000023015);
	pub const QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR: Self = Self(1000023016);
	pub const VIDEO_DECODE_INFO_KHR: Self = Self(1000024000);
	pub const VIDEO_DECODE_CAPABILITIES_KHR: Self = Self(1000024001);
	pub const VIDEO_DECODE_USAGE_INFO_KHR: Self = Self(1000024002);
	pub const DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV: Self = Self(1000026000);
	pub const DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV: Self = Self(1000026001);
	pub const DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV: Self = Self(1000026002);
	pub const PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT: Self = Self(1000028000);
	pub const PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT: Self = Self(1000028001);
	pub const PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT: Self = Self(1000028002);
	pub const CU_MODULE_CREATE_INFO_NVX: Self = Self(1000029000);
	pub const CU_FUNCTION_CREATE_INFO_NVX: Self = Self(1000029001);
	pub const CU_LAUNCH_INFO_NVX: Self = Self(1000029002);
	pub const CU_MODULE_TEXTURING_MODE_CREATE_INFO_NVX: Self = Self(1000029004);
	pub const IMAGE_VIEW_HANDLE_INFO_NVX: Self = Self(1000030000);
	pub const IMAGE_VIEW_ADDRESS_PROPERTIES_NVX: Self = Self(1000030001);
	pub const VIDEO_ENCODE_H264_CAPABILITIES_KHR: Self = Self(1000038000);
	pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000038001);
	pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1000038002);
	pub const VIDEO_ENCODE_H264_PICTURE_INFO_KHR: Self = Self(1000038003);
	pub const VIDEO_ENCODE_H264_DPB_SLOT_INFO_KHR: Self = Self(1000038004);
	pub const VIDEO_ENCODE_H264_NALU_SLICE_INFO_KHR: Self = Self(1000038005);
	pub const VIDEO_ENCODE_H264_GOP_REMAINING_FRAME_INFO_KHR: Self = Self(1000038006);
	pub const VIDEO_ENCODE_H264_PROFILE_INFO_KHR: Self = Self(1000038007);
	pub const VIDEO_ENCODE_H264_RATE_CONTROL_INFO_KHR: Self = Self(1000038008);
	pub const VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1000038009);
	pub const VIDEO_ENCODE_H264_SESSION_CREATE_INFO_KHR: Self = Self(1000038010);
	pub const VIDEO_ENCODE_H264_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1000038011);
	pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_GET_INFO_KHR: Self = Self(1000038012);
	pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_FEEDBACK_INFO_KHR: Self = Self(1000038013);
	pub const VIDEO_ENCODE_H265_CAPABILITIES_KHR: Self = Self(1000039000);
	pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000039001);
	pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1000039002);
	pub const VIDEO_ENCODE_H265_PICTURE_INFO_KHR: Self = Self(1000039003);
	pub const VIDEO_ENCODE_H265_DPB_SLOT_INFO_KHR: Self = Self(1000039004);
	pub const VIDEO_ENCODE_H265_NALU_SLICE_SEGMENT_INFO_KHR: Self = Self(1000039005);
	pub const VIDEO_ENCODE_H265_GOP_REMAINING_FRAME_INFO_KHR: Self = Self(1000039006);
	pub const VIDEO_ENCODE_H265_PROFILE_INFO_KHR: Self = Self(1000039007);
	pub const VIDEO_ENCODE_H265_RATE_CONTROL_INFO_KHR: Self = Self(1000039009);
	pub const VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1000039010);
	pub const VIDEO_ENCODE_H265_SESSION_CREATE_INFO_KHR: Self = Self(1000039011);
	pub const VIDEO_ENCODE_H265_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1000039012);
	pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_GET_INFO_KHR: Self = Self(1000039013);
	pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_FEEDBACK_INFO_KHR: Self = Self(1000039014);
	pub const VIDEO_DECODE_H264_CAPABILITIES_KHR: Self = Self(1000040000);
	pub const VIDEO_DECODE_H264_PICTURE_INFO_KHR: Self = Self(1000040001);
	pub const VIDEO_DECODE_H264_PROFILE_INFO_KHR: Self = Self(1000040003);
	pub const VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000040004);
	pub const VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1000040005);
	pub const VIDEO_DECODE_H264_DPB_SLOT_INFO_KHR: Self = Self(1000040006);
	pub const TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD: Self = Self(1000041000);
	pub const RENDERING_INFO_KHR: Self = Self(1000044000);
	pub const RENDERING_ATTACHMENT_INFO_KHR: Self = Self(1000044001);
	pub const PIPELINE_RENDERING_CREATE_INFO_KHR: Self = Self(1000044002);
	pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES_KHR: Self = Self(1000044003);
	pub const COMMAND_BUFFER_INHERITANCE_RENDERING_INFO_KHR: Self = Self(1000044004);
	#[cfg(feature = "platform-ggp")]
	pub const STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP: Self = Self(1000049000);
	pub const PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV: Self = Self(1000050000);
	pub const RENDER_PASS_MULTIVIEW_CREATE_INFO_KHR: Self = Self(1000053000);
	pub const PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHR: Self = Self(1000053001);
	pub const PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHR: Self = Self(1000053002);
	pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV: Self = Self(1000056000);
	pub const EXPORT_MEMORY_ALLOCATE_INFO_NV: Self = Self(1000056001);
	#[cfg(feature = "platform-win32")]
	pub const IMPORT_MEMORY_WIN32_HANDLE_INFO_NV: Self = Self(1000057000);
	#[cfg(feature = "platform-win32")]
	pub const EXPORT_MEMORY_WIN32_HANDLE_INFO_NV: Self = Self(1000057001);
	#[cfg(feature = "platform-win32")]
	pub const WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV: Self = Self(1000058000);
	pub const PHYSICAL_DEVICE_FEATURES_2_KHR: Self = Self(1000059000);
	pub const PHYSICAL_DEVICE_PROPERTIES_2_KHR: Self = Self(1000059001);
	pub const FORMAT_PROPERTIES_2_KHR: Self = Self(1000059002);
	pub const IMAGE_FORMAT_PROPERTIES_2_KHR: Self = Self(1000059003);
	pub const PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR: Self = Self(1000059004);
	pub const QUEUE_FAMILY_PROPERTIES_2_KHR: Self = Self(1000059005);
	pub const PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR: Self = Self(1000059006);
	pub const SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR: Self = Self(1000059007);
	pub const PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR: Self = Self(1000059008);
	pub const MEMORY_ALLOCATE_FLAGS_INFO_KHR: Self = Self(1000060000);
	pub const DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHR: Self = Self(1000060003);
	pub const DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHR: Self = Self(1000060004);
	pub const DEVICE_GROUP_SUBMIT_INFO_KHR: Self = Self(1000060005);
	pub const DEVICE_GROUP_BIND_SPARSE_INFO_KHR: Self = Self(1000060006);
	pub const BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHR: Self = Self(1000060013);
	pub const BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHR: Self = Self(1000060014);
	pub const VALIDATION_FLAGS_EXT: Self = Self(1000061000);
	#[cfg(feature = "platform-vi")]
	pub const VI_SURFACE_CREATE_INFO_NN: Self = Self(1000062000);
	pub const PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES_EXT: Self = Self(1000066000);
	pub const IMAGE_VIEW_ASTC_DECODE_MODE_EXT: Self = Self(1000067000);
	pub const PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT: Self = Self(1000067001);
	pub const PIPELINE_ROBUSTNESS_CREATE_INFO_EXT: Self = Self(1000068000);
	pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES_EXT: Self = Self(1000068001);
	pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES_EXT: Self = Self(1000068002);
	pub const PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR: Self = Self(1000070000);
	pub const DEVICE_GROUP_DEVICE_CREATE_INFO_KHR: Self = Self(1000070001);
	pub const PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR: Self = Self(1000071000);
	pub const EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHR: Self = Self(1000071001);
	pub const PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR: Self = Self(1000071002);
	pub const EXTERNAL_BUFFER_PROPERTIES_KHR: Self = Self(1000071003);
	pub const PHYSICAL_DEVICE_ID_PROPERTIES_KHR: Self = Self(1000071004);
	pub const EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR: Self = Self(1000072000);
	pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR: Self = Self(1000072001);
	pub const EXPORT_MEMORY_ALLOCATE_INFO_KHR: Self = Self(1000072002);
	#[cfg(feature = "platform-win32")]
	pub const IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR: Self = Self(1000073000);
	#[cfg(feature = "platform-win32")]
	pub const EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR: Self = Self(1000073001);
	#[cfg(feature = "platform-win32")]
	pub const MEMORY_WIN32_HANDLE_PROPERTIES_KHR: Self = Self(1000073002);
	#[cfg(feature = "platform-win32")]
	pub const MEMORY_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1000073003);
	pub const IMPORT_MEMORY_FD_INFO_KHR: Self = Self(1000074000);
	pub const MEMORY_FD_PROPERTIES_KHR: Self = Self(1000074001);
	pub const MEMORY_GET_FD_INFO_KHR: Self = Self(1000074002);
	#[cfg(feature = "platform-win32")]
	pub const WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR: Self = Self(1000075000);
	pub const PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR: Self = Self(1000076000);
	pub const EXTERNAL_SEMAPHORE_PROPERTIES_KHR: Self = Self(1000076001);
	pub const EXPORT_SEMAPHORE_CREATE_INFO_KHR: Self = Self(1000077000);
	#[cfg(feature = "platform-win32")]
	pub const IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: Self = Self(1000078000);
	#[cfg(feature = "platform-win32")]
	pub const EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: Self = Self(1000078001);
	#[cfg(feature = "platform-win32")]
	pub const D3D12_FENCE_SUBMIT_INFO_KHR: Self = Self(1000078002);
	#[cfg(feature = "platform-win32")]
	pub const SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1000078003);
	pub const IMPORT_SEMAPHORE_FD_INFO_KHR: Self = Self(1000079000);
	pub const SEMAPHORE_GET_FD_INFO_KHR: Self = Self(1000079001);
	pub const PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR: Self = Self(1000080000);
	pub const COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT: Self = Self(1000081000);
	pub const PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT: Self = Self(1000081001);
	pub const CONDITIONAL_RENDERING_BEGIN_INFO_EXT: Self = Self(1000081002);
	pub const PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES_KHR: Self = Self(1000082000);
	pub const PHYSICAL_DEVICE_FLOAT16_INT8_FEATURES_KHR: Self = Self(1000082000);
	pub const PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR: Self = Self(1000083000);
	pub const PRESENT_REGIONS_KHR: Self = Self(1000084000);
	pub const DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR: Self = Self(1000085000);
	pub const PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV: Self = Self(1000087000);
	pub const SURFACE_CAPABILITIES_2_EXT: Self = Self(1000090000);
	pub const SURFACE_CAPABILITIES2_EXT: Self = Self(1000090000);
	pub const DISPLAY_POWER_INFO_EXT: Self = Self(1000091000);
	pub const DEVICE_EVENT_INFO_EXT: Self = Self(1000091001);
	pub const DISPLAY_EVENT_INFO_EXT: Self = Self(1000091002);
	pub const SWAPCHAIN_COUNTER_CREATE_INFO_EXT: Self = Self(1000091003);
	pub const PRESENT_TIMES_INFO_GOOGLE: Self = Self(1000092000);
	pub const PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX: Self = Self(1000097000);
	pub const MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX: Self = Self(1000044009);
	pub const PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV: Self = Self(1000098000);
	pub const PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT: Self = Self(1000099000);
	pub const PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT: Self = Self(1000099001);
	pub const PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT: Self = Self(1000101000);
	pub const PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT: Self = Self(1000101001);
	pub const PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT: Self = Self(1000102000);
	pub const PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT: Self = Self(1000102001);
	pub const HDR_METADATA_EXT: Self = Self(1000105000);
	pub const PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES_KHR: Self = Self(1000108000);
	pub const FRAMEBUFFER_ATTACHMENTS_CREATE_INFO_KHR: Self = Self(1000108001);
	pub const FRAMEBUFFER_ATTACHMENT_IMAGE_INFO_KHR: Self = Self(1000108002);
	pub const RENDER_PASS_ATTACHMENT_BEGIN_INFO_KHR: Self = Self(1000108003);
	pub const ATTACHMENT_DESCRIPTION_2_KHR: Self = Self(1000109000);
	pub const ATTACHMENT_REFERENCE_2_KHR: Self = Self(1000109001);
	pub const SUBPASS_DESCRIPTION_2_KHR: Self = Self(1000109002);
	pub const SUBPASS_DEPENDENCY_2_KHR: Self = Self(1000109003);
	pub const RENDER_PASS_CREATE_INFO_2_KHR: Self = Self(1000109004);
	pub const SUBPASS_BEGIN_INFO_KHR: Self = Self(1000109005);
	pub const SUBPASS_END_INFO_KHR: Self = Self(1000109006);
	pub const PHYSICAL_DEVICE_RELAXED_LINE_RASTERIZATION_FEATURES_IMG: Self = Self(1000110000);
	pub const SHARED_PRESENT_SURFACE_CAPABILITIES_KHR: Self = Self(1000111000);
	pub const PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR: Self = Self(1000112000);
	pub const EXTERNAL_FENCE_PROPERTIES_KHR: Self = Self(1000112001);
	pub const EXPORT_FENCE_CREATE_INFO_KHR: Self = Self(1000113000);
	#[cfg(feature = "platform-win32")]
	pub const IMPORT_FENCE_WIN32_HANDLE_INFO_KHR: Self = Self(1000114000);
	#[cfg(feature = "platform-win32")]
	pub const EXPORT_FENCE_WIN32_HANDLE_INFO_KHR: Self = Self(1000114001);
	#[cfg(feature = "platform-win32")]
	pub const FENCE_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1000114002);
	pub const IMPORT_FENCE_FD_INFO_KHR: Self = Self(1000115000);
	pub const FENCE_GET_FD_INFO_KHR: Self = Self(1000115001);
	pub const PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR: Self = Self(1000116000);
	pub const PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR: Self = Self(1000116001);
	pub const QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR: Self = Self(1000116002);
	pub const PERFORMANCE_QUERY_SUBMIT_INFO_KHR: Self = Self(1000116003);
	pub const ACQUIRE_PROFILING_LOCK_INFO_KHR: Self = Self(1000116004);
	pub const PERFORMANCE_COUNTER_KHR: Self = Self(1000116005);
	pub const PERFORMANCE_COUNTER_DESCRIPTION_KHR: Self = Self(1000116006);
	pub const PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES_KHR: Self = Self(1000117000);
	pub const RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR: Self = Self(1000117001);
	pub const IMAGE_VIEW_USAGE_CREATE_INFO_KHR: Self = Self(1000117002);
	pub const PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO_KHR: Self = Self(1000117003);
	pub const PHYSICAL_DEVICE_SURFACE_INFO_2_KHR: Self = Self(1000119000);
	pub const SURFACE_CAPABILITIES_2_KHR: Self = Self(1000119001);
	pub const SURFACE_FORMAT_2_KHR: Self = Self(1000119002);
	pub const PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR: Self = Self(1000120000);
	pub const PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR: Self = Self(1000120000);
	pub const DISPLAY_PROPERTIES_2_KHR: Self = Self(1000121000);
	pub const DISPLAY_PLANE_PROPERTIES_2_KHR: Self = Self(1000121001);
	pub const DISPLAY_MODE_PROPERTIES_2_KHR: Self = Self(1000121002);
	pub const DISPLAY_PLANE_INFO_2_KHR: Self = Self(1000121003);
	pub const DISPLAY_PLANE_CAPABILITIES_2_KHR: Self = Self(1000121004);
	#[cfg(feature = "platform-ios")]
	pub const IOS_SURFACE_CREATE_INFO_MVK: Self = Self(1000122000);
	#[cfg(feature = "platform-macos")]
	pub const MACOS_SURFACE_CREATE_INFO_MVK: Self = Self(1000123000);
	pub const MEMORY_DEDICATED_REQUIREMENTS_KHR: Self = Self(1000127000);
	pub const MEMORY_DEDICATED_ALLOCATE_INFO_KHR: Self = Self(1000127001);
	pub const DEBUG_UTILS_OBJECT_NAME_INFO_EXT: Self = Self(1000128000);
	pub const DEBUG_UTILS_OBJECT_TAG_INFO_EXT: Self = Self(1000128001);
	pub const DEBUG_UTILS_LABEL_EXT: Self = Self(1000128002);
	pub const DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT: Self = Self(1000128003);
	pub const DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT: Self = Self(1000128004);
	#[cfg(feature = "platform-android")]
	pub const ANDROID_HARDWARE_BUFFER_USAGE_ANDROID: Self = Self(1000129000);
	#[cfg(feature = "platform-android")]
	pub const ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID: Self = Self(1000129001);
	#[cfg(feature = "platform-android")]
	pub const ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID: Self = Self(1000129002);
	#[cfg(feature = "platform-android")]
	pub const IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: Self = Self(1000129003);
	#[cfg(feature = "platform-android")]
	pub const MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: Self = Self(1000129004);
	#[cfg(feature = "platform-android")]
	pub const EXTERNAL_FORMAT_ANDROID: Self = Self(1000129005);
	#[cfg(feature = "platform-android")]
	pub const ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID: Self = Self(1000129006);
	pub const PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT: Self = Self(1000130000);
	pub const SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT: Self = Self(1000130001);
	pub const PHYSICAL_DEVICE_GPA_FEATURES_AMD: Self = Self(1000133000);
	pub const PHYSICAL_DEVICE_GPA_PROPERTIES_AMD: Self = Self(1000133001);
	pub const GPA_SAMPLE_BEGIN_INFO_AMD: Self = Self(1000133002);
	pub const GPA_SESSION_CREATE_INFO_AMD: Self = Self(1000133003);
	pub const GPA_DEVICE_CLOCK_MODE_INFO_AMD: Self = Self(1000133004);
	pub const PHYSICAL_DEVICE_GPA_PROPERTIES_2_AMD: Self = Self(1000133005);
	pub const GPA_DEVICE_GET_CLOCK_INFO_AMD: Self = Self(1000133006);
	#[cfg(feature = "provisional")]
	pub const PHYSICAL_DEVICE_SHADER_ENQUEUE_FEATURES_AMDX: Self = Self(1000134000);
	#[cfg(feature = "provisional")]
	pub const PHYSICAL_DEVICE_SHADER_ENQUEUE_PROPERTIES_AMDX: Self = Self(1000134001);
	#[cfg(feature = "provisional")]
	pub const EXECUTION_GRAPH_PIPELINE_SCRATCH_SIZE_AMDX: Self = Self(1000134002);
	#[cfg(feature = "provisional")]
	pub const EXECUTION_GRAPH_PIPELINE_CREATE_INFO_AMDX: Self = Self(1000134003);
	#[cfg(feature = "provisional")]
	pub const PIPELINE_SHADER_STAGE_NODE_CREATE_INFO_AMDX: Self = Self(1000134004);
	pub const TEXEL_BUFFER_DESCRIPTOR_INFO_EXT: Self = Self(1000135000);
	pub const IMAGE_DESCRIPTOR_INFO_EXT: Self = Self(1000135001);
	pub const RESOURCE_DESCRIPTOR_INFO_EXT: Self = Self(1000135002);
	pub const BIND_HEAP_INFO_EXT: Self = Self(1000135003);
	pub const PUSH_DATA_INFO_EXT: Self = Self(1000135004);
	pub const DESCRIPTOR_SET_AND_BINDING_MAPPING_EXT: Self = Self(1000135005);
	pub const SHADER_DESCRIPTOR_SET_AND_BINDING_MAPPING_INFO_EXT: Self = Self(1000135006);
	pub const OPAQUE_CAPTURE_DATA_CREATE_INFO_EXT: Self = Self(1000135007);
	pub const PHYSICAL_DEVICE_DESCRIPTOR_HEAP_PROPERTIES_EXT: Self = Self(1000135008);
	pub const PHYSICAL_DEVICE_DESCRIPTOR_HEAP_FEATURES_EXT: Self = Self(1000135009);
	pub const COMMAND_BUFFER_INHERITANCE_DESCRIPTOR_HEAP_INFO_EXT: Self = Self(1000135010);
	pub const SAMPLER_CUSTOM_BORDER_COLOR_INDEX_CREATE_INFO_EXT: Self = Self(1000135011);
	pub const INDIRECT_COMMANDS_LAYOUT_PUSH_DATA_TOKEN_NV: Self = Self(1000135012);
	pub const SUBSAMPLED_IMAGE_FORMAT_PROPERTIES_EXT: Self = Self(1000135013);
	pub const PHYSICAL_DEVICE_DESCRIPTOR_HEAP_TENSOR_PROPERTIES_ARM: Self = Self(1000135014);
	pub const ATTACHMENT_SAMPLE_COUNT_INFO_AMD: Self = Self(1000044008);
	pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES_EXT: Self = Self(1000138000);
	pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES_EXT: Self = Self(1000138001);
	pub const WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK_EXT: Self = Self(1000138002);
	pub const DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO_EXT: Self = Self(1000138003);
	pub const PHYSICAL_DEVICE_SHADER_BFLOAT16_FEATURES_KHR: Self = Self(1000141000);
	pub const SAMPLE_LOCATIONS_INFO_EXT: Self = Self(1000143000);
	pub const RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT: Self = Self(1000143001);
	pub const PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT: Self = Self(1000143002);
	pub const PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT: Self = Self(1000143003);
	pub const MULTISAMPLE_PROPERTIES_EXT: Self = Self(1000143004);
	pub const BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR: Self = Self(1000146000);
	pub const IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR: Self = Self(1000146001);
	pub const IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR: Self = Self(1000146002);
	pub const MEMORY_REQUIREMENTS_2_KHR: Self = Self(1000146003);
	pub const SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR: Self = Self(1000146004);
	pub const IMAGE_FORMAT_LIST_CREATE_INFO_KHR: Self = Self(1000147000);
	pub const PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT: Self = Self(1000148000);
	pub const PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT: Self = Self(1000148001);
	pub const PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT: Self = Self(1000148002);
	pub const PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV: Self = Self(1000149000);
	pub const WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR: Self = Self(1000150007);
	pub const ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR: Self = Self(1000150000);
	pub const ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR: Self = Self(1000150002);
	pub const ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR: Self = Self(1000150003);
	pub const ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR: Self = Self(1000150004);
	pub const ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR: Self = Self(1000150005);
	pub const ACCELERATION_STRUCTURE_GEOMETRY_KHR: Self = Self(1000150006);
	pub const ACCELERATION_STRUCTURE_VERSION_INFO_KHR: Self = Self(1000150009);
	pub const COPY_ACCELERATION_STRUCTURE_INFO_KHR: Self = Self(1000150010);
	pub const COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR: Self = Self(1000150011);
	pub const COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR: Self = Self(1000150012);
	pub const PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR: Self = Self(1000150013);
	pub const PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR: Self = Self(1000150014);
	pub const ACCELERATION_STRUCTURE_CREATE_INFO_KHR: Self = Self(1000150017);
	pub const ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR: Self = Self(1000150020);
	pub const PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR: Self = Self(1000347000);
	pub const PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR: Self = Self(1000347001);
	pub const RAY_TRACING_PIPELINE_CREATE_INFO_KHR: Self = Self(1000150015);
	pub const RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR: Self = Self(1000150016);
	pub const RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR: Self = Self(1000150018);
	pub const PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR: Self = Self(1000348013);
	pub const PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV: Self = Self(1000152000);
	pub const ATTACHMENT_SAMPLE_COUNT_INFO_NV: Self = Self(1000044008);
	pub const PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV: Self = Self(1000154000);
	pub const PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV: Self = Self(1000154001);
	pub const SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR: Self = Self(1000156000);
	pub const SAMPLER_YCBCR_CONVERSION_INFO_KHR: Self = Self(1000156001);
	pub const BIND_IMAGE_PLANE_MEMORY_INFO_KHR: Self = Self(1000156002);
	pub const IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR: Self = Self(1000156003);
	pub const PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR: Self = Self(1000156004);
	pub const SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES_KHR: Self = Self(1000156005);
	pub const BIND_BUFFER_MEMORY_INFO_KHR: Self = Self(1000157000);
	pub const BIND_IMAGE_MEMORY_INFO_KHR: Self = Self(1000157001);
	pub const DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT: Self = Self(1000158000);
	pub const PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT: Self = Self(1000158002);
	pub const IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT: Self = Self(1000158003);
	pub const IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT: Self = Self(1000158004);
	pub const IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT: Self = Self(1000158005);
	pub const DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT: Self = Self(1000158006);
	pub const VALIDATION_CACHE_CREATE_INFO_EXT: Self = Self(1000160000);
	pub const SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT: Self = Self(1000160001);
	pub const DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO_EXT: Self = Self(1000161000);
	pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT: Self = Self(1000161001);
	pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES_EXT: Self = Self(1000161002);
	pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO_EXT: Self = Self(1000161003);
	pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT_EXT: Self = Self(1000161004);
	#[cfg(feature = "provisional")]
	pub const PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR: Self = Self(1000163000);
	#[cfg(feature = "provisional")]
	pub const PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR: Self = Self(1000163001);
	pub const PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV: Self = Self(1000164000);
	pub const PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV: Self = Self(1000164001);
	pub const PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV: Self = Self(1000164002);
	pub const PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV: Self = Self(1000164005);
	pub const RAY_TRACING_PIPELINE_CREATE_INFO_NV: Self = Self(1000165000);
	pub const ACCELERATION_STRUCTURE_CREATE_INFO_NV: Self = Self(1000165001);
	pub const GEOMETRY_NV: Self = Self(1000165003);
	pub const GEOMETRY_TRIANGLES_NV: Self = Self(1000165004);
	pub const GEOMETRY_AABB_NV: Self = Self(1000165005);
	pub const BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV: Self = Self(1000165006);
	pub const WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV: Self = Self(1000165007);
	pub const ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV: Self = Self(1000165008);
	pub const PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV: Self = Self(1000165009);
	pub const RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV: Self = Self(1000165011);
	pub const ACCELERATION_STRUCTURE_INFO_NV: Self = Self(1000165012);
	pub const PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV: Self = Self(1000166000);
	pub const PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV: Self = Self(1000166001);
	pub const PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES_KHR: Self = Self(1000168000);
	pub const DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR: Self = Self(1000168001);
	pub const PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT: Self = Self(1000170000);
	pub const FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT: Self = Self(1000170001);
	pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_CONVERSION_FEATURES_QCOM: Self = Self(1000172000);
	pub const PHYSICAL_DEVICE_ELAPSED_TIMER_QUERY_FEATURES_QCOM: Self = Self(1000173000);
	pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT: Self = Self(1000174000);
	pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES_KHR: Self = Self(1000175000);
	pub const PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES_KHR: Self = Self(1000177000);
	pub const IMPORT_MEMORY_HOST_POINTER_INFO_EXT: Self = Self(1000178000);
	pub const MEMORY_HOST_POINTER_PROPERTIES_EXT: Self = Self(1000178001);
	pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT: Self = Self(1000178002);
	pub const PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES_KHR: Self = Self(1000180000);
	pub const PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR: Self = Self(1000181000);
	pub const PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD: Self = Self(1000183000);
	pub const CALIBRATED_TIMESTAMP_INFO_EXT: Self = Self(1000184000);
	pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD: Self = Self(1000185000);
	pub const VIDEO_DECODE_H265_CAPABILITIES_KHR: Self = Self(1000187000);
	pub const VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000187001);
	pub const VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1000187002);
	pub const VIDEO_DECODE_H265_PROFILE_INFO_KHR: Self = Self(1000187003);
	pub const VIDEO_DECODE_H265_PICTURE_INFO_KHR: Self = Self(1000187004);
	pub const VIDEO_DECODE_H265_DPB_SLOT_INFO_KHR: Self = Self(1000187005);
	pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR: Self = Self(1000174000);
	pub const PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR: Self = Self(1000388000);
	pub const QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR: Self = Self(1000388001);
	pub const DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD: Self = Self(1000189000);
	pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT: Self = Self(1000190000);
	pub const PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT: Self = Self(1000190001);
	pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT: Self = Self(1000190002);
	#[cfg(feature = "platform-ggp")]
	pub const PRESENT_FRAME_TOKEN_GGP: Self = Self(1000191000);
	pub const PIPELINE_CREATION_FEEDBACK_CREATE_INFO_EXT: Self = Self(1000192000);
	pub const PHYSICAL_DEVICE_DRIVER_PROPERTIES_KHR: Self = Self(1000196000);
	pub const PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES_KHR: Self = Self(1000197000);
	pub const PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES_KHR: Self = Self(1000199000);
	pub const SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE_KHR: Self = Self(1000199001);
	pub const PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV: Self = Self(1000201000);
	pub const PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV: Self = Self(1000202000);
	pub const PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV: Self = Self(1000202001);
	pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_NV: Self = Self(1000203000);
	pub const PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV: Self = Self(1000204000);
	pub const PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV: Self = Self(1000205000);
	pub const PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV: Self = Self(1000205002);
	pub const CHECKPOINT_DATA_NV: Self = Self(1000206000);
	pub const QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV: Self = Self(1000206001);
	pub const QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV: Self = Self(1000314008);
	pub const CHECKPOINT_DATA_2_NV: Self = Self(1000314009);
	pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES_KHR: Self = Self(1000207000);
	pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES_KHR: Self = Self(1000207001);
	pub const SEMAPHORE_TYPE_CREATE_INFO_KHR: Self = Self(1000207002);
	pub const TIMELINE_SEMAPHORE_SUBMIT_INFO_KHR: Self = Self(1000207003);
	pub const SEMAPHORE_WAIT_INFO_KHR: Self = Self(1000207004);
	pub const SEMAPHORE_SIGNAL_INFO_KHR: Self = Self(1000207005);
	pub const PHYSICAL_DEVICE_PRESENT_TIMING_FEATURES_EXT: Self = Self(1000208000);
	pub const SWAPCHAIN_TIMING_PROPERTIES_EXT: Self = Self(1000208001);
	pub const SWAPCHAIN_TIME_DOMAIN_PROPERTIES_EXT: Self = Self(1000208002);
	pub const PRESENT_TIMINGS_INFO_EXT: Self = Self(1000208003);
	pub const PRESENT_TIMING_INFO_EXT: Self = Self(1000208004);
	pub const PAST_PRESENTATION_TIMING_INFO_EXT: Self = Self(1000208005);
	pub const PAST_PRESENTATION_TIMING_PROPERTIES_EXT: Self = Self(1000208006);
	pub const PAST_PRESENTATION_TIMING_EXT: Self = Self(1000208007);
	pub const PRESENT_TIMING_SURFACE_CAPABILITIES_EXT: Self = Self(1000208008);
	pub const SWAPCHAIN_CALIBRATED_TIMESTAMP_INFO_EXT: Self = Self(1000208009);
	pub const PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL: Self = Self(1000209000);
	pub const QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL: Self = Self(1000210000);
	pub const QUERY_POOL_CREATE_INFO_INTEL: Self = Self(1000210000);
	pub const INITIALIZE_PERFORMANCE_API_INFO_INTEL: Self = Self(1000210001);
	pub const PERFORMANCE_MARKER_INFO_INTEL: Self = Self(1000210002);
	pub const PERFORMANCE_STREAM_MARKER_INFO_INTEL: Self = Self(1000210003);
	pub const PERFORMANCE_OVERRIDE_INFO_INTEL: Self = Self(1000210004);
	pub const PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL: Self = Self(1000210005);
	pub const PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES_KHR: Self = Self(1000211000);
	pub const PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT: Self = Self(1000212000);
	pub const DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD: Self = Self(1000213000);
	pub const SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD: Self = Self(1000213001);
	#[cfg(feature = "platform-fuchsia")]
	pub const IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA: Self = Self(1000214000);
	pub const PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES_KHR: Self = Self(1000215000);
	#[cfg(feature = "platform-metal")]
	pub const METAL_SURFACE_CREATE_INFO_EXT: Self = Self(1000217000);
	pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT: Self = Self(1000218000);
	pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT: Self = Self(1000218001);
	pub const RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT: Self = Self(1000218002);
	pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT: Self = Self(1000044007);
	pub const PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES_EXT: Self = Self(1000221000);
	pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES_EXT: Self = Self(1000225000);
	pub const PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT: Self = Self(1000225001);
	pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES_EXT: Self = Self(1000225002);
	pub const FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR: Self = Self(1000226000);
	pub const PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR: Self = Self(1000226001);
	pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR: Self = Self(1000226002);
	pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR: Self = Self(1000226003);
	pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR: Self = Self(1000226004);
	pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR: Self = Self(1000044006);
	pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD: Self = Self(1000227000);
	pub const PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD: Self = Self(1000229000);
	pub const PHYSICAL_DEVICE_SHADER_CONSTANT_DATA_FEATURES_KHR: Self = Self(1000231000);
	pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES_KHR: Self = Self(1000232000);
	pub const RENDERING_ATTACHMENT_LOCATION_INFO_KHR: Self = Self(1000232001);
	pub const RENDERING_INPUT_ATTACHMENT_INDEX_INFO_KHR: Self = Self(1000232002);
	pub const PHYSICAL_DEVICE_SHADER_ABORT_FEATURES_KHR: Self = Self(1000233000);
	pub const DEVICE_FAULT_SHADER_ABORT_MESSAGE_INFO_KHR: Self = Self(1000233001);
	pub const PHYSICAL_DEVICE_SHADER_ABORT_PROPERTIES_KHR: Self = Self(1000233002);
	pub const PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT: Self = Self(1000234000);
	pub const PHYSICAL_DEVICE_SHADER_QUAD_CONTROL_FEATURES_KHR: Self = Self(1000235000);
	pub const PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT: Self = Self(1000237000);
	pub const PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT: Self = Self(1000238000);
	pub const MEMORY_PRIORITY_ALLOCATE_INFO_EXT: Self = Self(1000238001);
	pub const SURFACE_PROTECTED_CAPABILITIES_KHR: Self = Self(1000239000);
	pub const PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV: Self =
		Self(1000240000);
	pub const PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES_KHR: Self = Self(1000241000);
	pub const ATTACHMENT_REFERENCE_STENCIL_LAYOUT_KHR: Self = Self(1000241001);
	pub const ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT_KHR: Self = Self(1000241002);
	pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT: Self = Self(1000244000);
	pub const PHYSICAL_DEVICE_BUFFER_ADDRESS_FEATURES_EXT: Self = Self(1000244000);
	pub const BUFFER_DEVICE_ADDRESS_INFO_EXT: Self = Self(1000244001);
	pub const BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT: Self = Self(1000244002);
	pub const PHYSICAL_DEVICE_TOOL_PROPERTIES_EXT: Self = Self(1000245000);
	pub const IMAGE_STENCIL_USAGE_CREATE_INFO_EXT: Self = Self(1000246000);
	pub const VALIDATION_FEATURES_EXT: Self = Self(1000247000);
	pub const PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR: Self = Self(1000248000);
	pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV: Self = Self(1000249000);
	pub const COOPERATIVE_MATRIX_PROPERTIES_NV: Self = Self(1000249001);
	pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV: Self = Self(1000249002);
	pub const PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV: Self = Self(1000250000);
	pub const PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV: Self = Self(1000250001);
	pub const FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV: Self = Self(1000250002);
	pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT: Self = Self(1000251000);
	pub const PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT: Self = Self(1000252000);
	pub const PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES_KHR: Self = Self(1000253000);
	pub const PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT: Self = Self(1000254000);
	pub const PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT: Self = Self(1000254001);
	pub const PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT: Self = Self(1000254002);
	#[cfg(feature = "platform-win32")]
	pub const SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT: Self = Self(1000255000);
	#[cfg(feature = "platform-win32")]
	pub const SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT: Self = Self(1000255002);
	#[cfg(feature = "platform-win32")]
	pub const SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT: Self = Self(1000255001);
	pub const HEADLESS_SURFACE_CREATE_INFO_EXT: Self = Self(1000256000);
	pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_KHR: Self = Self(1000257000);
	pub const BUFFER_DEVICE_ADDRESS_INFO_KHR: Self = Self(1000244001);
	pub const BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO_KHR: Self = Self(1000257002);
	pub const MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO_KHR: Self = Self(1000257003);
	pub const DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO_KHR: Self = Self(1000257004);
	pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT: Self = Self(1000259000);
	pub const PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT: Self = Self(1000259001);
	pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT: Self = Self(1000259002);
	pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT: Self = Self(1000260000);
	pub const PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES_EXT: Self = Self(1000261000);
	pub const PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT: Self = Self(1000265000);
	pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT: Self = Self(1000267000);
	pub const PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR: Self = Self(1000269000);
	pub const PIPELINE_INFO_KHR: Self = Self(1000269001);
	pub const PIPELINE_EXECUTABLE_PROPERTIES_KHR: Self = Self(1000269002);
	pub const PIPELINE_EXECUTABLE_INFO_KHR: Self = Self(1000269003);
	pub const PIPELINE_EXECUTABLE_STATISTIC_KHR: Self = Self(1000269004);
	pub const PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR: Self = Self(1000269005);
	pub const PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES_EXT: Self = Self(1000270000);
	pub const PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES_EXT: Self = Self(1000270001);
	pub const MEMORY_TO_IMAGE_COPY_EXT: Self = Self(1000270002);
	pub const IMAGE_TO_MEMORY_COPY_EXT: Self = Self(1000270003);
	pub const COPY_IMAGE_TO_MEMORY_INFO_EXT: Self = Self(1000270004);
	pub const COPY_MEMORY_TO_IMAGE_INFO_EXT: Self = Self(1000270005);
	pub const HOST_IMAGE_LAYOUT_TRANSITION_INFO_EXT: Self = Self(1000270006);
	pub const COPY_IMAGE_TO_IMAGE_INFO_EXT: Self = Self(1000270007);
	pub const SUBRESOURCE_HOST_MEMCPY_SIZE_EXT: Self = Self(1000270008);
	pub const HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY_EXT: Self = Self(1000270009);
	pub const MEMORY_MAP_INFO_KHR: Self = Self(1000271000);
	pub const MEMORY_UNMAP_INFO_KHR: Self = Self(1000271001);
	pub const PHYSICAL_DEVICE_MAP_MEMORY_PLACED_FEATURES_EXT: Self = Self(1000272000);
	pub const PHYSICAL_DEVICE_MAP_MEMORY_PLACED_PROPERTIES_EXT: Self = Self(1000272001);
	pub const MEMORY_MAP_PLACED_INFO_EXT: Self = Self(1000272002);
	pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT: Self = Self(1000273000);
	pub const SURFACE_PRESENT_MODE_EXT: Self = Self(1000274000);
	pub const SURFACE_PRESENT_SCALING_CAPABILITIES_EXT: Self = Self(1000274001);
	pub const SURFACE_PRESENT_MODE_COMPATIBILITY_EXT: Self = Self(1000274002);
	pub const PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_EXT: Self = Self(1000275000);
	pub const SWAPCHAIN_PRESENT_FENCE_INFO_EXT: Self = Self(1000275001);
	pub const SWAPCHAIN_PRESENT_MODES_CREATE_INFO_EXT: Self = Self(1000275002);
	pub const SWAPCHAIN_PRESENT_MODE_INFO_EXT: Self = Self(1000275003);
	pub const SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_EXT: Self = Self(1000275004);
	pub const RELEASE_SWAPCHAIN_IMAGES_INFO_EXT: Self = Self(1000275005);
	pub const PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES_EXT: Self =
		Self(1000276000);
	pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV: Self = Self(1000277000);
	pub const GRAPHICS_SHADER_GROUP_CREATE_INFO_NV: Self = Self(1000277001);
	pub const GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV: Self = Self(1000277002);
	pub const INDIRECT_COMMANDS_LAYOUT_TOKEN_NV: Self = Self(1000277003);
	pub const INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV: Self = Self(1000277004);
	pub const GENERATED_COMMANDS_INFO_NV: Self = Self(1000277005);
	pub const GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV: Self = Self(1000277006);
	pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV: Self = Self(1000277007);
	pub const PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV: Self = Self(1000278000);
	pub const COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV: Self = Self(1000278001);
	pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES_KHR: Self = Self(1000280000);
	pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES_KHR: Self = Self(1000280001);
	pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT: Self = Self(1000281000);
	pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES_EXT: Self = Self(1000281001);
	pub const COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM: Self = Self(1000282000);
	pub const RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM: Self = Self(1000282001);
	pub const PHYSICAL_DEVICE_DEPTH_BIAS_CONTROL_FEATURES_EXT: Self = Self(1000283000);
	pub const DEPTH_BIAS_INFO_EXT: Self = Self(1000283001);
	pub const DEPTH_BIAS_REPRESENTATION_INFO_EXT: Self = Self(1000283002);
	pub const PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT: Self = Self(1000284000);
	pub const DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT: Self = Self(1000284001);
	pub const DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT: Self = Self(1000284002);
	pub const PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT: Self = Self(1000286000);
	pub const PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT: Self = Self(1000286001);
	pub const SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT: Self = Self(1000287000);
	pub const PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT: Self = Self(1000287001);
	pub const PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT: Self = Self(1000287002);
	pub const PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_3D_FEATURES_EXT: Self = Self(1000288000);
	pub const PIPELINE_LIBRARY_CREATE_INFO_KHR: Self = Self(1000290000);
	pub const PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV: Self = Self(1000292000);
	pub const SURFACE_CAPABILITIES_PRESENT_BARRIER_NV: Self = Self(1000292001);
	pub const SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV: Self = Self(1000292002);
	pub const PRESENT_ID_KHR: Self = Self(1000294000);
	pub const PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR: Self = Self(1000294001);
	pub const PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES_EXT: Self = Self(1000295000);
	pub const DEVICE_PRIVATE_DATA_CREATE_INFO_EXT: Self = Self(1000295001);
	pub const PRIVATE_DATA_SLOT_CREATE_INFO_EXT: Self = Self(1000295002);
	pub const PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES_EXT: Self = Self(1000297000);
	pub const VIDEO_ENCODE_INFO_KHR: Self = Self(1000299000);
	pub const VIDEO_ENCODE_RATE_CONTROL_INFO_KHR: Self = Self(1000299001);
	pub const VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1000299002);
	pub const VIDEO_ENCODE_CAPABILITIES_KHR: Self = Self(1000299003);
	pub const VIDEO_ENCODE_USAGE_INFO_KHR: Self = Self(1000299004);
	pub const QUERY_POOL_VIDEO_ENCODE_FEEDBACK_CREATE_INFO_KHR: Self = Self(1000299005);
	pub const PHYSICAL_DEVICE_VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR: Self = Self(1000299006);
	pub const VIDEO_ENCODE_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1000299007);
	pub const VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR: Self = Self(1000299008);
	pub const VIDEO_ENCODE_SESSION_PARAMETERS_GET_INFO_KHR: Self = Self(1000299009);
	pub const VIDEO_ENCODE_SESSION_PARAMETERS_FEEDBACK_INFO_KHR: Self = Self(1000299010);
	pub const PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV: Self = Self(1000300000);
	pub const DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV: Self = Self(1000300001);
	pub const PERF_HINT_INFO_QCOM: Self = Self(1000302000);
	pub const PHYSICAL_DEVICE_QUEUE_PERF_HINT_FEATURES_QCOM: Self = Self(1000302001);
	pub const PHYSICAL_DEVICE_QUEUE_PERF_HINT_PROPERTIES_QCOM: Self = Self(1000302002);
	pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_3_FEATURES_QCOM: Self = Self(1000303000);
	pub const PHYSICAL_DEVICE_SHADER_MULTIPLE_WAIT_QUEUES_FEATURES_QCOM: Self = Self(1000304000);
	pub const PHYSICAL_DEVICE_SHADER_MULTIPLE_WAIT_QUEUES_PROPERTIES_QCOM: Self = Self(1000304001);
	pub const PHYSICAL_DEVICE_SHADER_SPLIT_BARRIER_FEATURES_EXT: Self = Self(1000305000);
	pub const PHYSICAL_DEVICE_SHADER_SPLIT_BARRIER_PROPERTIES_EXT: Self = Self(1000305001);
	#[cfg(feature = "provisional")]
	pub const CUDA_MODULE_CREATE_INFO_NV: Self = Self(1000307000);
	#[cfg(feature = "provisional")]
	pub const CUDA_FUNCTION_CREATE_INFO_NV: Self = Self(1000307001);
	#[cfg(feature = "provisional")]
	pub const CUDA_LAUNCH_INFO_NV: Self = Self(1000307002);
	#[cfg(feature = "provisional")]
	pub const PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_FEATURES_NV: Self = Self(1000307003);
	#[cfg(feature = "provisional")]
	pub const PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_PROPERTIES_NV: Self = Self(1000307004);
	pub const PHYSICAL_DEVICE_TILE_SHADING_FEATURES_QCOM: Self = Self(1000309000);
	pub const PHYSICAL_DEVICE_TILE_SHADING_PROPERTIES_QCOM: Self = Self(1000309001);
	pub const RENDER_PASS_TILE_SHADING_CREATE_INFO_QCOM: Self = Self(1000309002);
	pub const PER_TILE_BEGIN_INFO_QCOM: Self = Self(1000309003);
	pub const PER_TILE_END_INFO_QCOM: Self = Self(1000309004);
	pub const DISPATCH_TILE_INFO_QCOM: Self = Self(1000309005);
	pub const QUERY_LOW_LATENCY_SUPPORT_NV: Self = Self(1000310000);
	#[cfg(feature = "platform-metal")]
	pub const EXPORT_METAL_OBJECT_CREATE_INFO_EXT: Self = Self(1000311000);
	#[cfg(feature = "platform-metal")]
	pub const EXPORT_METAL_OBJECTS_INFO_EXT: Self = Self(1000311001);
	#[cfg(feature = "platform-metal")]
	pub const EXPORT_METAL_DEVICE_INFO_EXT: Self = Self(1000311002);
	#[cfg(feature = "platform-metal")]
	pub const EXPORT_METAL_COMMAND_QUEUE_INFO_EXT: Self = Self(1000311003);
	#[cfg(feature = "platform-metal")]
	pub const EXPORT_METAL_BUFFER_INFO_EXT: Self = Self(1000311004);
	#[cfg(feature = "platform-metal")]
	pub const IMPORT_METAL_BUFFER_INFO_EXT: Self = Self(1000311005);
	#[cfg(feature = "platform-metal")]
	pub const EXPORT_METAL_TEXTURE_INFO_EXT: Self = Self(1000311006);
	#[cfg(feature = "platform-metal")]
	pub const IMPORT_METAL_TEXTURE_INFO_EXT: Self = Self(1000311007);
	#[cfg(feature = "platform-metal")]
	pub const EXPORT_METAL_IO_SURFACE_INFO_EXT: Self = Self(1000311008);
	#[cfg(feature = "platform-metal")]
	pub const IMPORT_METAL_IO_SURFACE_INFO_EXT: Self = Self(1000311009);
	#[cfg(feature = "platform-metal")]
	pub const EXPORT_METAL_SHARED_EVENT_INFO_EXT: Self = Self(1000311010);
	#[cfg(feature = "platform-metal")]
	pub const IMPORT_METAL_SHARED_EVENT_INFO_EXT: Self = Self(1000311011);
	pub const MEMORY_BARRIER_2_KHR: Self = Self(1000314000);
	pub const BUFFER_MEMORY_BARRIER_2_KHR: Self = Self(1000314001);
	pub const IMAGE_MEMORY_BARRIER_2_KHR: Self = Self(1000314002);
	pub const DEPENDENCY_INFO_KHR: Self = Self(1000314003);
	pub const SUBMIT_INFO_2_KHR: Self = Self(1000314004);
	pub const SEMAPHORE_SUBMIT_INFO_KHR: Self = Self(1000314005);
	pub const COMMAND_BUFFER_SUBMIT_INFO_KHR: Self = Self(1000314006);
	pub const PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR: Self = Self(1000314007);
	pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT: Self = Self(1000316000);
	pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT: Self = Self(1000316001);
	pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT: Self = Self(1000316002);
	pub const DESCRIPTOR_ADDRESS_INFO_EXT: Self = Self(1000316003);
	pub const DESCRIPTOR_GET_INFO_EXT: Self = Self(1000316004);
	pub const BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316005);
	pub const IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316006);
	pub const IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316007);
	pub const SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316008);
	pub const OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT: Self = Self(1000316010);
	pub const DESCRIPTOR_BUFFER_BINDING_INFO_EXT: Self = Self(1000316011);
	pub const DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT: Self = Self(1000316012);
	pub const ACCELERATION_STRUCTURE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316009);
	pub const DEVICE_MEMORY_COPY_KHR: Self = Self(1000318000);
	pub const COPY_DEVICE_MEMORY_INFO_KHR: Self = Self(1000318001);
	pub const DEVICE_MEMORY_IMAGE_COPY_KHR: Self = Self(1000318002);
	pub const COPY_DEVICE_MEMORY_IMAGE_INFO_KHR: Self = Self(1000318003);
	pub const MEMORY_RANGE_BARRIERS_INFO_KHR: Self = Self(1000318004);
	pub const MEMORY_RANGE_BARRIER_KHR: Self = Self(1000318005);
	pub const PHYSICAL_DEVICE_DEVICE_ADDRESS_COMMANDS_FEATURES_KHR: Self = Self(1000318006);
	pub const BIND_INDEX_BUFFER_3_INFO_KHR: Self = Self(1000318007);
	pub const BIND_VERTEX_BUFFER_3_INFO_KHR: Self = Self(1000318008);
	pub const DRAW_INDIRECT_2_INFO_KHR: Self = Self(1000318009);
	pub const DRAW_INDIRECT_COUNT_2_INFO_KHR: Self = Self(1000318010);
	pub const DISPATCH_INDIRECT_2_INFO_KHR: Self = Self(1000318011);
	pub const CONDITIONAL_RENDERING_BEGIN_INFO_2_EXT: Self = Self(1000318012);
	pub const BIND_TRANSFORM_FEEDBACK_BUFFER_2_INFO_EXT: Self = Self(1000318013);
	pub const MEMORY_MARKER_INFO_AMD: Self = Self(1000318014);
	pub const ACCELERATION_STRUCTURE_CREATE_INFO_2_KHR: Self = Self(1000318015);
	pub const PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT: Self = Self(1000320000);
	pub const PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT: Self = Self(1000320001);
	pub const GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT: Self = Self(1000320002);
	pub const PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD: Self =
		Self(1000321000);
	pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR: Self = Self(1000203000);
	pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR: Self = Self(1000322000);
	pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR: Self =
		Self(1000323000);
	pub const PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES_KHR: Self = Self(1000325000);
	pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV: Self = Self(1000326000);
	pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV: Self = Self(1000326001);
	pub const PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV: Self = Self(1000326002);
	pub const ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV: Self = Self(1000327000);
	pub const PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV: Self = Self(1000327001);
	pub const ACCELERATION_STRUCTURE_MOTION_INFO_NV: Self = Self(1000327002);
	pub const PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT: Self = Self(1000328000);
	pub const PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT: Self = Self(1000328001);
	pub const PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT: Self = Self(1000330000);
	pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT: Self = Self(1000332000);
	pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT: Self = Self(1000332001);
	pub const COPY_COMMAND_TRANSFORM_INFO_QCOM: Self = Self(1000333000);
	pub const PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES_EXT: Self = Self(1000335000);
	pub const PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR: Self = Self(1000336000);
	pub const COPY_BUFFER_INFO_2_KHR: Self = Self(1000337000);
	pub const COPY_IMAGE_INFO_2_KHR: Self = Self(1000337001);
	pub const COPY_BUFFER_TO_IMAGE_INFO_2_KHR: Self = Self(1000337002);
	pub const COPY_IMAGE_TO_BUFFER_INFO_2_KHR: Self = Self(1000337003);
	pub const BLIT_IMAGE_INFO_2_KHR: Self = Self(1000337004);
	pub const RESOLVE_IMAGE_INFO_2_KHR: Self = Self(1000337005);
	pub const BUFFER_COPY_2_KHR: Self = Self(1000337006);
	pub const IMAGE_COPY_2_KHR: Self = Self(1000337007);
	pub const IMAGE_BLIT_2_KHR: Self = Self(1000337008);
	pub const BUFFER_IMAGE_COPY_2_KHR: Self = Self(1000337009);
	pub const IMAGE_RESOLVE_2_KHR: Self = Self(1000337010);
	pub const PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT: Self = Self(1000338000);
	pub const IMAGE_COMPRESSION_CONTROL_EXT: Self = Self(1000338001);
	pub const SUBRESOURCE_LAYOUT_2_EXT: Self = Self(1000338002);
	pub const IMAGE_SUBRESOURCE_2_EXT: Self = Self(1000338003);
	pub const IMAGE_COMPRESSION_PROPERTIES_EXT: Self = Self(1000338004);
	pub const PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT: Self = Self(1000339000);
	pub const PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT: Self = Self(1000340000);
	pub const PHYSICAL_DEVICE_FAULT_FEATURES_EXT: Self = Self(1000341000);
	pub const DEVICE_FAULT_COUNTS_EXT: Self = Self(1000341001);
	pub const DEVICE_FAULT_INFO_EXT: Self = Self(1000341002);
	pub const PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_ARM: Self =
		Self(1000342000);
	pub const PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT: Self = Self(1000344000);
	#[cfg(feature = "platform-directfb")]
	pub const DIRECTFB_SURFACE_CREATE_INFO_EXT: Self = Self(1000346000);
	pub const PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_VALVE: Self = Self(1000351000);
	pub const MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_VALVE: Self = Self(1000351002);
	pub const PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT: Self = Self(1000352000);
	pub const VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT: Self = Self(1000352001);
	pub const VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT: Self = Self(1000352002);
	pub const PHYSICAL_DEVICE_DRM_PROPERTIES_EXT: Self = Self(1000353000);
	pub const PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT: Self = Self(1000354000);
	pub const DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT: Self = Self(1000354001);
	pub const PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT: Self = Self(1000355000);
	pub const PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT: Self = Self(1000355001);
	pub const PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT: Self = Self(1000356000);
	pub const FORMAT_PROPERTIES_3_KHR: Self = Self(1000360000);
	pub const PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_EXT: Self = Self(1000361000);
	#[cfg(feature = "platform-fuchsia")]
	pub const IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000364000);
	#[cfg(feature = "platform-fuchsia")]
	pub const MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA: Self = Self(1000364001);
	#[cfg(feature = "platform-fuchsia")]
	pub const MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000364002);
	#[cfg(feature = "platform-fuchsia")]
	pub const IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000365000);
	#[cfg(feature = "platform-fuchsia")]
	pub const SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000365001);
	#[cfg(feature = "platform-fuchsia")]
	pub const BUFFER_COLLECTION_CREATE_INFO_FUCHSIA: Self = Self(1000366000);
	#[cfg(feature = "platform-fuchsia")]
	pub const IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA: Self = Self(1000366001);
	#[cfg(feature = "platform-fuchsia")]
	pub const BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA: Self = Self(1000366002);
	#[cfg(feature = "platform-fuchsia")]
	pub const BUFFER_COLLECTION_PROPERTIES_FUCHSIA: Self = Self(1000366003);
	#[cfg(feature = "platform-fuchsia")]
	pub const BUFFER_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366004);
	#[cfg(feature = "platform-fuchsia")]
	pub const BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA: Self = Self(1000366005);
	#[cfg(feature = "platform-fuchsia")]
	pub const IMAGE_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366006);
	#[cfg(feature = "platform-fuchsia")]
	pub const IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366007);
	#[cfg(feature = "platform-fuchsia")]
	pub const SYSMEM_COLOR_SPACE_FUCHSIA: Self = Self(1000366008);
	#[cfg(feature = "platform-fuchsia")]
	pub const BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366009);
	pub const SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI: Self = Self(1000369000);
	pub const PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI: Self = Self(1000369001);
	pub const PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI: Self = Self(1000369002);
	pub const PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI: Self = Self(1000370000);
	pub const MEMORY_GET_REMOTE_ADDRESS_INFO_NV: Self = Self(1000371000);
	pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV: Self = Self(1000371001);
	pub const PIPELINE_PROPERTIES_IDENTIFIER_EXT: Self = Self(1000372000);
	pub const PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT: Self = Self(1000372001);
	pub const PIPELINE_INFO_EXT: Self = Self(1000269001);
	pub const PHYSICAL_DEVICE_FRAME_BOUNDARY_FEATURES_EXT: Self = Self(1000375000);
	pub const FRAME_BOUNDARY_EXT: Self = Self(1000375001);
	pub const PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT: Self =
		Self(1000376000);
	pub const SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT: Self = Self(1000376001);
	pub const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT: Self = Self(1000376002);
	pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT: Self = Self(1000377000);
	#[cfg(feature = "platform-screen")]
	pub const SCREEN_SURFACE_CREATE_INFO_QNX: Self = Self(1000378000);
	pub const PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT: Self = Self(1000381000);
	pub const PIPELINE_COLOR_WRITE_CREATE_INFO_EXT: Self = Self(1000381001);
	pub const PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT: Self = Self(1000382000);
	pub const PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR: Self = Self(1000386000);
	pub const PHYSICAL_DEVICE_SHADER_UNTYPED_POINTERS_FEATURES_KHR: Self = Self(1000387000);
	pub const PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_EXT: Self = Self(1000388000);
	pub const QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_EXT: Self = Self(1000388001);
	pub const PHYSICAL_DEVICE_VIDEO_ENCODE_RGB_CONVERSION_FEATURES_VALVE: Self = Self(1000390000);
	pub const VIDEO_ENCODE_RGB_CONVERSION_CAPABILITIES_VALVE: Self = Self(1000390001);
	pub const VIDEO_ENCODE_PROFILE_RGB_CONVERSION_INFO_VALVE: Self = Self(1000390002);
	pub const VIDEO_ENCODE_SESSION_RGB_CONVERSION_CREATE_INFO_VALVE: Self = Self(1000390003);
	pub const PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT: Self = Self(1000391000);
	pub const IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT: Self = Self(1000391001);
	pub const PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT: Self = Self(1000392000);
	pub const PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT: Self = Self(1000392001);
	pub const PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT: Self = Self(1000393000);
	pub const PHYSICAL_DEVICE_SHADER_TILE_IMAGE_FEATURES_EXT: Self = Self(1000395000);
	pub const PHYSICAL_DEVICE_SHADER_TILE_IMAGE_PROPERTIES_EXT: Self = Self(1000395001);
	pub const MICROMAP_BUILD_INFO_EXT: Self = Self(1000396000);
	pub const MICROMAP_VERSION_INFO_EXT: Self = Self(1000396001);
	pub const COPY_MICROMAP_INFO_EXT: Self = Self(1000396002);
	pub const COPY_MICROMAP_TO_MEMORY_INFO_EXT: Self = Self(1000396003);
	pub const COPY_MEMORY_TO_MICROMAP_INFO_EXT: Self = Self(1000396004);
	pub const PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT: Self = Self(1000396005);
	pub const PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT: Self = Self(1000396006);
	pub const MICROMAP_CREATE_INFO_EXT: Self = Self(1000396007);
	pub const MICROMAP_BUILD_SIZES_INFO_EXT: Self = Self(1000396008);
	pub const ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_EXT: Self = Self(1000396009);
	#[cfg(feature = "provisional")]
	pub const PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_FEATURES_NV: Self = Self(1000397000);
	#[cfg(feature = "provisional")]
	pub const PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_PROPERTIES_NV: Self = Self(1000397001);
	#[cfg(feature = "provisional")]
	pub const ACCELERATION_STRUCTURE_TRIANGLES_DISPLACEMENT_MICROMAP_NV: Self = Self(1000397002);
	pub const PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_FEATURES_HUAWEI: Self = Self(1000404000);
	pub const PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_PROPERTIES_HUAWEI: Self = Self(1000404001);
	pub const PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_VRS_FEATURES_HUAWEI: Self = Self(1000404002);
	pub const PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT: Self = Self(1000411000);
	pub const SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT: Self = Self(1000411001);
	pub const PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT: Self = Self(1000412000);
	pub const PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES_KHR: Self = Self(1000413000);
	pub const PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES_KHR: Self = Self(1000413001);
	pub const DEVICE_BUFFER_MEMORY_REQUIREMENTS_KHR: Self = Self(1000413002);
	pub const DEVICE_IMAGE_MEMORY_REQUIREMENTS_KHR: Self = Self(1000413003);
	pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_ARM: Self = Self(1000415000);
	pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES_KHR: Self = Self(1000416000);
	pub const DEVICE_QUEUE_SHADER_CORE_CONTROL_CREATE_INFO_ARM: Self = Self(1000417000);
	pub const PHYSICAL_DEVICE_SCHEDULING_CONTROLS_FEATURES_ARM: Self = Self(1000417001);
	pub const PHYSICAL_DEVICE_SCHEDULING_CONTROLS_PROPERTIES_ARM: Self = Self(1000417002);
	pub const DISPATCH_PARAMETERS_ARM: Self = Self(1000417003);
	pub const PHYSICAL_DEVICE_SCHEDULING_CONTROLS_DISPATCH_PARAMETERS_PROPERTIES_ARM: Self =
		Self(1000417004);
	pub const PHYSICAL_DEVICE_IMAGE_SLICED_VIEW_OF_3D_FEATURES_EXT: Self = Self(1000418000);
	pub const IMAGE_VIEW_SLICED_CREATE_INFO_EXT: Self = Self(1000418001);
	pub const PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE: Self = Self(1000420000);
	pub const DESCRIPTOR_SET_BINDING_REFERENCE_VALVE: Self = Self(1000420001);
	pub const DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE: Self = Self(1000420002);
	pub const PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_EXT: Self = Self(1000421000);
	pub const PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT: Self = Self(1000422000);
	pub const PHYSICAL_DEVICE_RENDER_PASS_STRIPED_FEATURES_ARM: Self = Self(1000424000);
	pub const PHYSICAL_DEVICE_RENDER_PASS_STRIPED_PROPERTIES_ARM: Self = Self(1000424001);
	pub const RENDER_PASS_STRIPE_BEGIN_INFO_ARM: Self = Self(1000424002);
	pub const RENDER_PASS_STRIPE_INFO_ARM: Self = Self(1000424003);
	pub const RENDER_PASS_STRIPE_SUBMIT_INFO_ARM: Self = Self(1000424004);
	pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM: Self = Self(1000425000);
	pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM: Self = Self(1000425001);
	pub const SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM: Self = Self(1000425002);
	pub const PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_NV: Self = Self(1000426000);
	pub const PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_NV: Self = Self(1000426001);
	pub const PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_NV: Self = Self(1000427000);
	pub const PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_NV: Self = Self(1000427001);
	pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_COMPUTE_FEATURES_NV: Self = Self(1000428000);
	pub const COMPUTE_PIPELINE_INDIRECT_BUFFER_INFO_NV: Self = Self(1000428001);
	pub const PIPELINE_INDIRECT_DEVICE_ADDRESS_INFO_NV: Self = Self(1000428002);
	pub const PHYSICAL_DEVICE_RAY_TRACING_LINEAR_SWEPT_SPHERES_FEATURES_NV: Self = Self(1000429008);
	pub const ACCELERATION_STRUCTURE_GEOMETRY_LINEAR_SWEPT_SPHERES_DATA_NV: Self = Self(1000429009);
	pub const ACCELERATION_STRUCTURE_GEOMETRY_SPHERES_DATA_NV: Self = Self(1000429010);
	pub const PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV: Self = Self(1000430000);
	pub const PHYSICAL_DEVICE_SHADER_MAXIMAL_RECONVERGENCE_FEATURES_KHR: Self = Self(1000434000);
	pub const PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT: Self =
		Self(1000437000);
	pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM: Self = Self(1000440000);
	pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM: Self = Self(1000440001);
	pub const IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM: Self = Self(1000440002);
	pub const PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_FEATURES_EXT: Self = Self(1000451000);
	pub const PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_PROPERTIES_EXT: Self = Self(1000451001);
	#[cfg(feature = "platform-ohos")]
	pub const NATIVE_BUFFER_USAGE_OHOS: Self = Self(1000452000);
	#[cfg(feature = "platform-ohos")]
	pub const NATIVE_BUFFER_PROPERTIES_OHOS: Self = Self(1000452001);
	#[cfg(feature = "platform-ohos")]
	pub const NATIVE_BUFFER_FORMAT_PROPERTIES_OHOS: Self = Self(1000452002);
	#[cfg(feature = "platform-ohos")]
	pub const IMPORT_NATIVE_BUFFER_INFO_OHOS: Self = Self(1000452003);
	#[cfg(feature = "platform-ohos")]
	pub const MEMORY_GET_NATIVE_BUFFER_INFO_OHOS: Self = Self(1000452004);
	#[cfg(feature = "platform-ohos")]
	pub const EXTERNAL_FORMAT_OHOS: Self = Self(1000452005);
	pub const EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_EXT: Self = Self(1000453000);
	pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT: Self = Self(1000455000);
	pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_PROPERTIES_EXT: Self = Self(1000455001);
	pub const PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT: Self = Self(1000458000);
	pub const RENDER_PASS_CREATION_CONTROL_EXT: Self = Self(1000458001);
	pub const RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT: Self = Self(1000458002);
	pub const RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT: Self = Self(1000458003);
	pub const DIRECT_DRIVER_LOADING_INFO_LUNARG: Self = Self(1000459000);
	pub const DIRECT_DRIVER_LOADING_LIST_LUNARG: Self = Self(1000459001);
	pub const TENSOR_CREATE_INFO_ARM: Self = Self(1000460000);
	pub const TENSOR_VIEW_CREATE_INFO_ARM: Self = Self(1000460001);
	pub const BIND_TENSOR_MEMORY_INFO_ARM: Self = Self(1000460002);
	pub const WRITE_DESCRIPTOR_SET_TENSOR_ARM: Self = Self(1000460003);
	pub const PHYSICAL_DEVICE_TENSOR_PROPERTIES_ARM: Self = Self(1000460004);
	pub const TENSOR_FORMAT_PROPERTIES_ARM: Self = Self(1000460005);
	pub const TENSOR_DESCRIPTION_ARM: Self = Self(1000460006);
	pub const TENSOR_MEMORY_REQUIREMENTS_INFO_ARM: Self = Self(1000460007);
	pub const TENSOR_MEMORY_BARRIER_ARM: Self = Self(1000460008);
	pub const PHYSICAL_DEVICE_TENSOR_FEATURES_ARM: Self = Self(1000460009);
	pub const DEVICE_TENSOR_MEMORY_REQUIREMENTS_ARM: Self = Self(1000460010);
	pub const COPY_TENSOR_INFO_ARM: Self = Self(1000460011);
	pub const TENSOR_COPY_ARM: Self = Self(1000460012);
	pub const TENSOR_DEPENDENCY_INFO_ARM: Self = Self(1000460013);
	pub const MEMORY_DEDICATED_ALLOCATE_INFO_TENSOR_ARM: Self = Self(1000460014);
	pub const PHYSICAL_DEVICE_EXTERNAL_TENSOR_INFO_ARM: Self = Self(1000460015);
	pub const EXTERNAL_TENSOR_PROPERTIES_ARM: Self = Self(1000460016);
	pub const EXTERNAL_MEMORY_TENSOR_CREATE_INFO_ARM: Self = Self(1000460017);
	pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_FEATURES_ARM: Self = Self(1000460018);
	pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_PROPERTIES_ARM: Self = Self(1000460019);
	pub const DESCRIPTOR_GET_TENSOR_INFO_ARM: Self = Self(1000460020);
	pub const TENSOR_CAPTURE_DESCRIPTOR_DATA_INFO_ARM: Self = Self(1000460021);
	pub const TENSOR_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_ARM: Self = Self(1000460022);
	pub const FRAME_BOUNDARY_TENSORS_ARM: Self = Self(1000460023);
	pub const PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT: Self = Self(1000462000);
	pub const PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT: Self = Self(1000462001);
	pub const PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT: Self = Self(1000462002);
	pub const SHADER_MODULE_IDENTIFIER_EXT: Self = Self(1000462003);
	pub const PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT: Self =
		Self(1000342000);
	pub const PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV: Self = Self(1000464000);
	pub const PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV: Self = Self(1000464001);
	pub const OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV: Self = Self(1000464002);
	pub const OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV: Self = Self(1000464003);
	pub const OPTICAL_FLOW_SESSION_CREATE_INFO_NV: Self = Self(1000464004);
	pub const OPTICAL_FLOW_EXECUTE_INFO_NV: Self = Self(1000464005);
	pub const OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV: Self = Self(1000464010);
	pub const PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT: Self = Self(1000465000);
	pub const PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES_EXT: Self = Self(1000466000);
	#[cfg(feature = "platform-android")]
	pub const PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_FEATURES_ANDROID: Self = Self(1000468000);
	#[cfg(feature = "platform-android")]
	pub const PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_PROPERTIES_ANDROID: Self = Self(1000468001);
	#[cfg(feature = "platform-android")]
	pub const ANDROID_HARDWARE_BUFFER_FORMAT_RESOLVE_PROPERTIES_ANDROID: Self = Self(1000468002);
	pub const PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES_KHR: Self = Self(1000470000);
	pub const PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES_KHR: Self = Self(1000470001);
	pub const RENDERING_AREA_INFO_KHR: Self = Self(1000470003);
	pub const DEVICE_IMAGE_SUBRESOURCE_INFO_KHR: Self = Self(1000470004);
	pub const SUBRESOURCE_LAYOUT_2_KHR: Self = Self(1000338002);
	pub const IMAGE_SUBRESOURCE_2_KHR: Self = Self(1000338003);
	pub const PIPELINE_CREATE_FLAGS_2_CREATE_INFO_KHR: Self = Self(1000470005);
	pub const BUFFER_USAGE_FLAGS_2_CREATE_INFO_KHR: Self = Self(1000470006);
	pub const PHYSICAL_DEVICE_ANTI_LAG_FEATURES_AMD: Self = Self(1000476000);
	pub const ANTI_LAG_DATA_AMD: Self = Self(1000476001);
	pub const ANTI_LAG_PRESENTATION_INFO_AMD: Self = Self(1000476002);
	#[cfg(feature = "provisional")]
	pub const PHYSICAL_DEVICE_DENSE_GEOMETRY_FORMAT_FEATURES_AMDX: Self = Self(1000478000);
	#[cfg(feature = "provisional")]
	pub const ACCELERATION_STRUCTURE_DENSE_GEOMETRY_FORMAT_TRIANGLES_DATA_AMDX: Self =
		Self(1000478001);
	pub const SURFACE_CAPABILITIES_PRESENT_ID_2_KHR: Self = Self(1000479000);
	pub const PRESENT_ID_2_KHR: Self = Self(1000479001);
	pub const PHYSICAL_DEVICE_PRESENT_ID_2_FEATURES_KHR: Self = Self(1000479002);
	pub const SURFACE_CAPABILITIES_PRESENT_WAIT_2_KHR: Self = Self(1000480000);
	pub const PHYSICAL_DEVICE_PRESENT_WAIT_2_FEATURES_KHR: Self = Self(1000480001);
	pub const PRESENT_WAIT_2_INFO_KHR: Self = Self(1000480002);
	pub const PHYSICAL_DEVICE_RAY_TRACING_POSITION_FETCH_FEATURES_KHR: Self = Self(1000481000);
	pub const PHYSICAL_DEVICE_SHADER_OBJECT_FEATURES_EXT: Self = Self(1000482000);
	pub const PHYSICAL_DEVICE_SHADER_OBJECT_PROPERTIES_EXT: Self = Self(1000482001);
	pub const SHADER_CREATE_INFO_EXT: Self = Self(1000482002);
	pub const SHADER_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT: Self = Self(1000225001);
	pub const PHYSICAL_DEVICE_PIPELINE_BINARY_FEATURES_KHR: Self = Self(1000483000);
	pub const PIPELINE_BINARY_CREATE_INFO_KHR: Self = Self(1000483001);
	pub const PIPELINE_BINARY_INFO_KHR: Self = Self(1000483002);
	pub const PIPELINE_BINARY_KEY_KHR: Self = Self(1000483003);
	pub const PHYSICAL_DEVICE_PIPELINE_BINARY_PROPERTIES_KHR: Self = Self(1000483004);
	pub const RELEASE_CAPTURED_PIPELINE_DATA_INFO_KHR: Self = Self(1000483005);
	pub const PIPELINE_BINARY_DATA_INFO_KHR: Self = Self(1000483006);
	pub const PIPELINE_CREATE_INFO_KHR: Self = Self(1000483007);
	pub const DEVICE_PIPELINE_BINARY_INTERNAL_CACHE_CONTROL_KHR: Self = Self(1000483008);
	pub const PIPELINE_BINARY_HANDLES_INFO_KHR: Self = Self(1000483009);
	pub const PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM: Self = Self(1000484000);
	pub const TILE_PROPERTIES_QCOM: Self = Self(1000484001);
	pub const PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC: Self = Self(1000485000);
	pub const AMIGO_PROFILING_SUBMIT_INFO_SEC: Self = Self(1000485001);
	pub const SURFACE_PRESENT_MODE_KHR: Self = Self(1000274000);
	pub const SURFACE_PRESENT_SCALING_CAPABILITIES_KHR: Self = Self(1000274001);
	pub const SURFACE_PRESENT_MODE_COMPATIBILITY_KHR: Self = Self(1000274002);
	pub const PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_KHR: Self = Self(1000275000);
	pub const SWAPCHAIN_PRESENT_FENCE_INFO_KHR: Self = Self(1000275001);
	pub const SWAPCHAIN_PRESENT_MODES_CREATE_INFO_KHR: Self = Self(1000275002);
	pub const SWAPCHAIN_PRESENT_MODE_INFO_KHR: Self = Self(1000275003);
	pub const SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_KHR: Self = Self(1000275004);
	pub const RELEASE_SWAPCHAIN_IMAGES_INFO_KHR: Self = Self(1000275005);
	pub const PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_VIEWPORTS_FEATURES_QCOM: Self = Self(1000488000);
	pub const PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_NV: Self = Self(1000490000);
	pub const PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_NV: Self = Self(1000490001);
	pub const PHYSICAL_DEVICE_COOPERATIVE_VECTOR_FEATURES_NV: Self = Self(1000491000);
	pub const PHYSICAL_DEVICE_COOPERATIVE_VECTOR_PROPERTIES_NV: Self = Self(1000491001);
	pub const COOPERATIVE_VECTOR_PROPERTIES_NV: Self = Self(1000491002);
	pub const CONVERT_COOPERATIVE_VECTOR_MATRIX_INFO_NV: Self = Self(1000491004);
	pub const PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_FEATURES_NV: Self = Self(1000492000);
	pub const PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_PROPERTIES_NV: Self = Self(1000492001);
	pub const PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT: Self = Self(1000351000);
	pub const MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT: Self = Self(1000351002);
	pub const PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_FEATURES_EXT: Self = Self(1000495000);
	pub const PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_PROPERTIES_EXT: Self = Self(1000495001);
	pub const LAYER_SETTINGS_CREATE_INFO_EXT: Self = Self(1000496000);
	pub const PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM: Self = Self(1000497000);
	pub const PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_PROPERTIES_ARM: Self = Self(1000497001);
	pub const PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_EXT: Self = Self(1000498000);
	pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_FEATURES_EXT: Self =
		Self(1000499000);
	pub const PHYSICAL_DEVICE_INTERNALLY_SYNCHRONIZED_QUEUES_FEATURES_KHR: Self = Self(1000504000);
	pub const LATENCY_SLEEP_MODE_INFO_NV: Self = Self(1000505000);
	pub const LATENCY_SLEEP_INFO_NV: Self = Self(1000505001);
	pub const SET_LATENCY_MARKER_INFO_NV: Self = Self(1000505002);
	pub const GET_LATENCY_MARKER_INFO_NV: Self = Self(1000505003);
	pub const LATENCY_TIMINGS_FRAME_REPORT_NV: Self = Self(1000505004);
	pub const LATENCY_SUBMISSION_PRESENT_ID_NV: Self = Self(1000505005);
	pub const OUT_OF_BAND_QUEUE_TYPE_INFO_NV: Self = Self(1000505006);
	pub const SWAPCHAIN_LATENCY_CREATE_INFO_NV: Self = Self(1000505007);
	pub const LATENCY_SURFACE_CAPABILITIES_NV: Self = Self(1000505008);
	pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_KHR: Self = Self(1000506000);
	pub const COOPERATIVE_MATRIX_PROPERTIES_KHR: Self = Self(1000506001);
	pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_KHR: Self = Self(1000506002);
	pub const DATA_GRAPH_PIPELINE_CREATE_INFO_ARM: Self = Self(1000507000);
	pub const DATA_GRAPH_PIPELINE_SESSION_CREATE_INFO_ARM: Self = Self(1000507001);
	pub const DATA_GRAPH_PIPELINE_RESOURCE_INFO_ARM: Self = Self(1000507002);
	pub const DATA_GRAPH_PIPELINE_CONSTANT_ARM: Self = Self(1000507003);
	pub const DATA_GRAPH_PIPELINE_SESSION_MEMORY_REQUIREMENTS_INFO_ARM: Self = Self(1000507004);
	pub const BIND_DATA_GRAPH_PIPELINE_SESSION_MEMORY_INFO_ARM: Self = Self(1000507005);
	pub const PHYSICAL_DEVICE_DATA_GRAPH_FEATURES_ARM: Self = Self(1000507006);
	pub const DATA_GRAPH_PIPELINE_SHADER_MODULE_CREATE_INFO_ARM: Self = Self(1000507007);
	pub const DATA_GRAPH_PIPELINE_PROPERTY_QUERY_RESULT_ARM: Self = Self(1000507008);
	pub const DATA_GRAPH_PIPELINE_INFO_ARM: Self = Self(1000507009);
	pub const DATA_GRAPH_PIPELINE_COMPILER_CONTROL_CREATE_INFO_ARM: Self = Self(1000507010);
	pub const DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENTS_INFO_ARM: Self = Self(1000507011);
	pub const DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENT_ARM: Self = Self(1000507012);
	pub const DATA_GRAPH_PIPELINE_IDENTIFIER_CREATE_INFO_ARM: Self = Self(1000507013);
	pub const DATA_GRAPH_PIPELINE_DISPATCH_INFO_ARM: Self = Self(1000507014);
	pub const DATA_GRAPH_PROCESSING_ENGINE_CREATE_INFO_ARM: Self = Self(1000507016);
	pub const QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_PROPERTIES_ARM: Self = Self(1000507017);
	pub const QUEUE_FAMILY_DATA_GRAPH_PROPERTIES_ARM: Self = Self(1000507018);
	pub const PHYSICAL_DEVICE_QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_INFO_ARM: Self =
		Self(1000507019);
	pub const DATA_GRAPH_PIPELINE_CONSTANT_TENSOR_SEMI_STRUCTURED_SPARSITY_INFO_ARM: Self =
		Self(1000507015);
	pub const QUEUE_FAMILY_DATA_GRAPH_TOSA_PROPERTIES_ARM: Self = Self(1000508000);
	pub const PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_RENDER_AREAS_FEATURES_QCOM: Self = Self(1000510000);
	pub const MULTIVIEW_PER_VIEW_RENDER_AREAS_RENDER_PASS_BEGIN_INFO_QCOM: Self = Self(1000510001);
	pub const PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_KHR: Self = Self(1000201000);
	pub const PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_PROPERTIES_KHR: Self = Self(1000511000);
	pub const VIDEO_DECODE_AV1_CAPABILITIES_KHR: Self = Self(1000512000);
	pub const VIDEO_DECODE_AV1_PICTURE_INFO_KHR: Self = Self(1000512001);
	pub const VIDEO_DECODE_AV1_PROFILE_INFO_KHR: Self = Self(1000512003);
	pub const VIDEO_DECODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000512004);
	pub const VIDEO_DECODE_AV1_DPB_SLOT_INFO_KHR: Self = Self(1000512005);
	pub const VIDEO_ENCODE_AV1_CAPABILITIES_KHR: Self = Self(1000513000);
	pub const VIDEO_ENCODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000513001);
	pub const VIDEO_ENCODE_AV1_PICTURE_INFO_KHR: Self = Self(1000513002);
	pub const VIDEO_ENCODE_AV1_DPB_SLOT_INFO_KHR: Self = Self(1000513003);
	pub const PHYSICAL_DEVICE_VIDEO_ENCODE_AV1_FEATURES_KHR: Self = Self(1000513004);
	pub const VIDEO_ENCODE_AV1_PROFILE_INFO_KHR: Self = Self(1000513005);
	pub const VIDEO_ENCODE_AV1_RATE_CONTROL_INFO_KHR: Self = Self(1000513006);
	pub const VIDEO_ENCODE_AV1_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1000513007);
	pub const VIDEO_ENCODE_AV1_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1000513008);
	pub const VIDEO_ENCODE_AV1_SESSION_CREATE_INFO_KHR: Self = Self(1000513009);
	pub const VIDEO_ENCODE_AV1_GOP_REMAINING_FRAME_INFO_KHR: Self = Self(1000513010);
	pub const PHYSICAL_DEVICE_VIDEO_DECODE_VP9_FEATURES_KHR: Self = Self(1000514000);
	pub const VIDEO_DECODE_VP9_CAPABILITIES_KHR: Self = Self(1000514001);
	pub const VIDEO_DECODE_VP9_PICTURE_INFO_KHR: Self = Self(1000514002);
	pub const VIDEO_DECODE_VP9_PROFILE_INFO_KHR: Self = Self(1000514003);
	pub const PHYSICAL_DEVICE_VIDEO_MAINTENANCE_1_FEATURES_KHR: Self = Self(1000515000);
	pub const VIDEO_INLINE_QUERY_INFO_KHR: Self = Self(1000515001);
	pub const PHYSICAL_DEVICE_PER_STAGE_DESCRIPTOR_SET_FEATURES_NV: Self = Self(1000516000);
	pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_2_FEATURES_QCOM: Self = Self(1000518000);
	pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_2_PROPERTIES_QCOM: Self = Self(1000518001);
	pub const SAMPLER_BLOCK_MATCH_WINDOW_CREATE_INFO_QCOM: Self = Self(1000518002);
	pub const SAMPLER_CUBIC_WEIGHTS_CREATE_INFO_QCOM: Self = Self(1000519000);
	pub const PHYSICAL_DEVICE_CUBIC_WEIGHTS_FEATURES_QCOM: Self = Self(1000519001);
	pub const BLIT_IMAGE_CUBIC_WEIGHTS_INFO_QCOM: Self = Self(1000519002);
	pub const PHYSICAL_DEVICE_YCBCR_DEGAMMA_FEATURES_QCOM: Self = Self(1000520000);
	pub const SAMPLER_YCBCR_CONVERSION_YCBCR_DEGAMMA_CREATE_INFO_QCOM: Self = Self(1000520001);
	pub const PHYSICAL_DEVICE_CUBIC_CLAMP_FEATURES_QCOM: Self = Self(1000521000);
	pub const PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_FEATURES_EXT: Self =
		Self(1000524000);
	pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_KHR: Self = Self(1000525000);
	pub const PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_KHR: Self = Self(1000190001);
	pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_KHR: Self = Self(1000190002);
	pub const PHYSICAL_DEVICE_UNIFIED_IMAGE_LAYOUTS_FEATURES_KHR: Self = Self(1000527000);
	pub const ATTACHMENT_FEEDBACK_LOOP_INFO_EXT: Self = Self(1000527001);
	pub const PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES_KHR: Self = Self(1000528000);
	#[cfg(feature = "platform-screen")]
	pub const SCREEN_BUFFER_PROPERTIES_QNX: Self = Self(1000529000);
	#[cfg(feature = "platform-screen")]
	pub const SCREEN_BUFFER_FORMAT_PROPERTIES_QNX: Self = Self(1000529001);
	#[cfg(feature = "platform-screen")]
	pub const IMPORT_SCREEN_BUFFER_INFO_QNX: Self = Self(1000529002);
	#[cfg(feature = "platform-screen")]
	pub const EXTERNAL_FORMAT_QNX: Self = Self(1000529003);
	#[cfg(feature = "platform-screen")]
	pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCREEN_BUFFER_FEATURES_QNX: Self = Self(1000529004);
	pub const PHYSICAL_DEVICE_LAYERED_DRIVER_PROPERTIES_MSFT: Self = Self(1000530000);
	pub const PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_KHR: Self = Self(1000265000);
	pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_KHR: Self = Self(1000259000);
	pub const PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_KHR: Self = Self(1000259001);
	pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_KHR: Self = Self(1000259002);
	pub const CALIBRATED_TIMESTAMP_INFO_KHR: Self = Self(1000184000);
	pub const PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES_KHR: Self = Self(1000544000);
	pub const PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES_KHR: Self = Self(1000545000);
	pub const PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES_KHR: Self = Self(1000545001);
	pub const BIND_MEMORY_STATUS_KHR: Self = Self(1000545002);
	pub const BIND_DESCRIPTOR_SETS_INFO_KHR: Self = Self(1000545003);
	pub const PUSH_CONSTANTS_INFO_KHR: Self = Self(1000545004);
	pub const PUSH_DESCRIPTOR_SET_INFO_KHR: Self = Self(1000545005);
	pub const PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO_KHR: Self = Self(1000545006);
	pub const SET_DESCRIPTOR_BUFFER_OFFSETS_INFO_EXT: Self = Self(1000545007);
	pub const BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS_INFO_EXT: Self = Self(1000545008);
	pub const PHYSICAL_DEVICE_DESCRIPTOR_POOL_OVERALLOCATION_FEATURES_NV: Self = Self(1000546000);
	pub const PHYSICAL_DEVICE_TILE_MEMORY_HEAP_FEATURES_QCOM: Self = Self(1000547000);
	pub const PHYSICAL_DEVICE_TILE_MEMORY_HEAP_PROPERTIES_QCOM: Self = Self(1000547001);
	pub const TILE_MEMORY_REQUIREMENTS_QCOM: Self = Self(1000547002);
	pub const TILE_MEMORY_BIND_INFO_QCOM: Self = Self(1000547003);
	pub const TILE_MEMORY_SIZE_INFO_QCOM: Self = Self(1000547004);
	pub const PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_KHR: Self = Self(1000549000);
	pub const PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_KHR: Self = Self(1000426001);
	pub const COPY_MEMORY_INDIRECT_INFO_KHR: Self = Self(1000549002);
	pub const COPY_MEMORY_TO_IMAGE_INDIRECT_INFO_KHR: Self = Self(1000549003);
	pub const PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_EXT: Self = Self(1000427000);
	pub const PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_EXT: Self = Self(1000427001);
	pub const DECOMPRESS_MEMORY_INFO_EXT: Self = Self(1000550002);
	pub const DISPLAY_SURFACE_STEREO_CREATE_INFO_NV: Self = Self(1000551000);
	pub const DISPLAY_MODE_STEREO_PROPERTIES_NV: Self = Self(1000551001);
	pub const VIDEO_ENCODE_INTRA_REFRESH_CAPABILITIES_KHR: Self = Self(1000552000);
	pub const VIDEO_ENCODE_SESSION_INTRA_REFRESH_CREATE_INFO_KHR: Self = Self(1000552001);
	pub const VIDEO_ENCODE_INTRA_REFRESH_INFO_KHR: Self = Self(1000552002);
	pub const VIDEO_REFERENCE_INTRA_REFRESH_INFO_KHR: Self = Self(1000552003);
	pub const PHYSICAL_DEVICE_VIDEO_ENCODE_INTRA_REFRESH_FEATURES_KHR: Self = Self(1000552004);
	pub const VIDEO_ENCODE_QUANTIZATION_MAP_CAPABILITIES_KHR: Self = Self(1000553000);
	pub const VIDEO_FORMAT_QUANTIZATION_MAP_PROPERTIES_KHR: Self = Self(1000553001);
	pub const VIDEO_ENCODE_QUANTIZATION_MAP_INFO_KHR: Self = Self(1000553002);
	pub const VIDEO_ENCODE_QUANTIZATION_MAP_SESSION_PARAMETERS_CREATE_INFO_KHR: Self =
		Self(1000553005);
	pub const PHYSICAL_DEVICE_VIDEO_ENCODE_QUANTIZATION_MAP_FEATURES_KHR: Self = Self(1000553009);
	pub const VIDEO_ENCODE_H264_QUANTIZATION_MAP_CAPABILITIES_KHR: Self = Self(1000553003);
	pub const VIDEO_ENCODE_H265_QUANTIZATION_MAP_CAPABILITIES_KHR: Self = Self(1000553004);
	pub const VIDEO_FORMAT_H265_QUANTIZATION_MAP_PROPERTIES_KHR: Self = Self(1000553006);
	pub const VIDEO_ENCODE_AV1_QUANTIZATION_MAP_CAPABILITIES_KHR: Self = Self(1000553007);
	pub const VIDEO_FORMAT_AV1_QUANTIZATION_MAP_PROPERTIES_KHR: Self = Self(1000553008);
	pub const PHYSICAL_DEVICE_RAW_ACCESS_CHAINS_FEATURES_NV: Self = Self(1000555000);
	pub const EXTERNAL_COMPUTE_QUEUE_DEVICE_CREATE_INFO_NV: Self = Self(1000556000);
	pub const EXTERNAL_COMPUTE_QUEUE_CREATE_INFO_NV: Self = Self(1000556001);
	pub const EXTERNAL_COMPUTE_QUEUE_DATA_PARAMS_NV: Self = Self(1000556002);
	pub const PHYSICAL_DEVICE_EXTERNAL_COMPUTE_QUEUE_PROPERTIES_NV: Self = Self(1000556003);
	pub const PHYSICAL_DEVICE_SHADER_RELAXED_EXTENDED_INSTRUCTION_FEATURES_KHR: Self =
		Self(1000558000);
	pub const PHYSICAL_DEVICE_COMMAND_BUFFER_INHERITANCE_FEATURES_NV: Self = Self(1000559000);
	pub const PHYSICAL_DEVICE_MAINTENANCE_7_FEATURES_KHR: Self = Self(1000562000);
	pub const PHYSICAL_DEVICE_MAINTENANCE_7_PROPERTIES_KHR: Self = Self(1000562001);
	pub const PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_LIST_KHR: Self = Self(1000562002);
	pub const PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_KHR: Self = Self(1000562003);
	pub const PHYSICAL_DEVICE_LAYERED_API_VULKAN_PROPERTIES_KHR: Self = Self(1000562004);
	pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT16_VECTOR_FEATURES_NV: Self = Self(1000563000);
	pub const PHYSICAL_DEVICE_SHADER_REPLICATED_COMPOSITES_FEATURES_EXT: Self = Self(1000564000);
	pub const TENSOR_EXPLICIT_TILING_FORMAT_PROPERTIES_ARM: Self = Self(1000565000);
	pub const TENSOR_ROLLING_BACKING_CREATE_INFO_ARM: Self = Self(1000565001);
	pub const PHYSICAL_DEVICE_SHADER_FLOAT8_FEATURES_EXT: Self = Self(1000567000);
	pub const PHYSICAL_DEVICE_RAY_TRACING_VALIDATION_FEATURES_NV: Self = Self(1000568000);
	pub const PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_FEATURES_NV: Self = Self(1000569000);
	pub const PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_PROPERTIES_NV: Self = Self(1000569001);
	pub const CLUSTER_ACCELERATION_STRUCTURE_CLUSTERS_BOTTOM_LEVEL_INPUT_NV: Self = Self(1000569002);
	pub const CLUSTER_ACCELERATION_STRUCTURE_TRIANGLE_CLUSTER_INPUT_NV: Self = Self(1000569003);
	pub const CLUSTER_ACCELERATION_STRUCTURE_MOVE_OBJECTS_INPUT_NV: Self = Self(1000569004);
	pub const CLUSTER_ACCELERATION_STRUCTURE_INPUT_INFO_NV: Self = Self(1000569005);
	pub const CLUSTER_ACCELERATION_STRUCTURE_COMMANDS_INFO_NV: Self = Self(1000569006);
	pub const RAY_TRACING_PIPELINE_CLUSTER_ACCELERATION_STRUCTURE_CREATE_INFO_NV: Self =
		Self(1000569007);
	pub const PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_FEATURES_NV: Self = Self(1000570000);
	pub const PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_PROPERTIES_NV: Self =
		Self(1000570001);
	pub const WRITE_DESCRIPTOR_SET_PARTITIONED_ACCELERATION_STRUCTURE_NV: Self = Self(1000570002);
	pub const PARTITIONED_ACCELERATION_STRUCTURE_INSTANCES_INPUT_NV: Self = Self(1000570003);
	pub const BUILD_PARTITIONED_ACCELERATION_STRUCTURE_INFO_NV: Self = Self(1000570004);
	pub const PARTITIONED_ACCELERATION_STRUCTURE_FLAGS_NV: Self = Self(1000570005);
	pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_EXT: Self = Self(1000572000);
	pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_EXT: Self = Self(1000572001);
	pub const GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_EXT: Self = Self(1000572002);
	pub const INDIRECT_EXECUTION_SET_CREATE_INFO_EXT: Self = Self(1000572003);
	pub const GENERATED_COMMANDS_INFO_EXT: Self = Self(1000572004);
	pub const INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_EXT: Self = Self(1000572006);
	pub const INDIRECT_COMMANDS_LAYOUT_TOKEN_EXT: Self = Self(1000572007);
	pub const WRITE_INDIRECT_EXECUTION_SET_PIPELINE_EXT: Self = Self(1000572008);
	pub const WRITE_INDIRECT_EXECUTION_SET_SHADER_EXT: Self = Self(1000572009);
	pub const INDIRECT_EXECUTION_SET_PIPELINE_INFO_EXT: Self = Self(1000572010);
	pub const INDIRECT_EXECUTION_SET_SHADER_INFO_EXT: Self = Self(1000572011);
	pub const INDIRECT_EXECUTION_SET_SHADER_LAYOUT_INFO_EXT: Self = Self(1000572012);
	pub const GENERATED_COMMANDS_PIPELINE_INFO_EXT: Self = Self(1000572013);
	pub const GENERATED_COMMANDS_SHADER_INFO_EXT: Self = Self(1000572014);
	pub const PHYSICAL_DEVICE_FAULT_FEATURES_KHR: Self = Self(1000573000);
	pub const PHYSICAL_DEVICE_FAULT_PROPERTIES_KHR: Self = Self(1000573001);
	pub const DEVICE_FAULT_INFO_KHR: Self = Self(1000573002);
	pub const DEVICE_FAULT_DEBUG_INFO_KHR: Self = Self(1000573003);
	pub const PHYSICAL_DEVICE_MAINTENANCE_8_FEATURES_KHR: Self = Self(1000574000);
	pub const MEMORY_BARRIER_ACCESS_FLAGS_3_KHR: Self = Self(1000574002);
	pub const PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_FEATURES_MESA: Self = Self(1000575000);
	pub const PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_PROPERTIES_MESA: Self = Self(1000575001);
	pub const IMAGE_ALIGNMENT_CONTROL_CREATE_INFO_MESA: Self = Self(1000575002);
	pub const PHYSICAL_DEVICE_SHADER_FMA_FEATURES_KHR: Self = Self(1000579000);
	pub const PUSH_CONSTANT_BANK_INFO_NV: Self = Self(1000580000);
	pub const PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_FEATURES_NV: Self = Self(1000580001);
	pub const PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_PROPERTIES_NV: Self = Self(1000580002);
	pub const PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_EXT: Self = Self(1000581000);
	pub const PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_EXT: Self = Self(1000581001);
	pub const PHYSICAL_DEVICE_DEPTH_CLAMP_CONTROL_FEATURES_EXT: Self = Self(1000582000);
	pub const PIPELINE_VIEWPORT_DEPTH_CLAMP_CONTROL_CREATE_INFO_EXT: Self = Self(1000582001);
	pub const PHYSICAL_DEVICE_MAINTENANCE_9_FEATURES_KHR: Self = Self(1000584000);
	pub const PHYSICAL_DEVICE_MAINTENANCE_9_PROPERTIES_KHR: Self = Self(1000584001);
	pub const QUEUE_FAMILY_OWNERSHIP_TRANSFER_PROPERTIES_KHR: Self = Self(1000584002);
	pub const PHYSICAL_DEVICE_VIDEO_MAINTENANCE_2_FEATURES_KHR: Self = Self(1000586000);
	pub const VIDEO_DECODE_H264_INLINE_SESSION_PARAMETERS_INFO_KHR: Self = Self(1000586001);
	pub const VIDEO_DECODE_H265_INLINE_SESSION_PARAMETERS_INFO_KHR: Self = Self(1000586002);
	pub const VIDEO_DECODE_AV1_INLINE_SESSION_PARAMETERS_INFO_KHR: Self = Self(1000586003);
	#[cfg(feature = "platform-ohos")]
	pub const SURFACE_CREATE_INFO_OHOS: Self = Self(1000685000);
	pub const PHYSICAL_DEVICE_HDR_VIVID_FEATURES_HUAWEI: Self = Self(1000590000);
	pub const HDR_VIVID_DYNAMIC_METADATA_HUAWEI: Self = Self(1000590001);
	pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_FEATURES_NV: Self = Self(1000593000);
	pub const COOPERATIVE_MATRIX_FLEXIBLE_DIMENSIONS_PROPERTIES_NV: Self = Self(1000593001);
	pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_PROPERTIES_NV: Self = Self(1000593002);
	pub const PHYSICAL_DEVICE_PIPELINE_OPACITY_MICROMAP_FEATURES_ARM: Self = Self(1000596000);
	pub const PHYSICAL_DEVICE_VIDEO_ENCODE_FEEDBACK_2_FEATURES_KHR: Self = Self(1000598000);
	pub const VIDEO_ENCODE_FEEDBACK_2_CAPABILITIES_KHR: Self = Self(1000598001);
	pub const QUERY_POOL_VIDEO_ENCODE_PER_PARTITION_FEEDBACK_CREATE_INFO_KHR: Self = Self(1000598002);
	#[cfg(feature = "platform-metal")]
	pub const IMPORT_MEMORY_METAL_HANDLE_INFO_EXT: Self = Self(1000602000);
	#[cfg(feature = "platform-metal")]
	pub const MEMORY_METAL_HANDLE_PROPERTIES_EXT: Self = Self(1000602001);
	#[cfg(feature = "platform-metal")]
	pub const MEMORY_GET_METAL_HANDLE_INFO_EXT: Self = Self(1000602002);
	pub const PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_KHR: Self = Self(1000421000);
	pub const PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_FEATURES_ARM: Self = Self(1000605000);
	pub const PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_PROPERTIES_ARM: Self = Self(1000605001);
	pub const PERFORMANCE_COUNTER_ARM: Self = Self(1000605002);
	pub const PERFORMANCE_COUNTER_DESCRIPTION_ARM: Self = Self(1000605003);
	pub const RENDER_PASS_PERFORMANCE_COUNTERS_BY_REGION_BEGIN_INFO_ARM: Self = Self(1000605004);
	pub const PHYSICAL_DEVICE_SHADER_INSTRUMENTATION_FEATURES_ARM: Self = Self(1000607000);
	pub const PHYSICAL_DEVICE_SHADER_INSTRUMENTATION_PROPERTIES_ARM: Self = Self(1000607001);
	pub const SHADER_INSTRUMENTATION_CREATE_INFO_ARM: Self = Self(1000607002);
	pub const SHADER_INSTRUMENTATION_METRIC_DESCRIPTION_ARM: Self = Self(1000607003);
	pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_ROBUSTNESS_FEATURES_EXT: Self = Self(1000608000);
	pub const PHYSICAL_DEVICE_FORMAT_PACK_FEATURES_ARM: Self = Self(1000609000);
	pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_FEATURES_VALVE: Self = Self(1000611000);
	pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_PROPERTIES_VALVE: Self = Self(1000611001);
	pub const PIPELINE_FRAGMENT_DENSITY_MAP_LAYERED_CREATE_INFO_VALVE: Self = Self(1000611002);
	pub const PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_KHR: Self = Self(1000286000);
	pub const PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_KHR: Self = Self(1000286001);
	pub const SET_PRESENT_CONFIG_NV: Self = Self(1000613000);
	pub const PHYSICAL_DEVICE_PRESENT_METERING_FEATURES_NV: Self = Self(1000613001);
	pub const PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SWAPCHAIN_FEATURES_EXT: Self = Self(1000616000);
	pub const SWAPCHAIN_FLAGS_SURFACE_CAPABILITIES_EXT: Self = Self(1000616001);
	pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_EXT: Self = Self(1000425000);
	pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_EXT: Self = Self(1000425001);
	pub const RENDER_PASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_EXT: Self = Self(1000425002);
	pub const RENDERING_END_INFO_EXT: Self = Self(1000619003);
	pub const PHYSICAL_DEVICE_ZERO_INITIALIZE_DEVICE_MEMORY_FEATURES_EXT: Self = Self(1000620000);
	pub const PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_KHR: Self = Self(1000361000);
	pub const PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_KHR: Self = Self(1000623000);
	pub const PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_KHR: Self = Self(1000623001);
	pub const ACCELERATION_STRUCTURE_GEOMETRY_MICROMAP_DATA_KHR: Self = Self(1000623002);
	pub const ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_KHR: Self = Self(1000623003);
	pub const PHYSICAL_DEVICE_SHADER_64_BIT_INDEXING_FEATURES_EXT: Self = Self(1000627000);
	pub const PHYSICAL_DEVICE_CUSTOM_RESOLVE_FEATURES_EXT: Self = Self(1000628000);
	pub const BEGIN_CUSTOM_RESOLVE_INFO_EXT: Self = Self(1000628001);
	pub const CUSTOM_RESOLVE_CREATE_INFO_EXT: Self = Self(1000628002);
	pub const PHYSICAL_DEVICE_DATA_GRAPH_MODEL_FEATURES_QCOM: Self = Self(1000629000);
	pub const DATA_GRAPH_PIPELINE_BUILTIN_MODEL_CREATE_INFO_QCOM: Self = Self(1000629001);
	pub const PHYSICAL_DEVICE_MAINTENANCE_10_FEATURES_KHR: Self = Self(1000630000);
	pub const PHYSICAL_DEVICE_MAINTENANCE_10_PROPERTIES_KHR: Self = Self(1000630001);
	pub const RENDERING_ATTACHMENT_FLAGS_INFO_KHR: Self = Self(1000630002);
	pub const RENDERING_END_INFO_KHR: Self = Self(1000619003);
	pub const RESOLVE_IMAGE_MODE_INFO_KHR: Self = Self(1000630004);
	pub const PHYSICAL_DEVICE_DATA_GRAPH_OPTICAL_FLOW_FEATURES_ARM: Self = Self(1000631000);
	pub const QUEUE_FAMILY_DATA_GRAPH_OPTICAL_FLOW_PROPERTIES_ARM: Self = Self(1000631001);
	pub const DATA_GRAPH_OPTICAL_FLOW_IMAGE_FORMAT_INFO_ARM: Self = Self(1000631003);
	pub const DATA_GRAPH_OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_ARM: Self = Self(1000631004);
	pub const DATA_GRAPH_PIPELINE_OPTICAL_FLOW_DISPATCH_INFO_ARM: Self = Self(1000631005);
	pub const DATA_GRAPH_PIPELINE_OPTICAL_FLOW_CREATE_INFO_ARM: Self = Self(1000631002);
	pub const DATA_GRAPH_PIPELINE_RESOURCE_INFO_IMAGE_LAYOUT_ARM: Self = Self(1000631006);
	pub const DATA_GRAPH_PIPELINE_SINGLE_NODE_CREATE_INFO_ARM: Self = Self(1000631007);
	pub const DATA_GRAPH_PIPELINE_SINGLE_NODE_CONNECTION_ARM: Self = Self(1000631008);
	pub const PHYSICAL_DEVICE_SHADER_LONG_VECTOR_FEATURES_EXT: Self = Self(1000635000);
	pub const PHYSICAL_DEVICE_SHADER_LONG_VECTOR_PROPERTIES_EXT: Self = Self(1000635001);
	pub const PHYSICAL_DEVICE_PIPELINE_CACHE_INCREMENTAL_MODE_FEATURES_SEC: Self = Self(1000637000);
	pub const PHYSICAL_DEVICE_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_FEATURES_EXT: Self =
		Self(1000642000);
	pub const COMPUTE_OCCUPANCY_PRIORITY_PARAMETERS_NV: Self = Self(1000645000);
	pub const PHYSICAL_DEVICE_COMPUTE_OCCUPANCY_PRIORITY_FEATURES_NV: Self = Self(1000645001);
	pub const PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_KHR: Self = Self(1000498000);
	pub const PHYSICAL_DEVICE_MAINTENANCE_11_FEATURES_KHR: Self = Self(1000657000);
	pub const QUEUE_FAMILY_OPTIMAL_IMAGE_TRANSFER_GRANULARITY_PROPERTIES_KHR: Self = Self(1000657001);
	pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_MAINTENANCE_1_FEATURES_EXT: Self = Self(1000659000);
	pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_INFO_2_EXT: Self = Self(1000659001);
	pub const COOPERATIVE_MATRIX_PROPERTIES_2_EXT: Self = Self(1000659002);
	pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_PARTITIONED_FEATURES_EXT: Self = Self(1000662000);
	#[cfg(feature = "platform-ubm")]
	pub const UBM_SURFACE_CREATE_INFO_SEC: Self = Self(1000664000);
	pub const FORMAT_PROPERTIES_4_KHR: Self = Self(1000668000);
	pub const IMAGE_CREATE_FLAGS_2_CREATE_INFO_KHR: Self = Self(1000668001);
	pub const IMAGE_USAGE_FLAGS_2_CREATE_INFO_KHR: Self = Self(1000668002);
	pub const IMAGE_VIEW_USAGE_2_CREATE_INFO_KHR: Self = Self(1000668003);
	pub const PHYSICAL_DEVICE_EXTENDED_FLAGS_FEATURES_KHR: Self = Self(1000668004);
	pub const IMAGE_STENCIL_USAGE_2_CREATE_INFO_KHR: Self = Self(1000668005);
	pub const SHARED_PRESENT_SURFACE_CAPABILITIES_2_KHR: Self = Self(1000668006);
	pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_LAYOUTS_FEATURES_ARM: Self = Self(1000670000);
	pub const PHYSICAL_DEVICE_SHADER_OCP_MICROSCALING_TYPES_FEATURES_EXT: Self = Self(1000672000);
	pub const PHYSICAL_DEVICE_SHADER_MIXED_FLOAT_DOT_PRODUCT_FEATURES_VALVE: Self = Self(1000673000);
	pub const PHYSICAL_DEVICE_THROTTLE_HINT_FEATURES_SEC: Self = Self(1000674000);
	pub const THROTTLE_HINT_SUBMIT_INFO_SEC: Self = Self(1000674001);
	pub const DATA_GRAPH_PIPELINE_NEURAL_STATISTICS_CREATE_INFO_ARM: Self = Self(1000676000);
	pub const DATA_GRAPH_PIPELINE_SESSION_NEURAL_STATISTICS_CREATE_INFO_ARM: Self = Self(1000676001);
	pub const PHYSICAL_DEVICE_DATA_GRAPH_NEURAL_ACCELERATOR_STATISTICS_FEATURES_ARM: Self =
		Self(1000676002);
	pub const PHYSICAL_DEVICE_PRIMITIVE_RESTART_INDEX_FEATURES_EXT: Self = Self(1000678000);
	pub const PHYSICAL_DEVICE_IMAGE_TILING_CONTROL_FEATURES_EXT: Self = Self(1000687000);
	pub const IMAGE_TILING_CONTROL_CREATE_INFO_EXT: Self = Self(1000687001);
	pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_DECODE_VECTOR_FEATURES_NV: Self = Self(1000689000);
	pub const PHYSICAL_DEVICE_PRIVATE_DATA_BASE_HANDLE_FEATURES_NV: Self = Self(1000707000);
	pub const PHYSICAL_DEVICE_INFO_PROPERTIES_INTEL: Self = Self(1000708000);
	pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_ALLOCATION_ALIGNMENT_FEATURES_VALVE: Self =
		Self(1000709000);
	pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_ALLOCATION_ALIGNMENT_PROPERTIES_VALVE: Self =
		Self(1000709001);
	pub const BUFFER_DEVICE_ADDRESS_ALIGNMENT_ALLOCATE_INFO_VALVE: Self = Self(1000709002);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SubpassContents(pub i32);
impl SubpassContents {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const INLINE: Self = Self(0);
	pub const SECONDARY_COMMAND_BUFFERS: Self = Self(1);
	pub const INLINE_AND_SECONDARY_COMMAND_BUFFERS_EXT: Self = Self(1000451000);
	pub const INLINE_AND_SECONDARY_COMMAND_BUFFERS_KHR: Self = Self(1000451000);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Result(pub i32);
impl Result {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const SUCCESS: Self = Self(0);
	pub const NOT_READY: Self = Self(1);
	pub const TIMEOUT: Self = Self(2);
	pub const EVENT_SET: Self = Self(3);
	pub const EVENT_RESET: Self = Self(4);
	pub const INCOMPLETE: Self = Self(5);
	pub const ERROR_OUT_OF_HOST_MEMORY: Self = Self(-1);
	pub const ERROR_OUT_OF_DEVICE_MEMORY: Self = Self(-2);
	pub const ERROR_INITIALIZATION_FAILED: Self = Self(-3);
	pub const ERROR_DEVICE_LOST: Self = Self(-4);
	pub const ERROR_MEMORY_MAP_FAILED: Self = Self(-5);
	pub const ERROR_LAYER_NOT_PRESENT: Self = Self(-6);
	pub const ERROR_EXTENSION_NOT_PRESENT: Self = Self(-7);
	pub const ERROR_FEATURE_NOT_PRESENT: Self = Self(-8);
	pub const ERROR_INCOMPATIBLE_DRIVER: Self = Self(-9);
	pub const ERROR_TOO_MANY_OBJECTS: Self = Self(-10);
	pub const ERROR_FORMAT_NOT_SUPPORTED: Self = Self(-11);
	pub const ERROR_FRAGMENTED_POOL: Self = Self(-12);
	pub const ERROR_UNKNOWN: Self = Self(-13);
	pub const ERROR_VALIDATION_FAILED: Self = Self(-1000011001);
	pub const ERROR_OUT_OF_POOL_MEMORY: Self = Self(-1000069000);
	pub const ERROR_INVALID_EXTERNAL_HANDLE: Self = Self(-1000072003);
	pub const ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS: Self = Self(-1000257000);
	pub const ERROR_FRAGMENTATION: Self = Self(-1000161000);
	pub const PIPELINE_COMPILE_REQUIRED: Self = Self(1000297000);
	pub const ERROR_NOT_PERMITTED: Self = Self(-1000174001);
	pub const ERROR_SURFACE_LOST_KHR: Self = Self(-1000000000);
	pub const ERROR_NATIVE_WINDOW_IN_USE_KHR: Self = Self(-1000000001);
	pub const SUBOPTIMAL_KHR: Self = Self(1000001003);
	pub const ERROR_OUT_OF_DATE_KHR: Self = Self(-1000001004);
	pub const ERROR_INCOMPATIBLE_DISPLAY_KHR: Self = Self(-1000003001);
	pub const ERROR_VALIDATION_FAILED_EXT: Self = Self(-1000011001);
	pub const ERROR_INVALID_SHADER_NV: Self = Self(-1000012000);
	pub const ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR: Self = Self(-1000023000);
	pub const ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR: Self = Self(-1000023001);
	pub const ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR: Self = Self(-1000023002);
	pub const ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR: Self = Self(-1000023003);
	pub const ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR: Self = Self(-1000023004);
	pub const ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR: Self = Self(-1000023005);
	pub const ERROR_OUT_OF_POOL_MEMORY_KHR: Self = Self(-1000069000);
	pub const ERROR_INVALID_EXTERNAL_HANDLE_KHR: Self = Self(-1000072003);
	pub const ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT: Self = Self(-1000158000);
	pub const ERROR_FRAGMENTATION_EXT: Self = Self(-1000161000);
	pub const ERROR_NOT_PERMITTED_EXT: Self = Self(-1000174001);
	pub const ERROR_NOT_PERMITTED_KHR: Self = Self(-1000174001);
	pub const ERROR_PRESENT_TIMING_QUEUE_FULL_EXT: Self = Self(-1000208000);
	pub const ERROR_INVALID_DEVICE_ADDRESS_EXT: Self = Self(-1000257000);
	#[cfg(feature = "platform-win32")]
	pub const ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT: Self = Self(-1000255000);
	pub const ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR: Self = Self(-1000257000);
	pub const THREAD_IDLE_KHR: Self = Self(1000268000);
	pub const THREAD_DONE_KHR: Self = Self(1000268001);
	pub const OPERATION_DEFERRED_KHR: Self = Self(1000268002);
	pub const OPERATION_NOT_DEFERRED_KHR: Self = Self(1000268003);
	pub const PIPELINE_COMPILE_REQUIRED_EXT: Self = Self(1000297000);
	pub const ERROR_PIPELINE_COMPILE_REQUIRED_EXT: Self = Self(1000297000);
	pub const ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR: Self = Self(-1000299000);
	pub const ERROR_COMPRESSION_EXHAUSTED_EXT: Self = Self(-1000338000);
	pub const INCOMPATIBLE_SHADER_BINARY_EXT: Self = Self(1000482000);
	pub const ERROR_INCOMPATIBLE_SHADER_BINARY_EXT: Self = Self(1000482000);
	pub const PIPELINE_BINARY_MISSING_KHR: Self = Self(1000483000);
	pub const ERROR_NOT_ENOUGH_SPACE_KHR: Self = Self(-1000483000);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DynamicState(pub i32);
impl DynamicState {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const VIEWPORT: Self = Self(0);
	pub const SCISSOR: Self = Self(1);
	pub const LINE_WIDTH: Self = Self(2);
	pub const DEPTH_BIAS: Self = Self(3);
	pub const BLEND_CONSTANTS: Self = Self(4);
	pub const DEPTH_BOUNDS: Self = Self(5);
	pub const STENCIL_COMPARE_MASK: Self = Self(6);
	pub const STENCIL_WRITE_MASK: Self = Self(7);
	pub const STENCIL_REFERENCE: Self = Self(8);
	pub const CULL_MODE: Self = Self(1000267000);
	pub const FRONT_FACE: Self = Self(1000267001);
	pub const PRIMITIVE_TOPOLOGY: Self = Self(1000267002);
	pub const VIEWPORT_WITH_COUNT: Self = Self(1000267003);
	pub const SCISSOR_WITH_COUNT: Self = Self(1000267004);
	pub const VERTEX_INPUT_BINDING_STRIDE: Self = Self(1000267005);
	pub const DEPTH_TEST_ENABLE: Self = Self(1000267006);
	pub const DEPTH_WRITE_ENABLE: Self = Self(1000267007);
	pub const DEPTH_COMPARE_OP: Self = Self(1000267008);
	pub const DEPTH_BOUNDS_TEST_ENABLE: Self = Self(1000267009);
	pub const STENCIL_TEST_ENABLE: Self = Self(1000267010);
	pub const STENCIL_OP: Self = Self(1000267011);
	pub const RASTERIZER_DISCARD_ENABLE: Self = Self(1000377001);
	pub const DEPTH_BIAS_ENABLE: Self = Self(1000377002);
	pub const PRIMITIVE_RESTART_ENABLE: Self = Self(1000377004);
	pub const LINE_STIPPLE: Self = Self(1000259000);
	pub const VIEWPORT_W_SCALING_NV: Self = Self(1000087000);
	pub const DISCARD_RECTANGLE_EXT: Self = Self(1000099000);
	pub const DISCARD_RECTANGLE_ENABLE_EXT: Self = Self(1000099001);
	pub const DISCARD_RECTANGLE_MODE_EXT: Self = Self(1000099002);
	pub const SAMPLE_LOCATIONS_EXT: Self = Self(1000143000);
	pub const RAY_TRACING_PIPELINE_STACK_SIZE_KHR: Self = Self(1000347000);
	pub const VIEWPORT_SHADING_RATE_PALETTE_NV: Self = Self(1000164004);
	pub const VIEWPORT_COARSE_SAMPLE_ORDER_NV: Self = Self(1000164006);
	pub const EXCLUSIVE_SCISSOR_ENABLE_NV: Self = Self(1000205000);
	pub const EXCLUSIVE_SCISSOR_NV: Self = Self(1000205001);
	pub const FRAGMENT_SHADING_RATE_KHR: Self = Self(1000226000);
	pub const LINE_STIPPLE_EXT: Self = Self(1000259000);
	pub const CULL_MODE_EXT: Self = Self(1000267000);
	pub const FRONT_FACE_EXT: Self = Self(1000267001);
	pub const PRIMITIVE_TOPOLOGY_EXT: Self = Self(1000267002);
	pub const VIEWPORT_WITH_COUNT_EXT: Self = Self(1000267003);
	pub const SCISSOR_WITH_COUNT_EXT: Self = Self(1000267004);
	pub const VERTEX_INPUT_BINDING_STRIDE_EXT: Self = Self(1000267005);
	pub const DEPTH_TEST_ENABLE_EXT: Self = Self(1000267006);
	pub const DEPTH_WRITE_ENABLE_EXT: Self = Self(1000267007);
	pub const DEPTH_COMPARE_OP_EXT: Self = Self(1000267008);
	pub const DEPTH_BOUNDS_TEST_ENABLE_EXT: Self = Self(1000267009);
	pub const STENCIL_TEST_ENABLE_EXT: Self = Self(1000267010);
	pub const STENCIL_OP_EXT: Self = Self(1000267011);
	pub const VERTEX_INPUT_EXT: Self = Self(1000352000);
	pub const PATCH_CONTROL_POINTS_EXT: Self = Self(1000377000);
	pub const RASTERIZER_DISCARD_ENABLE_EXT: Self = Self(1000377001);
	pub const DEPTH_BIAS_ENABLE_EXT: Self = Self(1000377002);
	pub const LOGIC_OP_EXT: Self = Self(1000377003);
	pub const PRIMITIVE_RESTART_ENABLE_EXT: Self = Self(1000377004);
	pub const COLOR_WRITE_ENABLE_EXT: Self = Self(1000381000);
	pub const DEPTH_CLAMP_ENABLE_EXT: Self = Self(1000455003);
	pub const POLYGON_MODE_EXT: Self = Self(1000455004);
	pub const RASTERIZATION_SAMPLES_EXT: Self = Self(1000455005);
	pub const SAMPLE_MASK_EXT: Self = Self(1000455006);
	pub const ALPHA_TO_COVERAGE_ENABLE_EXT: Self = Self(1000455007);
	pub const ALPHA_TO_ONE_ENABLE_EXT: Self = Self(1000455008);
	pub const LOGIC_OP_ENABLE_EXT: Self = Self(1000455009);
	pub const COLOR_BLEND_ENABLE_EXT: Self = Self(1000455010);
	pub const COLOR_BLEND_EQUATION_EXT: Self = Self(1000455011);
	pub const COLOR_WRITE_MASK_EXT: Self = Self(1000455012);
	pub const TESSELLATION_DOMAIN_ORIGIN_EXT: Self = Self(1000455002);
	pub const RASTERIZATION_STREAM_EXT: Self = Self(1000455013);
	pub const CONSERVATIVE_RASTERIZATION_MODE_EXT: Self = Self(1000455014);
	pub const EXTRA_PRIMITIVE_OVERESTIMATION_SIZE_EXT: Self = Self(1000455015);
	pub const DEPTH_CLIP_ENABLE_EXT: Self = Self(1000455016);
	pub const SAMPLE_LOCATIONS_ENABLE_EXT: Self = Self(1000455017);
	pub const COLOR_BLEND_ADVANCED_EXT: Self = Self(1000455018);
	pub const PROVOKING_VERTEX_MODE_EXT: Self = Self(1000455019);
	pub const LINE_RASTERIZATION_MODE_EXT: Self = Self(1000455020);
	pub const LINE_STIPPLE_ENABLE_EXT: Self = Self(1000455021);
	pub const DEPTH_CLIP_NEGATIVE_ONE_TO_ONE_EXT: Self = Self(1000455022);
	pub const VIEWPORT_W_SCALING_ENABLE_NV: Self = Self(1000455023);
	pub const VIEWPORT_SWIZZLE_NV: Self = Self(1000455024);
	pub const COVERAGE_TO_COLOR_ENABLE_NV: Self = Self(1000455025);
	pub const COVERAGE_TO_COLOR_LOCATION_NV: Self = Self(1000455026);
	pub const COVERAGE_MODULATION_MODE_NV: Self = Self(1000455027);
	pub const COVERAGE_MODULATION_TABLE_ENABLE_NV: Self = Self(1000455028);
	pub const COVERAGE_MODULATION_TABLE_NV: Self = Self(1000455029);
	pub const SHADING_RATE_IMAGE_ENABLE_NV: Self = Self(1000455030);
	pub const REPRESENTATIVE_FRAGMENT_TEST_ENABLE_NV: Self = Self(1000455031);
	pub const COVERAGE_REDUCTION_MODE_NV: Self = Self(1000455032);
	pub const ATTACHMENT_FEEDBACK_LOOP_ENABLE_EXT: Self = Self(1000524000);
	pub const LINE_STIPPLE_KHR: Self = Self(1000259000);
	pub const DEPTH_CLAMP_RANGE_EXT: Self = Self(1000582000);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DescriptorUpdateTemplateType(pub i32);
impl DescriptorUpdateTemplateType {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DESCRIPTOR_SET: Self = Self(0);
	pub const PUSH_DESCRIPTORS: Self = Self(1);
	pub const PUSH_DESCRIPTORS_KHR: Self = Self(1);
	pub const DESCRIPTOR_SET_KHR: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ObjectType(pub i32);
impl ObjectType {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const UNKNOWN: Self = Self(0);
	pub const INSTANCE: Self = Self(1);
	pub const PHYSICAL_DEVICE: Self = Self(2);
	pub const DEVICE: Self = Self(3);
	pub const QUEUE: Self = Self(4);
	pub const SEMAPHORE: Self = Self(5);
	pub const COMMAND_BUFFER: Self = Self(6);
	pub const FENCE: Self = Self(7);
	pub const DEVICE_MEMORY: Self = Self(8);
	pub const BUFFER: Self = Self(9);
	pub const IMAGE: Self = Self(10);
	pub const EVENT: Self = Self(11);
	pub const QUERY_POOL: Self = Self(12);
	pub const BUFFER_VIEW: Self = Self(13);
	pub const IMAGE_VIEW: Self = Self(14);
	pub const SHADER_MODULE: Self = Self(15);
	pub const PIPELINE_CACHE: Self = Self(16);
	pub const PIPELINE_LAYOUT: Self = Self(17);
	pub const RENDER_PASS: Self = Self(18);
	pub const PIPELINE: Self = Self(19);
	pub const DESCRIPTOR_SET_LAYOUT: Self = Self(20);
	pub const SAMPLER: Self = Self(21);
	pub const DESCRIPTOR_POOL: Self = Self(22);
	pub const DESCRIPTOR_SET: Self = Self(23);
	pub const FRAMEBUFFER: Self = Self(24);
	pub const COMMAND_POOL: Self = Self(25);
	pub const DESCRIPTOR_UPDATE_TEMPLATE: Self = Self(1000085000);
	pub const SAMPLER_YCBCR_CONVERSION: Self = Self(1000156000);
	pub const PRIVATE_DATA_SLOT: Self = Self(1000295000);
	pub const SURFACE_KHR: Self = Self(1000000000);
	pub const SWAPCHAIN_KHR: Self = Self(1000001000);
	pub const DISPLAY_KHR: Self = Self(1000002000);
	pub const DISPLAY_MODE_KHR: Self = Self(1000002001);
	pub const DEBUG_REPORT_CALLBACK_EXT: Self = Self(1000011000);
	pub const VIDEO_SESSION_KHR: Self = Self(1000023000);
	pub const VIDEO_SESSION_PARAMETERS_KHR: Self = Self(1000023001);
	pub const CU_MODULE_NVX: Self = Self(1000029000);
	pub const CU_FUNCTION_NVX: Self = Self(1000029001);
	pub const DESCRIPTOR_UPDATE_TEMPLATE_KHR: Self = Self(1000085000);
	pub const DEBUG_UTILS_MESSENGER_EXT: Self = Self(1000128000);
	pub const GPA_SESSION_AMD: Self = Self(1000133000);
	pub const ACCELERATION_STRUCTURE_KHR: Self = Self(1000150000);
	pub const SAMPLER_YCBCR_CONVERSION_KHR: Self = Self(1000156000);
	pub const VALIDATION_CACHE_EXT: Self = Self(1000160000);
	pub const ACCELERATION_STRUCTURE_NV: Self = Self(1000165000);
	pub const PERFORMANCE_CONFIGURATION_INTEL: Self = Self(1000210000);
	pub const DEFERRED_OPERATION_KHR: Self = Self(1000268000);
	pub const INDIRECT_COMMANDS_LAYOUT_NV: Self = Self(1000277000);
	pub const PRIVATE_DATA_SLOT_EXT: Self = Self(1000295000);
	#[cfg(feature = "provisional")]
	pub const CUDA_MODULE_NV: Self = Self(1000307000);
	#[cfg(feature = "provisional")]
	pub const CUDA_FUNCTION_NV: Self = Self(1000307001);
	#[cfg(feature = "platform-fuchsia")]
	pub const BUFFER_COLLECTION_FUCHSIA: Self = Self(1000366000);
	pub const MICROMAP_EXT: Self = Self(1000396000);
	pub const TENSOR_ARM: Self = Self(1000460000);
	pub const TENSOR_VIEW_ARM: Self = Self(1000460001);
	pub const OPTICAL_FLOW_SESSION_NV: Self = Self(1000464000);
	pub const SHADER_EXT: Self = Self(1000482000);
	pub const PIPELINE_BINARY_KHR: Self = Self(1000483000);
	pub const DATA_GRAPH_PIPELINE_SESSION_ARM: Self = Self(1000507000);
	pub const EXTERNAL_COMPUTE_QUEUE_NV: Self = Self(1000556000);
	pub const INDIRECT_COMMANDS_LAYOUT_EXT: Self = Self(1000572000);
	pub const INDIRECT_EXECUTION_SET_EXT: Self = Self(1000572001);
	pub const SHADER_INSTRUMENTATION_ARM: Self = Self(1000607000);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct RayTracingInvocationReorderModeEXT(pub i32);
impl RayTracingInvocationReorderModeEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const NONE_EXT: Self = Self(0);
	pub const REORDER_EXT: Self = Self(1);
	pub const NONE_NV: Self = Self(0);
	pub const REORDER_NV: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct RayTracingLssIndexingModeNV(pub i32);
impl RayTracingLssIndexingModeNV {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const LIST_NV: Self = Self(0);
	pub const SUCCESSIVE_NV: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct RayTracingLssPrimitiveEndCapsModeNV(pub i32);
impl RayTracingLssPrimitiveEndCapsModeNV {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const NONE_NV: Self = Self(0);
	pub const CHAINED_NV: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DirectDriverLoadingModeLUNARG(pub i32);
impl DirectDriverLoadingModeLUNARG {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DIRECT_DRIVER_LOADING_MODE_EXCLUSIVE_LUNARG: Self = Self(0);
	pub const DIRECT_DRIVER_LOADING_MODE_INCLUSIVE_LUNARG: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AntiLagModeAMD(pub i32);
impl AntiLagModeAMD {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DRIVER_CONTROL_AMD: Self = Self(0);
	pub const ON_AMD: Self = Self(1);
	pub const OFF_AMD: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AntiLagStageAMD(pub i32);
impl AntiLagStageAMD {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const INPUT_AMD: Self = Self(0);
	pub const PRESENT_AMD: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct QueueFlagBits(pub u32);
impl QueueFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const GRAPHICS_BIT: Self = Self(1);
	pub const COMPUTE_BIT: Self = Self(2);
	pub const TRANSFER_BIT: Self = Self(4);
	pub const SPARSE_BINDING_BIT: Self = Self(8);
	pub const PROTECTED_BIT: Self = Self(16);
	pub const VIDEO_DECODE_BIT_KHR: Self = Self(32);
	pub const VIDEO_ENCODE_BIT_KHR: Self = Self(64);
	pub const OPTICAL_FLOW_BIT_NV: Self = Self(256);
	pub const DATA_GRAPH_BIT_ARM: Self = Self(1024);
}

impl std::ops::BitOr for QueueFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for QueueFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for QueueFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for QueueFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CullModeFlagBits(pub u32);
impl CullModeFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const NONE: Self = Self(0);
	pub const FRONT_BIT: Self = Self(1);
	pub const BACK_BIT: Self = Self(2);
	pub const FRONT_AND_BACK: Self = Self(3);
}

impl std::ops::BitOr for CullModeFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for CullModeFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for CullModeFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for CullModeFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct RenderPassCreateFlagBits(pub u32);
impl RenderPassCreateFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const TRANSFORM_BIT_QCOM: Self = Self(2);
	pub const PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE: Self = Self(4);
}

impl std::ops::BitOr for RenderPassCreateFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for RenderPassCreateFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for RenderPassCreateFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for RenderPassCreateFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DeviceQueueCreateFlagBits(pub u32);
impl DeviceQueueCreateFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const PROTECTED_BIT: Self = Self(1);
	pub const INTERNALLY_SYNCHRONIZED_BIT_KHR: Self = Self(4);
}

impl std::ops::BitOr for DeviceQueueCreateFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for DeviceQueueCreateFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for DeviceQueueCreateFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for DeviceQueueCreateFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct MemoryPropertyFlagBits(pub u32);
impl MemoryPropertyFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEVICE_LOCAL_BIT: Self = Self(1);
	pub const HOST_VISIBLE_BIT: Self = Self(2);
	pub const HOST_COHERENT_BIT: Self = Self(4);
	pub const HOST_CACHED_BIT: Self = Self(8);
	pub const LAZILY_ALLOCATED_BIT: Self = Self(16);
	pub const PROTECTED_BIT: Self = Self(32);
	pub const DEVICE_COHERENT_BIT_AMD: Self = Self(64);
	pub const DEVICE_UNCACHED_BIT_AMD: Self = Self(128);
	pub const RDMA_CAPABLE_BIT_NV: Self = Self(256);
}

impl std::ops::BitOr for MemoryPropertyFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for MemoryPropertyFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for MemoryPropertyFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for MemoryPropertyFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct MemoryHeapFlagBits(pub u32);
impl MemoryHeapFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEVICE_LOCAL_BIT: Self = Self(1);
	pub const MULTI_INSTANCE_BIT: Self = Self(2);
	pub const MULTI_INSTANCE_BIT_KHR: Self = Self(2);
	pub const TILE_MEMORY_BIT_QCOM: Self = Self(8);
}

impl std::ops::BitOr for MemoryHeapFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for MemoryHeapFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for MemoryHeapFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for MemoryHeapFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AccessFlagBits(pub u32);
impl AccessFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const INDIRECT_COMMAND_READ_BIT: Self = Self(1);
	pub const INDEX_READ_BIT: Self = Self(2);
	pub const VERTEX_ATTRIBUTE_READ_BIT: Self = Self(4);
	pub const UNIFORM_READ_BIT: Self = Self(8);
	pub const INPUT_ATTACHMENT_READ_BIT: Self = Self(16);
	pub const SHADER_READ_BIT: Self = Self(32);
	pub const SHADER_WRITE_BIT: Self = Self(64);
	pub const COLOR_ATTACHMENT_READ_BIT: Self = Self(128);
	pub const COLOR_ATTACHMENT_WRITE_BIT: Self = Self(256);
	pub const DEPTH_STENCIL_ATTACHMENT_READ_BIT: Self = Self(512);
	pub const DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: Self = Self(1024);
	pub const TRANSFER_READ_BIT: Self = Self(2048);
	pub const TRANSFER_WRITE_BIT: Self = Self(4096);
	pub const HOST_READ_BIT: Self = Self(8192);
	pub const HOST_WRITE_BIT: Self = Self(16384);
	pub const MEMORY_READ_BIT: Self = Self(32768);
	pub const MEMORY_WRITE_BIT: Self = Self(65536);
	pub const NONE: Self = Self(0);
	pub const TRANSFORM_FEEDBACK_WRITE_BIT_EXT: Self = Self(33554432);
	pub const TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT: Self = Self(67108864);
	pub const TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT: Self = Self(134217728);
	pub const CONDITIONAL_RENDERING_READ_BIT_EXT: Self = Self(1048576);
	pub const COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT: Self = Self(524288);
	pub const ACCELERATION_STRUCTURE_READ_BIT_KHR: Self = Self(2097152);
	pub const ACCELERATION_STRUCTURE_WRITE_BIT_KHR: Self = Self(4194304);
	pub const SHADING_RATE_IMAGE_READ_BIT_NV: Self = Self(8388608);
	pub const ACCELERATION_STRUCTURE_READ_BIT_NV: Self = Self(2097152);
	pub const ACCELERATION_STRUCTURE_WRITE_BIT_NV: Self = Self(4194304);
	pub const FRAGMENT_DENSITY_MAP_READ_BIT_EXT: Self = Self(16777216);
	pub const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR: Self = Self(8388608);
	pub const COMMAND_PREPROCESS_READ_BIT_NV: Self = Self(131072);
	pub const COMMAND_PREPROCESS_WRITE_BIT_NV: Self = Self(262144);
	pub const NONE_KHR: Self = Self(0);
	pub const COMMAND_PREPROCESS_READ_BIT_EXT: Self = Self(131072);
	pub const COMMAND_PREPROCESS_WRITE_BIT_EXT: Self = Self(262144);
}

impl std::ops::BitOr for AccessFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for AccessFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for AccessFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for AccessFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BufferUsageFlagBits(pub u32);
impl BufferUsageFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const TRANSFER_SRC_BIT: Self = Self(1);
	pub const TRANSFER_DST_BIT: Self = Self(2);
	pub const UNIFORM_TEXEL_BUFFER_BIT: Self = Self(4);
	pub const STORAGE_TEXEL_BUFFER_BIT: Self = Self(8);
	pub const UNIFORM_BUFFER_BIT: Self = Self(16);
	pub const STORAGE_BUFFER_BIT: Self = Self(32);
	pub const INDEX_BUFFER_BIT: Self = Self(64);
	pub const VERTEX_BUFFER_BIT: Self = Self(128);
	pub const INDIRECT_BUFFER_BIT: Self = Self(256);
	pub const SHADER_DEVICE_ADDRESS_BIT: Self = Self(131072);
	pub const VIDEO_DECODE_SRC_BIT_KHR: Self = Self(8192);
	pub const VIDEO_DECODE_DST_BIT_KHR: Self = Self(16384);
	pub const TRANSFORM_FEEDBACK_BUFFER_BIT_EXT: Self = Self(2048);
	pub const TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT: Self = Self(4096);
	pub const CONDITIONAL_RENDERING_BIT_EXT: Self = Self(512);
	#[cfg(feature = "provisional")]
	pub const EXECUTION_GRAPH_SCRATCH_BIT_AMDX: Self = Self(33554432);
	pub const DESCRIPTOR_HEAP_BIT_EXT: Self = Self(268435456);
	pub const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR: Self = Self(524288);
	pub const ACCELERATION_STRUCTURE_STORAGE_BIT_KHR: Self = Self(1048576);
	pub const SHADER_BINDING_TABLE_BIT_KHR: Self = Self(1024);
	pub const RAY_TRACING_BIT_NV: Self = Self(1024);
	pub const SHADER_DEVICE_ADDRESS_BIT_EXT: Self = Self(131072);
	pub const SHADER_DEVICE_ADDRESS_BIT_KHR: Self = Self(131072);
	pub const VIDEO_ENCODE_DST_BIT_KHR: Self = Self(32768);
	pub const VIDEO_ENCODE_SRC_BIT_KHR: Self = Self(65536);
	pub const SAMPLER_DESCRIPTOR_BUFFER_BIT_EXT: Self = Self(2097152);
	pub const RESOURCE_DESCRIPTOR_BUFFER_BIT_EXT: Self = Self(4194304);
	pub const PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_BIT_EXT: Self = Self(67108864);
	pub const MICROMAP_BUILD_INPUT_READ_ONLY_BIT_EXT: Self = Self(8388608);
	pub const MICROMAP_STORAGE_BIT_EXT: Self = Self(16777216);
	pub const TILE_MEMORY_BIT_QCOM: Self = Self(134217728);
}

impl std::ops::BitOr for BufferUsageFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for BufferUsageFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for BufferUsageFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for BufferUsageFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BufferUsageFlagBits2(pub u64);
impl BufferUsageFlagBits2 {
	pub const fn from_raw(value: u64) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u64 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const TRANSFER_SRC_BIT: Self = Self(1);
	pub const TRANSFER_DST_BIT: Self = Self(2);
	pub const UNIFORM_TEXEL_BUFFER_BIT: Self = Self(4);
	pub const STORAGE_TEXEL_BUFFER_BIT: Self = Self(8);
	pub const UNIFORM_BUFFER_BIT: Self = Self(16);
	pub const STORAGE_BUFFER_BIT: Self = Self(32);
	pub const INDEX_BUFFER_BIT: Self = Self(64);
	pub const VERTEX_BUFFER_BIT: Self = Self(128);
	pub const INDIRECT_BUFFER_BIT: Self = Self(256);
	pub const SHADER_DEVICE_ADDRESS_BIT: Self = Self(131072);
	#[cfg(feature = "provisional")]
	pub const EXECUTION_GRAPH_SCRATCH_BIT_AMDX: Self = Self(33554432);
	pub const DESCRIPTOR_HEAP_BIT_EXT: Self = Self(268435456);
	pub const MICROMAP_BUILD_INPUT_READ_ONLY_BIT_EXT: Self = Self(8388608);
	pub const MICROMAP_STORAGE_BIT_EXT: Self = Self(16777216);
	pub const TRANSFER_SRC_BIT_KHR: Self = Self(1);
	pub const TRANSFER_DST_BIT_KHR: Self = Self(2);
	pub const UNIFORM_TEXEL_BUFFER_BIT_KHR: Self = Self(4);
	pub const STORAGE_TEXEL_BUFFER_BIT_KHR: Self = Self(8);
	pub const UNIFORM_BUFFER_BIT_KHR: Self = Self(16);
	pub const STORAGE_BUFFER_BIT_KHR: Self = Self(32);
	pub const INDEX_BUFFER_BIT_KHR: Self = Self(64);
	pub const VERTEX_BUFFER_BIT_KHR: Self = Self(128);
	pub const INDIRECT_BUFFER_BIT_KHR: Self = Self(256);
	pub const CONDITIONAL_RENDERING_BIT_EXT: Self = Self(512);
	pub const SHADER_BINDING_TABLE_BIT_KHR: Self = Self(1024);
	pub const RAY_TRACING_BIT_NV: Self = Self(1024);
	pub const TRANSFORM_FEEDBACK_BUFFER_BIT_EXT: Self = Self(2048);
	pub const TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT: Self = Self(4096);
	pub const VIDEO_DECODE_SRC_BIT_KHR: Self = Self(8192);
	pub const VIDEO_DECODE_DST_BIT_KHR: Self = Self(16384);
	pub const VIDEO_ENCODE_DST_BIT_KHR: Self = Self(32768);
	pub const VIDEO_ENCODE_SRC_BIT_KHR: Self = Self(65536);
	pub const SHADER_DEVICE_ADDRESS_BIT_KHR: Self = Self(131072);
	pub const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR: Self = Self(524288);
	pub const ACCELERATION_STRUCTURE_STORAGE_BIT_KHR: Self = Self(1048576);
	pub const SAMPLER_DESCRIPTOR_BUFFER_BIT_EXT: Self = Self(2097152);
	pub const RESOURCE_DESCRIPTOR_BUFFER_BIT_EXT: Self = Self(4194304);
	pub const PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_BIT_EXT: Self = Self(67108864);
	#[cfg(feature = "provisional")]
	pub const COMPRESSED_DATA_DGF1_BIT_AMDX: Self = Self(8589934592);
	pub const DATA_GRAPH_FOREIGN_DESCRIPTOR_BIT_ARM: Self = Self(536870912);
	pub const TILE_MEMORY_BIT_QCOM: Self = Self(134217728);
	pub const MEMORY_DECOMPRESSION_BIT_EXT: Self = Self(4294967296);
	pub const PREPROCESS_BUFFER_BIT_EXT: Self = Self(2147483648);
}

impl std::ops::BitOr for BufferUsageFlagBits2 {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for BufferUsageFlagBits2 {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for BufferUsageFlagBits2 {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for BufferUsageFlagBits2 {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BufferCreateFlagBits(pub u32);
impl BufferCreateFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const SPARSE_BINDING_BIT: Self = Self(1);
	pub const SPARSE_RESIDENCY_BIT: Self = Self(2);
	pub const SPARSE_ALIASED_BIT: Self = Self(4);
	pub const PROTECTED_BIT: Self = Self(8);
	pub const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT: Self = Self(16);
	pub const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT: Self = Self(16);
	pub const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR: Self = Self(16);
	pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT: Self = Self(32);
	pub const VIDEO_PROFILE_INDEPENDENT_BIT_KHR: Self = Self(64);
}

impl std::ops::BitOr for BufferCreateFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for BufferCreateFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for BufferCreateFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for BufferCreateFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ShaderStageFlagBits(pub u32);
impl ShaderStageFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const VERTEX_BIT: Self = Self(1);
	pub const TESSELLATION_CONTROL_BIT: Self = Self(2);
	pub const TESSELLATION_EVALUATION_BIT: Self = Self(4);
	pub const GEOMETRY_BIT: Self = Self(8);
	pub const FRAGMENT_BIT: Self = Self(16);
	pub const COMPUTE_BIT: Self = Self(32);
	pub const ALL_GRAPHICS: Self = Self(31);
	pub const ALL: Self = Self(2147483647);
	pub const RAYGEN_BIT_KHR: Self = Self(256);
	pub const ANY_HIT_BIT_KHR: Self = Self(512);
	pub const CLOSEST_HIT_BIT_KHR: Self = Self(1024);
	pub const MISS_BIT_KHR: Self = Self(2048);
	pub const INTERSECTION_BIT_KHR: Self = Self(4096);
	pub const CALLABLE_BIT_KHR: Self = Self(8192);
	pub const RAYGEN_BIT_NV: Self = Self(256);
	pub const ANY_HIT_BIT_NV: Self = Self(512);
	pub const CLOSEST_HIT_BIT_NV: Self = Self(1024);
	pub const MISS_BIT_NV: Self = Self(2048);
	pub const INTERSECTION_BIT_NV: Self = Self(4096);
	pub const CALLABLE_BIT_NV: Self = Self(8192);
	pub const TASK_BIT_NV: Self = Self(64);
	pub const MESH_BIT_NV: Self = Self(128);
	pub const TASK_BIT_EXT: Self = Self(64);
	pub const MESH_BIT_EXT: Self = Self(128);
	pub const SUBPASS_SHADING_BIT_HUAWEI: Self = Self(16384);
	pub const CLUSTER_CULLING_BIT_HUAWEI: Self = Self(524288);
}

impl std::ops::BitOr for ShaderStageFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ShaderStageFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ShaderStageFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ShaderStageFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ImageUsageFlagBits(pub u32);
impl ImageUsageFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const TRANSFER_SRC_BIT: Self = Self(1);
	pub const TRANSFER_DST_BIT: Self = Self(2);
	pub const SAMPLED_BIT: Self = Self(4);
	pub const STORAGE_BIT: Self = Self(8);
	pub const COLOR_ATTACHMENT_BIT: Self = Self(16);
	pub const DEPTH_STENCIL_ATTACHMENT_BIT: Self = Self(32);
	pub const TRANSIENT_ATTACHMENT_BIT: Self = Self(64);
	pub const INPUT_ATTACHMENT_BIT: Self = Self(128);
	pub const HOST_TRANSFER_BIT: Self = Self(4194304);
	pub const VIDEO_DECODE_DST_BIT_KHR: Self = Self(1024);
	pub const VIDEO_DECODE_SRC_BIT_KHR: Self = Self(2048);
	pub const VIDEO_DECODE_DPB_BIT_KHR: Self = Self(4096);
	pub const SHADING_RATE_IMAGE_BIT_NV: Self = Self(256);
	pub const FRAGMENT_DENSITY_MAP_BIT_EXT: Self = Self(512);
	pub const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: Self = Self(256);
	pub const HOST_TRANSFER_BIT_EXT: Self = Self(4194304);
	pub const VIDEO_ENCODE_DST_BIT_KHR: Self = Self(8192);
	pub const VIDEO_ENCODE_SRC_BIT_KHR: Self = Self(16384);
	pub const VIDEO_ENCODE_DPB_BIT_KHR: Self = Self(32768);
	pub const ATTACHMENT_FEEDBACK_LOOP_BIT_EXT: Self = Self(524288);
	pub const INVOCATION_MASK_BIT_HUAWEI: Self = Self(262144);
	pub const SAMPLE_WEIGHT_BIT_QCOM: Self = Self(1048576);
	pub const SAMPLE_BLOCK_MATCH_BIT_QCOM: Self = Self(2097152);
	pub const TENSOR_ALIASING_BIT_ARM: Self = Self(8388608);
	pub const TILE_MEMORY_BIT_QCOM: Self = Self(134217728);
	pub const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR: Self = Self(33554432);
	pub const VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR: Self = Self(67108864);
}

impl std::ops::BitOr for ImageUsageFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ImageUsageFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ImageUsageFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ImageUsageFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ImageUsageFlagBits2KHR(pub u64);
impl ImageUsageFlagBits2KHR {
	pub const fn from_raw(value: u64) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u64 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const TRANSFER_SRC_BIT_KHR: Self = Self(1);
	pub const TRANSFER_DST_BIT_KHR: Self = Self(2);
	pub const SAMPLED_BIT_KHR: Self = Self(4);
	pub const STORAGE_BIT_KHR: Self = Self(8);
	pub const COLOR_ATTACHMENT_BIT_KHR: Self = Self(16);
	pub const DEPTH_STENCIL_ATTACHMENT_BIT_KHR: Self = Self(32);
	pub const TRANSIENT_ATTACHMENT_BIT_KHR: Self = Self(64);
	pub const INPUT_ATTACHMENT_BIT_KHR: Self = Self(128);
	pub const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: Self = Self(256);
	pub const FRAGMENT_DENSITY_MAP_BIT_EXT: Self = Self(512);
	pub const VIDEO_DECODE_DST_BIT_KHR: Self = Self(1024);
	pub const VIDEO_DECODE_SRC_BIT_KHR: Self = Self(2048);
	pub const VIDEO_DECODE_DPB_BIT_KHR: Self = Self(4096);
	pub const VIDEO_ENCODE_DST_BIT_KHR: Self = Self(8192);
	pub const VIDEO_ENCODE_SRC_BIT_KHR: Self = Self(16384);
	pub const VIDEO_ENCODE_DPB_BIT_KHR: Self = Self(32768);
	pub const INVOCATION_MASK_BIT_HUAWEI: Self = Self(262144);
	pub const ATTACHMENT_FEEDBACK_LOOP_BIT_EXT: Self = Self(524288);
	pub const SAMPLE_WEIGHT_BIT_QCOM: Self = Self(1048576);
	pub const SAMPLE_BLOCK_MATCH_BIT_QCOM: Self = Self(2097152);
	pub const HOST_TRANSFER_BIT_KHR: Self = Self(4194304);
	pub const TENSOR_ALIASING_BIT_ARM: Self = Self(8388608);
	pub const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR: Self = Self(33554432);
	pub const VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR: Self = Self(67108864);
	pub const TILE_MEMORY_BIT_QCOM: Self = Self(134217728);
}

impl std::ops::BitOr for ImageUsageFlagBits2KHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ImageUsageFlagBits2KHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ImageUsageFlagBits2KHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ImageUsageFlagBits2KHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ImageCreateFlagBits(pub u32);
impl ImageCreateFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const SPARSE_BINDING_BIT: Self = Self(1);
	pub const SPARSE_RESIDENCY_BIT: Self = Self(2);
	pub const SPARSE_ALIASED_BIT: Self = Self(4);
	pub const MUTABLE_FORMAT_BIT: Self = Self(8);
	pub const CUBE_COMPATIBLE_BIT: Self = Self(16);
	pub const ALIAS_BIT: Self = Self(1024);
	pub const SPLIT_INSTANCE_BIND_REGIONS_BIT: Self = Self(64);
	pub const _2D_ARRAY_COMPATIBLE_BIT: Self = Self(32);
	pub const BLOCK_TEXEL_VIEW_COMPATIBLE_BIT: Self = Self(128);
	pub const EXTENDED_USAGE_BIT: Self = Self(256);
	pub const PROTECTED_BIT: Self = Self(2048);
	pub const DISJOINT_BIT: Self = Self(512);
	pub const CORNER_SAMPLED_BIT_NV: Self = Self(8192);
	pub const SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR: Self = Self(64);
	pub const _2D_ARRAY_COMPATIBLE_BIT_KHR: Self = Self(32);
	pub const BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR: Self = Self(128);
	pub const EXTENDED_USAGE_BIT_KHR: Self = Self(256);
	pub const DESCRIPTOR_HEAP_CAPTURE_REPLAY_BIT_EXT: Self = Self(65536);
	pub const SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT: Self = Self(4096);
	pub const DISJOINT_BIT_KHR: Self = Self(512);
	pub const ALIAS_BIT_KHR: Self = Self(1024);
	pub const SUBSAMPLED_BIT_EXT: Self = Self(16384);
	pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT: Self = Self(65536);
	pub const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT: Self = Self(262144);
	pub const _2D_VIEW_COMPATIBLE_BIT_EXT: Self = Self(131072);
	pub const FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM: Self = Self(32768);
	pub const VIDEO_PROFILE_INDEPENDENT_BIT_KHR: Self = Self(1048576);
	pub const FRAGMENT_DENSITY_MAP_OFFSET_BIT_EXT: Self = Self(32768);
	pub const ALIAS_SINGLE_LAYER_DESCRIPTOR_BIT_KHR: Self = Self(4194304);
}

impl std::ops::BitOr for ImageCreateFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ImageCreateFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ImageCreateFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ImageCreateFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ImageCreateFlagBits2KHR(pub u64);
impl ImageCreateFlagBits2KHR {
	pub const fn from_raw(value: u64) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u64 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const SPARSE_BINDING_BIT_KHR: Self = Self(1);
	pub const SPARSE_RESIDENCY_BIT_KHR: Self = Self(2);
	pub const SPARSE_ALIASED_BIT_KHR: Self = Self(4);
	pub const MUTABLE_FORMAT_BIT_KHR: Self = Self(8);
	pub const CUBE_COMPATIBLE_BIT_KHR: Self = Self(16);
	pub const ALIAS_SINGLE_LAYER_DESCRIPTOR_BIT_KHR: Self = Self(4194304);
	pub const _2D_ARRAY_COMPATIBLE_BIT_KHR: Self = Self(32);
	pub const SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR: Self = Self(64);
	pub const BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR: Self = Self(128);
	pub const EXTENDED_USAGE_BIT_KHR: Self = Self(256);
	pub const DISJOINT_BIT_KHR: Self = Self(512);
	pub const ALIAS_BIT_KHR: Self = Self(1024);
	pub const PROTECTED_BIT_KHR: Self = Self(2048);
	pub const SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT: Self = Self(4096);
	pub const CORNER_SAMPLED_BIT_NV: Self = Self(8192);
	pub const SUBSAMPLED_BIT_EXT: Self = Self(16384);
	pub const FRAGMENT_DENSITY_MAP_OFFSET_BIT_EXT: Self = Self(32768);
	pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT: Self = Self(65536);
	pub const _2D_VIEW_COMPATIBLE_BIT_EXT: Self = Self(131072);
	pub const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT: Self = Self(262144);
	pub const VIDEO_PROFILE_INDEPENDENT_BIT_KHR: Self = Self(1048576);
}

impl std::ops::BitOr for ImageCreateFlagBits2KHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ImageCreateFlagBits2KHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ImageCreateFlagBits2KHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ImageCreateFlagBits2KHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ImageViewCreateFlagBits(pub u32);
impl ImageViewCreateFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT: Self = Self(1);
	pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT: Self = Self(4);
	pub const FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT: Self = Self(2);
}

impl std::ops::BitOr for ImageViewCreateFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ImageViewCreateFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ImageViewCreateFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ImageViewCreateFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SamplerCreateFlagBits(pub u32);
impl SamplerCreateFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const SUBSAMPLED_BIT_EXT: Self = Self(1);
	pub const SUBSAMPLED_COARSE_RECONSTRUCTION_BIT_EXT: Self = Self(2);
	pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT: Self = Self(8);
	pub const NON_SEAMLESS_CUBE_MAP_BIT_EXT: Self = Self(4);
	pub const IMAGE_PROCESSING_BIT_QCOM: Self = Self(16);
}

impl std::ops::BitOr for SamplerCreateFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for SamplerCreateFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for SamplerCreateFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for SamplerCreateFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PipelineCreateFlagBits(pub u32);
impl PipelineCreateFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DISABLE_OPTIMIZATION_BIT: Self = Self(1);
	pub const ALLOW_DERIVATIVES_BIT: Self = Self(2);
	pub const DERIVATIVE_BIT: Self = Self(4);
	pub const DISPATCH_BASE_BIT: Self = Self(16);
	pub const DISPATCH_BASE: Self = Self(16);
	pub const VIEW_INDEX_FROM_DEVICE_INDEX_BIT: Self = Self(8);
	pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT: Self = Self(256);
	pub const EARLY_RETURN_ON_FAILURE_BIT: Self = Self(512);
	pub const NO_PROTECTED_ACCESS_BIT: Self = Self(134217728);
	pub const PROTECTED_ACCESS_ONLY_BIT: Self = Self(1073741824);
	pub const VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR: Self = Self(8);
	pub const DISPATCH_BASE_BIT_KHR: Self = Self(16);
	pub const DISPATCH_BASE_KHR: Self = Self(16);
	pub const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR: Self = Self(16384);
	pub const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR: Self = Self(32768);
	pub const RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR: Self = Self(65536);
	pub const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR: Self = Self(131072);
	pub const RAY_TRACING_SKIP_TRIANGLES_BIT_KHR: Self = Self(4096);
	pub const RAY_TRACING_SKIP_AABBS_BIT_KHR: Self = Self(8192);
	pub const RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR: Self = Self(524288);
	pub const DEFER_COMPILE_BIT_NV: Self = Self(32);
	pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT: Self = Self(4194304);
	pub const PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT: Self =
		Self(4194304);
	pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: Self = Self(2097152);
	pub const PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: Self =
		Self(2097152);
	pub const CAPTURE_STATISTICS_BIT_KHR: Self = Self(64);
	pub const CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR: Self = Self(128);
	pub const INDIRECT_BINDABLE_BIT_NV: Self = Self(262144);
	pub const LIBRARY_BIT_KHR: Self = Self(2048);
	pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT_EXT: Self = Self(256);
	pub const EARLY_RETURN_ON_FAILURE_BIT_EXT: Self = Self(512);
	pub const DESCRIPTOR_BUFFER_BIT_EXT: Self = Self(536870912);
	pub const RETAIN_LINK_TIME_OPTIMIZATION_INFO_BIT_EXT: Self = Self(8388608);
	pub const LINK_TIME_OPTIMIZATION_BIT_EXT: Self = Self(1024);
	pub const RAY_TRACING_ALLOW_MOTION_BIT_NV: Self = Self(1048576);
	pub const COLOR_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT: Self = Self(33554432);
	pub const DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT: Self = Self(67108864);
	pub const RAY_TRACING_OPACITY_MICROMAP_BIT_EXT: Self = Self(16777216);
	#[cfg(feature = "provisional")]
	pub const RAY_TRACING_DISPLACEMENT_MICROMAP_BIT_NV: Self = Self(268435456);
	pub const NO_PROTECTED_ACCESS_BIT_EXT: Self = Self(134217728);
	pub const PROTECTED_ACCESS_ONLY_BIT_EXT: Self = Self(1073741824);
	pub const RAY_TRACING_OPACITY_MICROMAP_BIT_KHR: Self = Self(16777216);
}

impl std::ops::BitOr for PipelineCreateFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for PipelineCreateFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for PipelineCreateFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for PipelineCreateFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PipelineCreateFlagBits2(pub u64);
impl PipelineCreateFlagBits2 {
	pub const fn from_raw(value: u64) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u64 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DISABLE_OPTIMIZATION_BIT: Self = Self(1);
	pub const ALLOW_DERIVATIVES_BIT: Self = Self(2);
	pub const DERIVATIVE_BIT: Self = Self(4);
	pub const VIEW_INDEX_FROM_DEVICE_INDEX_BIT: Self = Self(8);
	pub const DISPATCH_BASE_BIT: Self = Self(16);
	pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT: Self = Self(256);
	pub const EARLY_RETURN_ON_FAILURE_BIT: Self = Self(512);
	pub const NO_PROTECTED_ACCESS_BIT: Self = Self(134217728);
	pub const PROTECTED_ACCESS_ONLY_BIT: Self = Self(1073741824);
	#[cfg(feature = "provisional")]
	pub const EXECUTION_GRAPH_BIT_AMDX: Self = Self(4294967296);
	pub const DESCRIPTOR_HEAP_BIT_EXT: Self = Self(68719476736);
	pub const RAY_TRACING_SKIP_BUILT_IN_PRIMITIVES_BIT_KHR: Self = Self(4096);
	pub const RAY_TRACING_OPACITY_MICROMAP_BIT_EXT: Self = Self(16777216);
	pub const RAY_TRACING_ALLOW_SPHERES_AND_LINEAR_SWEPT_SPHERES_BIT_NV: Self = Self(8589934592);
	pub const ENABLE_LEGACY_DITHERING_BIT_EXT: Self = Self(17179869184);
	pub const DISABLE_OPTIMIZATION_BIT_KHR: Self = Self(1);
	pub const ALLOW_DERIVATIVES_BIT_KHR: Self = Self(2);
	pub const DERIVATIVE_BIT_KHR: Self = Self(4);
	pub const VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR: Self = Self(8);
	pub const DISPATCH_BASE_BIT_KHR: Self = Self(16);
	pub const DEFER_COMPILE_BIT_NV: Self = Self(32);
	pub const CAPTURE_STATISTICS_BIT_KHR: Self = Self(64);
	pub const CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR: Self = Self(128);
	pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT_KHR: Self = Self(256);
	pub const EARLY_RETURN_ON_FAILURE_BIT_KHR: Self = Self(512);
	pub const LINK_TIME_OPTIMIZATION_BIT_EXT: Self = Self(1024);
	pub const RETAIN_LINK_TIME_OPTIMIZATION_INFO_BIT_EXT: Self = Self(8388608);
	pub const LIBRARY_BIT_KHR: Self = Self(2048);
	pub const RAY_TRACING_SKIP_TRIANGLES_BIT_KHR: Self = Self(4096);
	pub const RAY_TRACING_SKIP_AABBS_BIT_KHR: Self = Self(8192);
	pub const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR: Self = Self(16384);
	pub const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR: Self = Self(32768);
	pub const RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR: Self = Self(65536);
	pub const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR: Self = Self(131072);
	pub const RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR: Self = Self(524288);
	pub const INDIRECT_BINDABLE_BIT_NV: Self = Self(262144);
	pub const RAY_TRACING_ALLOW_MOTION_BIT_NV: Self = Self(1048576);
	pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: Self = Self(2097152);
	pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT: Self = Self(4194304);
	pub const COLOR_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT: Self = Self(33554432);
	pub const DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT: Self = Self(67108864);
	pub const NO_PROTECTED_ACCESS_BIT_EXT: Self = Self(134217728);
	pub const PROTECTED_ACCESS_ONLY_BIT_EXT: Self = Self(1073741824);
	#[cfg(feature = "provisional")]
	pub const RAY_TRACING_DISPLACEMENT_MICROMAP_BIT_NV: Self = Self(268435456);
	pub const DESCRIPTOR_BUFFER_BIT_EXT: Self = Self(536870912);
	pub const DISALLOW_OPACITY_MICROMAP_BIT_ARM: Self = Self(137438953472);
	pub const INSTRUMENT_SHADERS_BIT_ARM: Self = Self(549755813888);
	pub const CAPTURE_DATA_BIT_KHR: Self = Self(2147483648);
	pub const INDIRECT_BINDABLE_BIT_EXT: Self = Self(274877906944);
	pub const PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE: Self = Self(1099511627776);
	pub const RAY_TRACING_OPACITY_MICROMAP_BIT_KHR: Self = Self(16777216);
	pub const OPACITY_MICROMAP_DISALLOW_MIXED_SPECIAL_INDEX_BIT_KHR: Self = Self(2199023255552);
	pub const _64_BIT_INDEXING_BIT_EXT: Self = Self(8796093022208);
}

impl std::ops::BitOr for PipelineCreateFlagBits2 {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for PipelineCreateFlagBits2 {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for PipelineCreateFlagBits2 {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for PipelineCreateFlagBits2 {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PipelineShaderStageCreateFlagBits(pub u32);
impl PipelineShaderStageCreateFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ALLOW_VARYING_SUBGROUP_SIZE_BIT: Self = Self(1);
	pub const REQUIRE_FULL_SUBGROUPS_BIT: Self = Self(2);
	pub const ALLOW_VARYING_SUBGROUP_SIZE_BIT_EXT: Self = Self(1);
	pub const REQUIRE_FULL_SUBGROUPS_BIT_EXT: Self = Self(2);
}

impl std::ops::BitOr for PipelineShaderStageCreateFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for PipelineShaderStageCreateFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for PipelineShaderStageCreateFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for PipelineShaderStageCreateFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ColorComponentFlagBits(pub u32);
impl ColorComponentFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const R_BIT: Self = Self(1);
	pub const G_BIT: Self = Self(2);
	pub const B_BIT: Self = Self(4);
	pub const A_BIT: Self = Self(8);
}

impl std::ops::BitOr for ColorComponentFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ColorComponentFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ColorComponentFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ColorComponentFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FenceCreateFlagBits(pub u32);
impl FenceCreateFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const SIGNALED_BIT: Self = Self(1);
}

impl std::ops::BitOr for FenceCreateFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for FenceCreateFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for FenceCreateFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for FenceCreateFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FormatFeatureFlagBits(pub u32);
impl FormatFeatureFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const SAMPLED_IMAGE_BIT: Self = Self(1);
	pub const STORAGE_IMAGE_BIT: Self = Self(2);
	pub const STORAGE_IMAGE_ATOMIC_BIT: Self = Self(4);
	pub const UNIFORM_TEXEL_BUFFER_BIT: Self = Self(8);
	pub const STORAGE_TEXEL_BUFFER_BIT: Self = Self(16);
	pub const STORAGE_TEXEL_BUFFER_ATOMIC_BIT: Self = Self(32);
	pub const VERTEX_BUFFER_BIT: Self = Self(64);
	pub const COLOR_ATTACHMENT_BIT: Self = Self(128);
	pub const COLOR_ATTACHMENT_BLEND_BIT: Self = Self(256);
	pub const DEPTH_STENCIL_ATTACHMENT_BIT: Self = Self(512);
	pub const BLIT_SRC_BIT: Self = Self(1024);
	pub const BLIT_DST_BIT: Self = Self(2048);
	pub const SAMPLED_IMAGE_FILTER_LINEAR_BIT: Self = Self(4096);
	pub const TRANSFER_SRC_BIT: Self = Self(16384);
	pub const TRANSFER_DST_BIT: Self = Self(32768);
	pub const MIDPOINT_CHROMA_SAMPLES_BIT: Self = Self(131072);
	pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT: Self = Self(262144);
	pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT: Self = Self(524288);
	pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT: Self = Self(1048576);
	pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT: Self =
		Self(2097152);
	pub const DISJOINT_BIT: Self = Self(4194304);
	pub const COSITED_CHROMA_SAMPLES_BIT: Self = Self(8388608);
	pub const SAMPLED_IMAGE_FILTER_MINMAX_BIT: Self = Self(65536);
	pub const SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG: Self = Self(8192);
	pub const VIDEO_DECODE_OUTPUT_BIT_KHR: Self = Self(33554432);
	pub const VIDEO_DECODE_DPB_BIT_KHR: Self = Self(67108864);
	pub const TRANSFER_SRC_BIT_KHR: Self = Self(16384);
	pub const TRANSFER_DST_BIT_KHR: Self = Self(32768);
	pub const SAMPLED_IMAGE_FILTER_MINMAX_BIT_EXT: Self = Self(65536);
	pub const ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR: Self = Self(536870912);
	pub const MIDPOINT_CHROMA_SAMPLES_BIT_KHR: Self = Self(131072);
	pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR: Self = Self(262144);
	pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR: Self =
		Self(524288);
	pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR: Self =
		Self(1048576);
	pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR: Self =
		Self(2097152);
	pub const DISJOINT_BIT_KHR: Self = Self(4194304);
	pub const COSITED_CHROMA_SAMPLES_BIT_KHR: Self = Self(8388608);
	pub const SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT: Self = Self(8192);
	pub const FRAGMENT_DENSITY_MAP_BIT_EXT: Self = Self(16777216);
	pub const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: Self = Self(1073741824);
	pub const VIDEO_ENCODE_INPUT_BIT_KHR: Self = Self(134217728);
	pub const VIDEO_ENCODE_DPB_BIT_KHR: Self = Self(268435456);
}

impl std::ops::BitOr for FormatFeatureFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for FormatFeatureFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for FormatFeatureFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for FormatFeatureFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct QueryControlFlagBits(pub u32);
impl QueryControlFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const PRECISE_BIT: Self = Self(1);
}

impl std::ops::BitOr for QueryControlFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for QueryControlFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for QueryControlFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for QueryControlFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct QueryResultFlagBits(pub u32);
impl QueryResultFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const _64_BIT: Self = Self(1);
	pub const WAIT_BIT: Self = Self(2);
	pub const WITH_AVAILABILITY_BIT: Self = Self(4);
	pub const PARTIAL_BIT: Self = Self(8);
	pub const WITH_STATUS_BIT_KHR: Self = Self(16);
}

impl std::ops::BitOr for QueryResultFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for QueryResultFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for QueryResultFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for QueryResultFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CommandBufferUsageFlagBits(pub u32);
impl CommandBufferUsageFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ONE_TIME_SUBMIT_BIT: Self = Self(1);
	pub const RENDER_PASS_CONTINUE_BIT: Self = Self(2);
	pub const SIMULTANEOUS_USE_BIT: Self = Self(4);
}

impl std::ops::BitOr for CommandBufferUsageFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for CommandBufferUsageFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for CommandBufferUsageFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for CommandBufferUsageFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct QueryPipelineStatisticFlagBits(pub u32);
impl QueryPipelineStatisticFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const INPUT_ASSEMBLY_VERTICES_BIT: Self = Self(1);
	pub const INPUT_ASSEMBLY_PRIMITIVES_BIT: Self = Self(2);
	pub const VERTEX_SHADER_INVOCATIONS_BIT: Self = Self(4);
	pub const GEOMETRY_SHADER_INVOCATIONS_BIT: Self = Self(8);
	pub const GEOMETRY_SHADER_PRIMITIVES_BIT: Self = Self(16);
	pub const CLIPPING_INVOCATIONS_BIT: Self = Self(32);
	pub const CLIPPING_PRIMITIVES_BIT: Self = Self(64);
	pub const FRAGMENT_SHADER_INVOCATIONS_BIT: Self = Self(128);
	pub const TESSELLATION_CONTROL_SHADER_PATCHES_BIT: Self = Self(256);
	pub const TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT: Self = Self(512);
	pub const COMPUTE_SHADER_INVOCATIONS_BIT: Self = Self(1024);
	pub const TASK_SHADER_INVOCATIONS_BIT_EXT: Self = Self(2048);
	pub const MESH_SHADER_INVOCATIONS_BIT_EXT: Self = Self(4096);
	pub const CLUSTER_CULLING_SHADER_INVOCATIONS_BIT_HUAWEI: Self = Self(8192);
}

impl std::ops::BitOr for QueryPipelineStatisticFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for QueryPipelineStatisticFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for QueryPipelineStatisticFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for QueryPipelineStatisticFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct MemoryMapFlagBits(pub u32);
impl MemoryMapFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const PLACED_BIT_EXT: Self = Self(1);
}

impl std::ops::BitOr for MemoryMapFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for MemoryMapFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for MemoryMapFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for MemoryMapFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ImageAspectFlagBits(pub u32);
impl ImageAspectFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const COLOR_BIT: Self = Self(1);
	pub const DEPTH_BIT: Self = Self(2);
	pub const STENCIL_BIT: Self = Self(4);
	pub const METADATA_BIT: Self = Self(8);
	pub const PLANE_0_BIT: Self = Self(16);
	pub const PLANE_1_BIT: Self = Self(32);
	pub const PLANE_2_BIT: Self = Self(64);
	pub const NONE: Self = Self(0);
	pub const PLANE_0_BIT_KHR: Self = Self(16);
	pub const PLANE_1_BIT_KHR: Self = Self(32);
	pub const PLANE_2_BIT_KHR: Self = Self(64);
	pub const MEMORY_PLANE_0_BIT_EXT: Self = Self(128);
	pub const MEMORY_PLANE_1_BIT_EXT: Self = Self(256);
	pub const MEMORY_PLANE_2_BIT_EXT: Self = Self(512);
	pub const MEMORY_PLANE_3_BIT_EXT: Self = Self(1024);
	pub const NONE_KHR: Self = Self(0);
}

impl std::ops::BitOr for ImageAspectFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ImageAspectFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ImageAspectFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ImageAspectFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SparseImageFormatFlagBits(pub u32);
impl SparseImageFormatFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const SINGLE_MIPTAIL_BIT: Self = Self(1);
	pub const ALIGNED_MIP_SIZE_BIT: Self = Self(2);
	pub const NONSTANDARD_BLOCK_SIZE_BIT: Self = Self(4);
}

impl std::ops::BitOr for SparseImageFormatFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for SparseImageFormatFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for SparseImageFormatFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for SparseImageFormatFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SparseMemoryBindFlagBits(pub u32);
impl SparseMemoryBindFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const METADATA_BIT: Self = Self(1);
}

impl std::ops::BitOr for SparseMemoryBindFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for SparseMemoryBindFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for SparseMemoryBindFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for SparseMemoryBindFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PipelineStageFlagBits(pub u32);
impl PipelineStageFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const TOP_OF_PIPE_BIT: Self = Self(1);
	pub const DRAW_INDIRECT_BIT: Self = Self(2);
	pub const VERTEX_INPUT_BIT: Self = Self(4);
	pub const VERTEX_SHADER_BIT: Self = Self(8);
	pub const TESSELLATION_CONTROL_SHADER_BIT: Self = Self(16);
	pub const TESSELLATION_EVALUATION_SHADER_BIT: Self = Self(32);
	pub const GEOMETRY_SHADER_BIT: Self = Self(64);
	pub const FRAGMENT_SHADER_BIT: Self = Self(128);
	pub const EARLY_FRAGMENT_TESTS_BIT: Self = Self(256);
	pub const LATE_FRAGMENT_TESTS_BIT: Self = Self(512);
	pub const COLOR_ATTACHMENT_OUTPUT_BIT: Self = Self(1024);
	pub const COMPUTE_SHADER_BIT: Self = Self(2048);
	pub const TRANSFER_BIT: Self = Self(4096);
	pub const BOTTOM_OF_PIPE_BIT: Self = Self(8192);
	pub const HOST_BIT: Self = Self(16384);
	pub const ALL_GRAPHICS_BIT: Self = Self(32768);
	pub const ALL_COMMANDS_BIT: Self = Self(65536);
	pub const NONE: Self = Self(0);
	pub const TRANSFORM_FEEDBACK_BIT_EXT: Self = Self(16777216);
	pub const CONDITIONAL_RENDERING_BIT_EXT: Self = Self(262144);
	pub const ACCELERATION_STRUCTURE_BUILD_BIT_KHR: Self = Self(33554432);
	pub const RAY_TRACING_SHADER_BIT_KHR: Self = Self(2097152);
	pub const SHADING_RATE_IMAGE_BIT_NV: Self = Self(4194304);
	pub const RAY_TRACING_SHADER_BIT_NV: Self = Self(2097152);
	pub const ACCELERATION_STRUCTURE_BUILD_BIT_NV: Self = Self(33554432);
	pub const TASK_SHADER_BIT_NV: Self = Self(524288);
	pub const MESH_SHADER_BIT_NV: Self = Self(1048576);
	pub const FRAGMENT_DENSITY_PROCESS_BIT_EXT: Self = Self(8388608);
	pub const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: Self = Self(4194304);
	pub const COMMAND_PREPROCESS_BIT_NV: Self = Self(131072);
	pub const NONE_KHR: Self = Self(0);
	pub const TASK_SHADER_BIT_EXT: Self = Self(524288);
	pub const MESH_SHADER_BIT_EXT: Self = Self(1048576);
	pub const COMMAND_PREPROCESS_BIT_EXT: Self = Self(131072);
}

impl std::ops::BitOr for PipelineStageFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for PipelineStageFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for PipelineStageFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for PipelineStageFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CommandPoolCreateFlagBits(pub u32);
impl CommandPoolCreateFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const TRANSIENT_BIT: Self = Self(1);
	pub const RESET_COMMAND_BUFFER_BIT: Self = Self(2);
	pub const PROTECTED_BIT: Self = Self(4);
}

impl std::ops::BitOr for CommandPoolCreateFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for CommandPoolCreateFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for CommandPoolCreateFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for CommandPoolCreateFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CommandPoolResetFlagBits(pub u32);
impl CommandPoolResetFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const RELEASE_RESOURCES_BIT: Self = Self(1);
}

impl std::ops::BitOr for CommandPoolResetFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for CommandPoolResetFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for CommandPoolResetFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for CommandPoolResetFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CommandBufferResetFlagBits(pub u32);
impl CommandBufferResetFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const RELEASE_RESOURCES_BIT: Self = Self(1);
}

impl std::ops::BitOr for CommandBufferResetFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for CommandBufferResetFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for CommandBufferResetFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for CommandBufferResetFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SampleCountFlagBits(pub u32);
impl SampleCountFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const _1_BIT: Self = Self(1);
	pub const _2_BIT: Self = Self(2);
	pub const _4_BIT: Self = Self(4);
	pub const _8_BIT: Self = Self(8);
	pub const _16_BIT: Self = Self(16);
	pub const _32_BIT: Self = Self(32);
	pub const _64_BIT: Self = Self(64);
}

impl std::ops::BitOr for SampleCountFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for SampleCountFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for SampleCountFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for SampleCountFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AttachmentDescriptionFlagBits(pub u32);
impl AttachmentDescriptionFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const MAY_ALIAS_BIT: Self = Self(1);
	pub const RESOLVE_SKIP_TRANSFER_FUNCTION_BIT_KHR: Self = Self(2);
	pub const RESOLVE_ENABLE_TRANSFER_FUNCTION_BIT_KHR: Self = Self(4);
}

impl std::ops::BitOr for AttachmentDescriptionFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for AttachmentDescriptionFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for AttachmentDescriptionFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for AttachmentDescriptionFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StencilFaceFlagBits(pub u32);
impl StencilFaceFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const FRONT_BIT: Self = Self(1);
	pub const BACK_BIT: Self = Self(2);
	pub const FRONT_AND_BACK: Self = Self(3);
	pub const STENCIL_FRONT_AND_BACK: Self = Self(3);
}

impl std::ops::BitOr for StencilFaceFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for StencilFaceFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for StencilFaceFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for StencilFaceFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DescriptorPoolCreateFlagBits(pub u32);
impl DescriptorPoolCreateFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const FREE_DESCRIPTOR_SET_BIT: Self = Self(1);
	pub const UPDATE_AFTER_BIND_BIT: Self = Self(2);
	pub const UPDATE_AFTER_BIND_BIT_EXT: Self = Self(2);
	pub const HOST_ONLY_BIT_VALVE: Self = Self(4);
	pub const HOST_ONLY_BIT_EXT: Self = Self(4);
	pub const ALLOW_OVERALLOCATION_SETS_BIT_NV: Self = Self(8);
	pub const ALLOW_OVERALLOCATION_POOLS_BIT_NV: Self = Self(16);
}

impl std::ops::BitOr for DescriptorPoolCreateFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for DescriptorPoolCreateFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for DescriptorPoolCreateFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for DescriptorPoolCreateFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DependencyFlagBits(pub u32);
impl DependencyFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const BY_REGION_BIT: Self = Self(1);
	pub const DEVICE_GROUP_BIT: Self = Self(4);
	pub const VIEW_LOCAL_BIT: Self = Self(2);
	pub const VIEW_LOCAL_BIT_KHR: Self = Self(2);
	pub const DEVICE_GROUP_BIT_KHR: Self = Self(4);
	pub const FEEDBACK_LOOP_BIT_EXT: Self = Self(8);
	pub const QUEUE_FAMILY_OWNERSHIP_TRANSFER_USE_ALL_STAGES_BIT_KHR: Self = Self(32);
	pub const ASYMMETRIC_EVENT_BIT_KHR: Self = Self(64);
}

impl std::ops::BitOr for DependencyFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for DependencyFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for DependencyFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for DependencyFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SemaphoreType(pub i32);
impl SemaphoreType {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const BINARY: Self = Self(0);
	pub const TIMELINE: Self = Self(1);
	pub const BINARY_KHR: Self = Self(0);
	pub const TIMELINE_KHR: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SemaphoreWaitFlagBits(pub u32);
impl SemaphoreWaitFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ANY_BIT: Self = Self(1);
	pub const ANY_BIT_KHR: Self = Self(1);
}

impl std::ops::BitOr for SemaphoreWaitFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for SemaphoreWaitFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for SemaphoreWaitFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for SemaphoreWaitFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PresentModeKHR(pub i32);
impl PresentModeKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const IMMEDIATE_KHR: Self = Self(0);
	pub const MAILBOX_KHR: Self = Self(1);
	pub const FIFO_KHR: Self = Self(2);
	pub const FIFO_RELAXED_KHR: Self = Self(3);
	pub const SHARED_DEMAND_REFRESH_KHR: Self = Self(1000111000);
	pub const SHARED_CONTINUOUS_REFRESH_KHR: Self = Self(1000111001);
	pub const FIFO_LATEST_READY_EXT: Self = Self(1000361000);
	pub const FIFO_LATEST_READY_KHR: Self = Self(1000361000);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ColorSpaceKHR(pub i32);
impl ColorSpaceKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const SRGB_NONLINEAR_KHR: Self = Self(0);
	pub const COLORSPACE_SRGB_NONLINEAR_KHR: Self = Self(0);
	pub const DISPLAY_P3_NONLINEAR_EXT: Self = Self(1000104001);
	pub const EXTENDED_SRGB_LINEAR_EXT: Self = Self(1000104002);
	pub const DISPLAY_P3_LINEAR_EXT: Self = Self(1000104003);
	pub const DCI_P3_NONLINEAR_EXT: Self = Self(1000104004);
	pub const BT709_LINEAR_EXT: Self = Self(1000104005);
	pub const BT709_NONLINEAR_EXT: Self = Self(1000104006);
	pub const BT2020_LINEAR_EXT: Self = Self(1000104007);
	pub const HDR10_ST2084_EXT: Self = Self(1000104008);
	pub const DOLBYVISION_EXT: Self = Self(1000104009);
	pub const HDR10_HLG_EXT: Self = Self(1000104010);
	pub const ADOBERGB_LINEAR_EXT: Self = Self(1000104011);
	pub const ADOBERGB_NONLINEAR_EXT: Self = Self(1000104012);
	pub const PASS_THROUGH_EXT: Self = Self(1000104013);
	pub const EXTENDED_SRGB_NONLINEAR_EXT: Self = Self(1000104014);
	pub const DCI_P3_LINEAR_EXT: Self = Self(1000104003);
	pub const DISPLAY_NATIVE_AMD: Self = Self(1000213000);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DisplayPlaneAlphaFlagBitsKHR(pub u32);
impl DisplayPlaneAlphaFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const OPAQUE_BIT_KHR: Self = Self(1);
	pub const GLOBAL_BIT_KHR: Self = Self(2);
	pub const PER_PIXEL_BIT_KHR: Self = Self(4);
	pub const PER_PIXEL_PREMULTIPLIED_BIT_KHR: Self = Self(8);
}

impl std::ops::BitOr for DisplayPlaneAlphaFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for DisplayPlaneAlphaFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for DisplayPlaneAlphaFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for DisplayPlaneAlphaFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CompositeAlphaFlagBitsKHR(pub u32);
impl CompositeAlphaFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const OPAQUE_BIT_KHR: Self = Self(1);
	pub const PRE_MULTIPLIED_BIT_KHR: Self = Self(2);
	pub const POST_MULTIPLIED_BIT_KHR: Self = Self(4);
	pub const INHERIT_BIT_KHR: Self = Self(8);
}

impl std::ops::BitOr for CompositeAlphaFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for CompositeAlphaFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for CompositeAlphaFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for CompositeAlphaFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SurfaceTransformFlagBitsKHR(pub u32);
impl SurfaceTransformFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const IDENTITY_BIT_KHR: Self = Self(1);
	pub const ROTATE_90_BIT_KHR: Self = Self(2);
	pub const ROTATE_180_BIT_KHR: Self = Self(4);
	pub const ROTATE_270_BIT_KHR: Self = Self(8);
	pub const HORIZONTAL_MIRROR_BIT_KHR: Self = Self(16);
	pub const HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR: Self = Self(32);
	pub const HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR: Self = Self(64);
	pub const HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR: Self = Self(128);
	pub const INHERIT_BIT_KHR: Self = Self(256);
}

impl std::ops::BitOr for SurfaceTransformFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for SurfaceTransformFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for SurfaceTransformFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for SurfaceTransformFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DisplaySurfaceStereoTypeNV(pub i32);
impl DisplaySurfaceStereoTypeNV {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const NONE_NV: Self = Self(0);
	pub const ONBOARD_DIN_NV: Self = Self(1);
	pub const HDMI_3D_NV: Self = Self(2);
	pub const INBAND_DISPLAYPORT_NV: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TimeDomainKHR(pub i32);
impl TimeDomainKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEVICE_KHR: Self = Self(0);
	pub const CLOCK_MONOTONIC_KHR: Self = Self(1);
	pub const CLOCK_MONOTONIC_RAW_KHR: Self = Self(2);
	pub const QUERY_PERFORMANCE_COUNTER_KHR: Self = Self(3);
	pub const DEVICE_EXT: Self = Self(0);
	pub const CLOCK_MONOTONIC_EXT: Self = Self(1);
	pub const CLOCK_MONOTONIC_RAW_EXT: Self = Self(2);
	pub const QUERY_PERFORMANCE_COUNTER_EXT: Self = Self(3);
	pub const PRESENT_STAGE_LOCAL_EXT: Self = Self(1000208000);
	pub const SWAPCHAIN_LOCAL_EXT: Self = Self(1000208001);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DebugReportFlagBitsEXT(pub u32);
impl DebugReportFlagBitsEXT {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const INFORMATION_BIT_EXT: Self = Self(1);
	pub const WARNING_BIT_EXT: Self = Self(2);
	pub const PERFORMANCE_WARNING_BIT_EXT: Self = Self(4);
	pub const ERROR_BIT_EXT: Self = Self(8);
	pub const DEBUG_BIT_EXT: Self = Self(16);
}

impl std::ops::BitOr for DebugReportFlagBitsEXT {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for DebugReportFlagBitsEXT {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for DebugReportFlagBitsEXT {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for DebugReportFlagBitsEXT {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DebugReportObjectTypeEXT(pub i32);
impl DebugReportObjectTypeEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const UNKNOWN_EXT: Self = Self(0);
	pub const INSTANCE_EXT: Self = Self(1);
	pub const PHYSICAL_DEVICE_EXT: Self = Self(2);
	pub const DEVICE_EXT: Self = Self(3);
	pub const QUEUE_EXT: Self = Self(4);
	pub const SEMAPHORE_EXT: Self = Self(5);
	pub const COMMAND_BUFFER_EXT: Self = Self(6);
	pub const FENCE_EXT: Self = Self(7);
	pub const DEVICE_MEMORY_EXT: Self = Self(8);
	pub const BUFFER_EXT: Self = Self(9);
	pub const IMAGE_EXT: Self = Self(10);
	pub const EVENT_EXT: Self = Self(11);
	pub const QUERY_POOL_EXT: Self = Self(12);
	pub const BUFFER_VIEW_EXT: Self = Self(13);
	pub const IMAGE_VIEW_EXT: Self = Self(14);
	pub const SHADER_MODULE_EXT: Self = Self(15);
	pub const PIPELINE_CACHE_EXT: Self = Self(16);
	pub const PIPELINE_LAYOUT_EXT: Self = Self(17);
	pub const RENDER_PASS_EXT: Self = Self(18);
	pub const PIPELINE_EXT: Self = Self(19);
	pub const DESCRIPTOR_SET_LAYOUT_EXT: Self = Self(20);
	pub const SAMPLER_EXT: Self = Self(21);
	pub const DESCRIPTOR_POOL_EXT: Self = Self(22);
	pub const DESCRIPTOR_SET_EXT: Self = Self(23);
	pub const FRAMEBUFFER_EXT: Self = Self(24);
	pub const COMMAND_POOL_EXT: Self = Self(25);
	pub const SURFACE_KHR_EXT: Self = Self(26);
	pub const SWAPCHAIN_KHR_EXT: Self = Self(27);
	pub const DEBUG_REPORT_CALLBACK_EXT_EXT: Self = Self(28);
	pub const DEBUG_REPORT_EXT: Self = Self(28);
	pub const DISPLAY_KHR_EXT: Self = Self(29);
	pub const DISPLAY_MODE_KHR_EXT: Self = Self(30);
	pub const VALIDATION_CACHE_EXT_EXT: Self = Self(33);
	pub const VALIDATION_CACHE_EXT: Self = Self(33);
	pub const SAMPLER_YCBCR_CONVERSION_EXT: Self = Self(1000156000);
	pub const DESCRIPTOR_UPDATE_TEMPLATE_EXT: Self = Self(1000085000);
	pub const CU_MODULE_NVX_EXT: Self = Self(1000029000);
	pub const CU_FUNCTION_NVX_EXT: Self = Self(1000029001);
	pub const DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT: Self = Self(1000085000);
	pub const ACCELERATION_STRUCTURE_KHR_EXT: Self = Self(1000150000);
	pub const SAMPLER_YCBCR_CONVERSION_KHR_EXT: Self = Self(1000156000);
	pub const ACCELERATION_STRUCTURE_NV_EXT: Self = Self(1000165000);
	#[cfg(feature = "provisional")]
	pub const CUDA_MODULE_NV_EXT: Self = Self(1000307000);
	#[cfg(feature = "provisional")]
	pub const CUDA_FUNCTION_NV_EXT: Self = Self(1000307001);
	#[cfg(feature = "platform-fuchsia")]
	pub const BUFFER_COLLECTION_FUCHSIA_EXT: Self = Self(1000366000);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DeviceMemoryReportEventTypeEXT(pub i32);
impl DeviceMemoryReportEventTypeEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ALLOCATE_EXT: Self = Self(0);
	pub const FREE_EXT: Self = Self(1);
	pub const IMPORT_EXT: Self = Self(2);
	pub const UNIMPORT_EXT: Self = Self(3);
	pub const ALLOCATION_FAILED_EXT: Self = Self(4);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct RasterizationOrderAMD(pub i32);
impl RasterizationOrderAMD {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const STRICT_AMD: Self = Self(0);
	pub const RELAXED_AMD: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ExternalMemoryHandleTypeFlagBitsNV(pub u32);
impl ExternalMemoryHandleTypeFlagBitsNV {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const OPAQUE_WIN32_BIT_NV: Self = Self(1);
	pub const OPAQUE_WIN32_KMT_BIT_NV: Self = Self(2);
	pub const D3D11_IMAGE_BIT_NV: Self = Self(4);
	pub const D3D11_IMAGE_KMT_BIT_NV: Self = Self(8);
}

impl std::ops::BitOr for ExternalMemoryHandleTypeFlagBitsNV {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ExternalMemoryHandleTypeFlagBitsNV {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ExternalMemoryHandleTypeFlagBitsNV {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ExternalMemoryHandleTypeFlagBitsNV {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ExternalMemoryFeatureFlagBitsNV(pub u32);
impl ExternalMemoryFeatureFlagBitsNV {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEDICATED_ONLY_BIT_NV: Self = Self(1);
	pub const EXPORTABLE_BIT_NV: Self = Self(2);
	pub const IMPORTABLE_BIT_NV: Self = Self(4);
}

impl std::ops::BitOr for ExternalMemoryFeatureFlagBitsNV {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ExternalMemoryFeatureFlagBitsNV {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ExternalMemoryFeatureFlagBitsNV {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ExternalMemoryFeatureFlagBitsNV {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ClusterAccelerationStructureIndexFormatFlagBitsNV(pub u32);
impl ClusterAccelerationStructureIndexFormatFlagBitsNV {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const _8BIT_NV: Self = Self(1);
	pub const _16BIT_NV: Self = Self(2);
	pub const _32BIT_NV: Self = Self(4);
}

impl std::ops::BitOr for ClusterAccelerationStructureIndexFormatFlagBitsNV {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ClusterAccelerationStructureIndexFormatFlagBitsNV {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ClusterAccelerationStructureIndexFormatFlagBitsNV {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ClusterAccelerationStructureIndexFormatFlagBitsNV {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ClusterAccelerationStructureTypeNV(pub i32);
impl ClusterAccelerationStructureTypeNV {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const CLUSTERS_BOTTOM_LEVEL_NV: Self = Self(0);
	pub const TRIANGLE_CLUSTER_NV: Self = Self(1);
	pub const TRIANGLE_CLUSTER_TEMPLATE_NV: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ClusterAccelerationStructureOpTypeNV(pub i32);
impl ClusterAccelerationStructureOpTypeNV {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const MOVE_OBJECTS_NV: Self = Self(0);
	pub const BUILD_CLUSTERS_BOTTOM_LEVEL_NV: Self = Self(1);
	pub const BUILD_TRIANGLE_CLUSTER_NV: Self = Self(2);
	pub const BUILD_TRIANGLE_CLUSTER_TEMPLATE_NV: Self = Self(3);
	pub const INSTANTIATE_TRIANGLE_CLUSTER_NV: Self = Self(4);
	pub const GET_CLUSTER_TEMPLATE_INDICES_NV: Self = Self(5);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ClusterAccelerationStructureOpModeNV(pub i32);
impl ClusterAccelerationStructureOpModeNV {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const IMPLICIT_DESTINATIONS_NV: Self = Self(0);
	pub const EXPLICIT_DESTINATIONS_NV: Self = Self(1);
	pub const COMPUTE_SIZES_NV: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ClusterAccelerationStructureClusterFlagBitsNV(pub u32);
impl ClusterAccelerationStructureClusterFlagBitsNV {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ALLOW_DISABLE_OPACITY_MICROMAPS_NV: Self = Self(1);
}

impl std::ops::BitOr for ClusterAccelerationStructureClusterFlagBitsNV {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ClusterAccelerationStructureClusterFlagBitsNV {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ClusterAccelerationStructureClusterFlagBitsNV {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ClusterAccelerationStructureClusterFlagBitsNV {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ClusterAccelerationStructureGeometryFlagBitsNV(pub u32);
impl ClusterAccelerationStructureGeometryFlagBitsNV {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const CULL_DISABLE_BIT_NV: Self = Self(1);
	pub const NO_DUPLICATE_ANYHIT_INVOCATION_BIT_NV: Self = Self(2);
	pub const OPAQUE_BIT_NV: Self = Self(4);
}

impl std::ops::BitOr for ClusterAccelerationStructureGeometryFlagBitsNV {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ClusterAccelerationStructureGeometryFlagBitsNV {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ClusterAccelerationStructureGeometryFlagBitsNV {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ClusterAccelerationStructureGeometryFlagBitsNV {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ClusterAccelerationStructureAddressResolutionFlagBitsNV(pub u32);
impl ClusterAccelerationStructureAddressResolutionFlagBitsNV {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const NONE_NV: Self = Self(0);
	pub const INDIRECTED_DST_IMPLICIT_DATA_BIT_NV: Self = Self(1);
	pub const INDIRECTED_SCRATCH_DATA_BIT_NV: Self = Self(2);
	pub const INDIRECTED_DST_ADDRESS_ARRAY_BIT_NV: Self = Self(4);
	pub const INDIRECTED_DST_SIZES_ARRAY_BIT_NV: Self = Self(8);
	pub const INDIRECTED_SRC_INFOS_ARRAY_BIT_NV: Self = Self(16);
	pub const INDIRECTED_SRC_INFOS_COUNT_BIT_NV: Self = Self(32);
}

impl std::ops::BitOr for ClusterAccelerationStructureAddressResolutionFlagBitsNV {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ClusterAccelerationStructureAddressResolutionFlagBitsNV {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ClusterAccelerationStructureAddressResolutionFlagBitsNV {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ClusterAccelerationStructureAddressResolutionFlagBitsNV {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ValidationCheckEXT(pub i32);
impl ValidationCheckEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ALL_EXT: Self = Self(0);
	pub const SHADERS_EXT: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ValidationFeatureEnableEXT(pub i32);
impl ValidationFeatureEnableEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const GPU_ASSISTED_EXT: Self = Self(0);
	pub const GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT: Self = Self(1);
	pub const BEST_PRACTICES_EXT: Self = Self(2);
	pub const DEBUG_PRINTF_EXT: Self = Self(3);
	pub const SYNCHRONIZATION_VALIDATION_EXT: Self = Self(4);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ValidationFeatureDisableEXT(pub i32);
impl ValidationFeatureDisableEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ALL_EXT: Self = Self(0);
	pub const SHADERS_EXT: Self = Self(1);
	pub const THREAD_SAFETY_EXT: Self = Self(2);
	pub const API_PARAMETERS_EXT: Self = Self(3);
	pub const OBJECT_LIFETIMES_EXT: Self = Self(4);
	pub const CORE_CHECKS_EXT: Self = Self(5);
	pub const UNIQUE_HANDLES_EXT: Self = Self(6);
	pub const SHADER_VALIDATION_CACHE_EXT: Self = Self(7);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct LayerSettingTypeEXT(pub i32);
impl LayerSettingTypeEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const BOOL32_EXT: Self = Self(0);
	pub const INT32_EXT: Self = Self(1);
	pub const INT64_EXT: Self = Self(2);
	pub const UINT32_EXT: Self = Self(3);
	pub const UINT64_EXT: Self = Self(4);
	pub const FLOAT32_EXT: Self = Self(5);
	pub const FLOAT64_EXT: Self = Self(6);
	pub const STRING_EXT: Self = Self(7);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SubgroupFeatureFlagBits(pub u32);
impl SubgroupFeatureFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const BASIC_BIT: Self = Self(1);
	pub const VOTE_BIT: Self = Self(2);
	pub const ARITHMETIC_BIT: Self = Self(4);
	pub const BALLOT_BIT: Self = Self(8);
	pub const SHUFFLE_BIT: Self = Self(16);
	pub const SHUFFLE_RELATIVE_BIT: Self = Self(32);
	pub const CLUSTERED_BIT: Self = Self(64);
	pub const QUAD_BIT: Self = Self(128);
	pub const ROTATE_BIT: Self = Self(512);
	pub const ROTATE_CLUSTERED_BIT: Self = Self(1024);
	pub const PARTITIONED_BIT_NV: Self = Self(256);
	pub const ROTATE_BIT_KHR: Self = Self(512);
	pub const ROTATE_CLUSTERED_BIT_KHR: Self = Self(1024);
	pub const PARTITIONED_BIT_EXT: Self = Self(256);
}

impl std::ops::BitOr for SubgroupFeatureFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for SubgroupFeatureFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for SubgroupFeatureFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for SubgroupFeatureFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct IndirectCommandsLayoutUsageFlagBitsNV(pub u32);
impl IndirectCommandsLayoutUsageFlagBitsNV {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const EXPLICIT_PREPROCESS_BIT_NV: Self = Self(1);
	pub const INDEXED_SEQUENCES_BIT_NV: Self = Self(2);
	pub const UNORDERED_SEQUENCES_BIT_NV: Self = Self(4);
}

impl std::ops::BitOr for IndirectCommandsLayoutUsageFlagBitsNV {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for IndirectCommandsLayoutUsageFlagBitsNV {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for IndirectCommandsLayoutUsageFlagBitsNV {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for IndirectCommandsLayoutUsageFlagBitsNV {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct IndirectStateFlagBitsNV(pub u32);
impl IndirectStateFlagBitsNV {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const FLAG_FRONTFACE_BIT_NV: Self = Self(1);
}

impl std::ops::BitOr for IndirectStateFlagBitsNV {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for IndirectStateFlagBitsNV {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for IndirectStateFlagBitsNV {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for IndirectStateFlagBitsNV {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct IndirectCommandsTokenTypeNV(pub i32);
impl IndirectCommandsTokenTypeNV {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const SHADER_GROUP_NV: Self = Self(0);
	pub const STATE_FLAGS_NV: Self = Self(1);
	pub const INDEX_BUFFER_NV: Self = Self(2);
	pub const VERTEX_BUFFER_NV: Self = Self(3);
	pub const PUSH_CONSTANT_NV: Self = Self(4);
	pub const DRAW_INDEXED_NV: Self = Self(5);
	pub const DRAW_NV: Self = Self(6);
	pub const DRAW_TASKS_NV: Self = Self(7);
	pub const PUSH_DATA_NV: Self = Self(1000135000);
	pub const DRAW_MESH_TASKS_NV: Self = Self(1000328000);
	pub const PIPELINE_NV: Self = Self(1000428003);
	pub const DISPATCH_NV: Self = Self(1000428004);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PrivateDataSlotCreateFlagBits(pub u32);
impl PrivateDataSlotCreateFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const BASE_OBJECT_HANDLE_BIT_NV: Self = Self(1);
}

impl std::ops::BitOr for PrivateDataSlotCreateFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for PrivateDataSlotCreateFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for PrivateDataSlotCreateFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for PrivateDataSlotCreateFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DescriptorSetLayoutCreateFlagBits(pub u32);
impl DescriptorSetLayoutCreateFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const UPDATE_AFTER_BIND_POOL_BIT: Self = Self(2);
	pub const PUSH_DESCRIPTOR_BIT: Self = Self(1);
	pub const PUSH_DESCRIPTOR_BIT_KHR: Self = Self(1);
	pub const UPDATE_AFTER_BIND_POOL_BIT_EXT: Self = Self(2);
	pub const DESCRIPTOR_BUFFER_BIT_EXT: Self = Self(16);
	pub const EMBEDDED_IMMUTABLE_SAMPLERS_BIT_EXT: Self = Self(32);
	pub const HOST_ONLY_POOL_BIT_VALVE: Self = Self(4);
	pub const INDIRECT_BINDABLE_BIT_NV: Self = Self(128);
	pub const HOST_ONLY_POOL_BIT_EXT: Self = Self(4);
	pub const PER_STAGE_BIT_NV: Self = Self(64);
}

impl std::ops::BitOr for DescriptorSetLayoutCreateFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for DescriptorSetLayoutCreateFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for DescriptorSetLayoutCreateFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for DescriptorSetLayoutCreateFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ExternalMemoryHandleTypeFlagBits(pub u32);
impl ExternalMemoryHandleTypeFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const OPAQUE_FD_BIT: Self = Self(1);
	pub const OPAQUE_WIN32_BIT: Self = Self(2);
	pub const OPAQUE_WIN32_KMT_BIT: Self = Self(4);
	pub const D3D11_TEXTURE_BIT: Self = Self(8);
	pub const D3D11_TEXTURE_KMT_BIT: Self = Self(16);
	pub const D3D12_HEAP_BIT: Self = Self(32);
	pub const D3D12_RESOURCE_BIT: Self = Self(64);
	pub const OPAQUE_FD_BIT_KHR: Self = Self(1);
	pub const OPAQUE_WIN32_BIT_KHR: Self = Self(2);
	pub const OPAQUE_WIN32_KMT_BIT_KHR: Self = Self(4);
	pub const D3D11_TEXTURE_BIT_KHR: Self = Self(8);
	pub const D3D11_TEXTURE_KMT_BIT_KHR: Self = Self(16);
	pub const D3D12_HEAP_BIT_KHR: Self = Self(32);
	pub const D3D12_RESOURCE_BIT_KHR: Self = Self(64);
	pub const DMA_BUF_BIT_EXT: Self = Self(512);
	#[cfg(feature = "platform-android")]
	pub const ANDROID_HARDWARE_BUFFER_BIT_ANDROID: Self = Self(1024);
	pub const HOST_ALLOCATION_BIT_EXT: Self = Self(128);
	pub const HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT: Self = Self(256);
	#[cfg(feature = "platform-fuchsia")]
	pub const ZIRCON_VMO_BIT_FUCHSIA: Self = Self(2048);
	pub const RDMA_ADDRESS_BIT_NV: Self = Self(4096);
	#[cfg(feature = "platform-ohos")]
	pub const OH_NATIVE_BUFFER_BIT_OHOS: Self = Self(32768);
	#[cfg(feature = "platform-screen")]
	pub const SCREEN_BUFFER_BIT_QNX: Self = Self(16384);
	#[cfg(feature = "platform-metal")]
	pub const MTLBUFFER_BIT_EXT: Self = Self(65536);
	#[cfg(feature = "platform-metal")]
	pub const MTLTEXTURE_BIT_EXT: Self = Self(131072);
	#[cfg(feature = "platform-metal")]
	pub const MTLHEAP_BIT_EXT: Self = Self(262144);
}

impl std::ops::BitOr for ExternalMemoryHandleTypeFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ExternalMemoryHandleTypeFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ExternalMemoryHandleTypeFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ExternalMemoryHandleTypeFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ExternalMemoryFeatureFlagBits(pub u32);
impl ExternalMemoryFeatureFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEDICATED_ONLY_BIT: Self = Self(1);
	pub const EXPORTABLE_BIT: Self = Self(2);
	pub const IMPORTABLE_BIT: Self = Self(4);
	pub const DEDICATED_ONLY_BIT_KHR: Self = Self(1);
	pub const EXPORTABLE_BIT_KHR: Self = Self(2);
	pub const IMPORTABLE_BIT_KHR: Self = Self(4);
}

impl std::ops::BitOr for ExternalMemoryFeatureFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ExternalMemoryFeatureFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ExternalMemoryFeatureFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ExternalMemoryFeatureFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ExternalSemaphoreHandleTypeFlagBits(pub u32);
impl ExternalSemaphoreHandleTypeFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const OPAQUE_FD_BIT: Self = Self(1);
	pub const OPAQUE_WIN32_BIT: Self = Self(2);
	pub const OPAQUE_WIN32_KMT_BIT: Self = Self(4);
	pub const D3D12_FENCE_BIT: Self = Self(8);
	pub const D3D11_FENCE_BIT: Self = Self(8);
	pub const SYNC_FD_BIT: Self = Self(16);
	pub const OPAQUE_FD_BIT_KHR: Self = Self(1);
	pub const OPAQUE_WIN32_BIT_KHR: Self = Self(2);
	pub const OPAQUE_WIN32_KMT_BIT_KHR: Self = Self(4);
	pub const D3D12_FENCE_BIT_KHR: Self = Self(8);
	pub const SYNC_FD_BIT_KHR: Self = Self(16);
	#[cfg(feature = "platform-fuchsia")]
	pub const ZIRCON_EVENT_BIT_FUCHSIA: Self = Self(128);
}

impl std::ops::BitOr for ExternalSemaphoreHandleTypeFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ExternalSemaphoreHandleTypeFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ExternalSemaphoreHandleTypeFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ExternalSemaphoreHandleTypeFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ExternalSemaphoreFeatureFlagBits(pub u32);
impl ExternalSemaphoreFeatureFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const EXPORTABLE_BIT: Self = Self(1);
	pub const IMPORTABLE_BIT: Self = Self(2);
	pub const EXPORTABLE_BIT_KHR: Self = Self(1);
	pub const IMPORTABLE_BIT_KHR: Self = Self(2);
}

impl std::ops::BitOr for ExternalSemaphoreFeatureFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ExternalSemaphoreFeatureFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ExternalSemaphoreFeatureFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ExternalSemaphoreFeatureFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SemaphoreImportFlagBits(pub u32);
impl SemaphoreImportFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const TEMPORARY_BIT: Self = Self(1);
	pub const TEMPORARY_BIT_KHR: Self = Self(1);
}

impl std::ops::BitOr for SemaphoreImportFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for SemaphoreImportFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for SemaphoreImportFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for SemaphoreImportFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ExternalFenceHandleTypeFlagBits(pub u32);
impl ExternalFenceHandleTypeFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const OPAQUE_FD_BIT: Self = Self(1);
	pub const OPAQUE_WIN32_BIT: Self = Self(2);
	pub const OPAQUE_WIN32_KMT_BIT: Self = Self(4);
	pub const SYNC_FD_BIT: Self = Self(8);
	pub const OPAQUE_FD_BIT_KHR: Self = Self(1);
	pub const OPAQUE_WIN32_BIT_KHR: Self = Self(2);
	pub const OPAQUE_WIN32_KMT_BIT_KHR: Self = Self(4);
	pub const SYNC_FD_BIT_KHR: Self = Self(8);
}

impl std::ops::BitOr for ExternalFenceHandleTypeFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ExternalFenceHandleTypeFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ExternalFenceHandleTypeFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ExternalFenceHandleTypeFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ExternalFenceFeatureFlagBits(pub u32);
impl ExternalFenceFeatureFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const EXPORTABLE_BIT: Self = Self(1);
	pub const IMPORTABLE_BIT: Self = Self(2);
	pub const EXPORTABLE_BIT_KHR: Self = Self(1);
	pub const IMPORTABLE_BIT_KHR: Self = Self(2);
}

impl std::ops::BitOr for ExternalFenceFeatureFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ExternalFenceFeatureFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ExternalFenceFeatureFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ExternalFenceFeatureFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FenceImportFlagBits(pub u32);
impl FenceImportFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const TEMPORARY_BIT: Self = Self(1);
	pub const TEMPORARY_BIT_KHR: Self = Self(1);
}

impl std::ops::BitOr for FenceImportFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for FenceImportFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for FenceImportFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for FenceImportFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SurfaceCounterFlagBitsEXT(pub u32);
impl SurfaceCounterFlagBitsEXT {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const VBLANK_BIT_EXT: Self = Self(1);
	pub const VBLANK_EXT: Self = Self(1);
}

impl std::ops::BitOr for SurfaceCounterFlagBitsEXT {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for SurfaceCounterFlagBitsEXT {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for SurfaceCounterFlagBitsEXT {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for SurfaceCounterFlagBitsEXT {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DisplayPowerStateEXT(pub i32);
impl DisplayPowerStateEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const OFF_EXT: Self = Self(0);
	pub const SUSPEND_EXT: Self = Self(1);
	pub const ON_EXT: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DeviceEventTypeEXT(pub i32);
impl DeviceEventTypeEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DISPLAY_HOTPLUG_EXT: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DisplayEventTypeEXT(pub i32);
impl DisplayEventTypeEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const FIRST_PIXEL_OUT_EXT: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PeerMemoryFeatureFlagBits(pub u32);
impl PeerMemoryFeatureFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const COPY_SRC_BIT: Self = Self(1);
	pub const COPY_DST_BIT: Self = Self(2);
	pub const GENERIC_SRC_BIT: Self = Self(4);
	pub const GENERIC_DST_BIT: Self = Self(8);
	pub const COPY_SRC_BIT_KHR: Self = Self(1);
	pub const COPY_DST_BIT_KHR: Self = Self(2);
	pub const GENERIC_SRC_BIT_KHR: Self = Self(4);
	pub const GENERIC_DST_BIT_KHR: Self = Self(8);
}

impl std::ops::BitOr for PeerMemoryFeatureFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for PeerMemoryFeatureFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for PeerMemoryFeatureFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for PeerMemoryFeatureFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct MemoryAllocateFlagBits(pub u32);
impl MemoryAllocateFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEVICE_MASK_BIT: Self = Self(1);
	pub const DEVICE_ADDRESS_BIT: Self = Self(2);
	pub const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT: Self = Self(4);
	pub const DEVICE_MASK_BIT_KHR: Self = Self(1);
	pub const DEVICE_ADDRESS_BIT_KHR: Self = Self(2);
	pub const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR: Self = Self(4);
	pub const ZERO_INITIALIZE_BIT_EXT: Self = Self(8);
}

impl std::ops::BitOr for MemoryAllocateFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for MemoryAllocateFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for MemoryAllocateFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for MemoryAllocateFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DeviceGroupPresentModeFlagBitsKHR(pub u32);
impl DeviceGroupPresentModeFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const LOCAL_BIT_KHR: Self = Self(1);
	pub const REMOTE_BIT_KHR: Self = Self(2);
	pub const SUM_BIT_KHR: Self = Self(4);
	pub const LOCAL_MULTI_DEVICE_BIT_KHR: Self = Self(8);
}

impl std::ops::BitOr for DeviceGroupPresentModeFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for DeviceGroupPresentModeFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for DeviceGroupPresentModeFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for DeviceGroupPresentModeFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SwapchainCreateFlagBitsKHR(pub u32);
impl SwapchainCreateFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR: Self = Self(1);
	pub const PROTECTED_BIT_KHR: Self = Self(2);
	pub const MUTABLE_FORMAT_BIT_KHR: Self = Self(4);
	pub const PRESENT_TIMING_BIT_EXT: Self = Self(512);
	pub const DEFERRED_MEMORY_ALLOCATION_BIT_EXT: Self = Self(8);
	pub const PRESENT_ID_2_BIT_KHR: Self = Self(64);
	pub const PRESENT_WAIT_2_BIT_KHR: Self = Self(128);
	pub const DEFERRED_MEMORY_ALLOCATION_BIT_KHR: Self = Self(8);
	pub const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT: Self = Self(256);
}

impl std::ops::BitOr for SwapchainCreateFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for SwapchainCreateFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for SwapchainCreateFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for SwapchainCreateFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ViewportCoordinateSwizzleNV(pub i32);
impl ViewportCoordinateSwizzleNV {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const POSITIVE_X_NV: Self = Self(0);
	pub const NEGATIVE_X_NV: Self = Self(1);
	pub const POSITIVE_Y_NV: Self = Self(2);
	pub const NEGATIVE_Y_NV: Self = Self(3);
	pub const POSITIVE_Z_NV: Self = Self(4);
	pub const NEGATIVE_Z_NV: Self = Self(5);
	pub const POSITIVE_W_NV: Self = Self(6);
	pub const NEGATIVE_W_NV: Self = Self(7);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DiscardRectangleModeEXT(pub i32);
impl DiscardRectangleModeEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const INCLUSIVE_EXT: Self = Self(0);
	pub const EXCLUSIVE_EXT: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SubpassDescriptionFlagBits(pub u32);
impl SubpassDescriptionFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const PER_VIEW_ATTRIBUTES_BIT_NVX: Self = Self(1);
	pub const PER_VIEW_POSITION_X_ONLY_BIT_NVX: Self = Self(2);
	pub const FRAGMENT_REGION_BIT_QCOM: Self = Self(4);
	pub const SHADER_RESOLVE_BIT_QCOM: Self = Self(8);
	pub const TILE_SHADING_APRON_BIT_QCOM: Self = Self(256);
	pub const RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_ARM: Self = Self(16);
	pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM: Self = Self(32);
	pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM: Self = Self(64);
	pub const RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_EXT: Self = Self(16);
	pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT: Self = Self(32);
	pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT: Self = Self(64);
	pub const ENABLE_LEGACY_DITHERING_BIT_EXT: Self = Self(128);
	pub const FRAGMENT_REGION_BIT_EXT: Self = Self(4);
	pub const CUSTOM_RESOLVE_BIT_EXT: Self = Self(8);
}

impl std::ops::BitOr for SubpassDescriptionFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for SubpassDescriptionFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for SubpassDescriptionFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for SubpassDescriptionFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PointClippingBehavior(pub i32);
impl PointClippingBehavior {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ALL_CLIP_PLANES: Self = Self(0);
	pub const USER_CLIP_PLANES_ONLY: Self = Self(1);
	pub const ALL_CLIP_PLANES_KHR: Self = Self(0);
	pub const USER_CLIP_PLANES_ONLY_KHR: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SamplerReductionMode(pub i32);
impl SamplerReductionMode {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const WEIGHTED_AVERAGE: Self = Self(0);
	pub const MIN: Self = Self(1);
	pub const MAX: Self = Self(2);
	pub const WEIGHTED_AVERAGE_EXT: Self = Self(0);
	pub const MIN_EXT: Self = Self(1);
	pub const MAX_EXT: Self = Self(2);
	pub const WEIGHTED_AVERAGE_RANGECLAMP_QCOM: Self = Self(1000521000);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TessellationDomainOrigin(pub i32);
impl TessellationDomainOrigin {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const UPPER_LEFT: Self = Self(0);
	pub const LOWER_LEFT: Self = Self(1);
	pub const UPPER_LEFT_KHR: Self = Self(0);
	pub const LOWER_LEFT_KHR: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SamplerYcbcrModelConversion(pub i32);
impl SamplerYcbcrModelConversion {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const RGB_IDENTITY: Self = Self(0);
	pub const YCBCR_IDENTITY: Self = Self(1);
	pub const YCBCR_709: Self = Self(2);
	pub const YCBCR_601: Self = Self(3);
	pub const YCBCR_2020: Self = Self(4);
	pub const RGB_IDENTITY_KHR: Self = Self(0);
	pub const YCBCR_IDENTITY_KHR: Self = Self(1);
	pub const YCBCR_709_KHR: Self = Self(2);
	pub const YCBCR_601_KHR: Self = Self(3);
	pub const YCBCR_2020_KHR: Self = Self(4);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SamplerYcbcrRange(pub i32);
impl SamplerYcbcrRange {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ITU_FULL: Self = Self(0);
	pub const ITU_NARROW: Self = Self(1);
	pub const ITU_FULL_KHR: Self = Self(0);
	pub const ITU_NARROW_KHR: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ChromaLocation(pub i32);
impl ChromaLocation {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const COSITED_EVEN: Self = Self(0);
	pub const MIDPOINT: Self = Self(1);
	pub const COSITED_EVEN_KHR: Self = Self(0);
	pub const MIDPOINT_KHR: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BlendOverlapEXT(pub i32);
impl BlendOverlapEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const UNCORRELATED_EXT: Self = Self(0);
	pub const DISJOINT_EXT: Self = Self(1);
	pub const CONJOINT_EXT: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CoverageModulationModeNV(pub i32);
impl CoverageModulationModeNV {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const NONE_NV: Self = Self(0);
	pub const RGB_NV: Self = Self(1);
	pub const ALPHA_NV: Self = Self(2);
	pub const RGBA_NV: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CoverageReductionModeNV(pub i32);
impl CoverageReductionModeNV {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const MERGE_NV: Self = Self(0);
	pub const TRUNCATE_NV: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ValidationCacheHeaderVersionEXT(pub i32);
impl ValidationCacheHeaderVersionEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ONE_EXT: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ShaderInfoTypeAMD(pub i32);
impl ShaderInfoTypeAMD {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const STATISTICS_AMD: Self = Self(0);
	pub const BINARY_AMD: Self = Self(1);
	pub const DISASSEMBLY_AMD: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct QueueGlobalPriority(pub i32);
impl QueueGlobalPriority {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const LOW: Self = Self(128);
	pub const MEDIUM: Self = Self(256);
	pub const HIGH: Self = Self(512);
	pub const REALTIME: Self = Self(1024);
	pub const LOW_EXT: Self = Self(128);
	pub const MEDIUM_EXT: Self = Self(256);
	pub const HIGH_EXT: Self = Self(512);
	pub const REALTIME_EXT: Self = Self(1024);
	pub const LOW_KHR: Self = Self(128);
	pub const MEDIUM_KHR: Self = Self(256);
	pub const HIGH_KHR: Self = Self(512);
	pub const REALTIME_KHR: Self = Self(1024);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DebugUtilsMessageSeverityFlagBitsEXT(pub u32);
impl DebugUtilsMessageSeverityFlagBitsEXT {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const VERBOSE_BIT_EXT: Self = Self(1);
	pub const INFO_BIT_EXT: Self = Self(16);
	pub const WARNING_BIT_EXT: Self = Self(256);
	pub const ERROR_BIT_EXT: Self = Self(4096);
}

impl std::ops::BitOr for DebugUtilsMessageSeverityFlagBitsEXT {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for DebugUtilsMessageSeverityFlagBitsEXT {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for DebugUtilsMessageSeverityFlagBitsEXT {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for DebugUtilsMessageSeverityFlagBitsEXT {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DebugUtilsMessageTypeFlagBitsEXT(pub u32);
impl DebugUtilsMessageTypeFlagBitsEXT {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const GENERAL_BIT_EXT: Self = Self(1);
	pub const VALIDATION_BIT_EXT: Self = Self(2);
	pub const PERFORMANCE_BIT_EXT: Self = Self(4);
	pub const DEVICE_ADDRESS_BINDING_BIT_EXT: Self = Self(8);
}

impl std::ops::BitOr for DebugUtilsMessageTypeFlagBitsEXT {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for DebugUtilsMessageTypeFlagBitsEXT {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for DebugUtilsMessageTypeFlagBitsEXT {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for DebugUtilsMessageTypeFlagBitsEXT {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ConservativeRasterizationModeEXT(pub i32);
impl ConservativeRasterizationModeEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DISABLED_EXT: Self = Self(0);
	pub const OVERESTIMATE_EXT: Self = Self(1);
	pub const UNDERESTIMATE_EXT: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DescriptorBindingFlagBits(pub u32);
impl DescriptorBindingFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const UPDATE_AFTER_BIND_BIT: Self = Self(1);
	pub const UPDATE_UNUSED_WHILE_PENDING_BIT: Self = Self(2);
	pub const PARTIALLY_BOUND_BIT: Self = Self(4);
	pub const VARIABLE_DESCRIPTOR_COUNT_BIT: Self = Self(8);
	pub const UPDATE_AFTER_BIND_BIT_EXT: Self = Self(1);
	pub const UPDATE_UNUSED_WHILE_PENDING_BIT_EXT: Self = Self(2);
	pub const PARTIALLY_BOUND_BIT_EXT: Self = Self(4);
	pub const VARIABLE_DESCRIPTOR_COUNT_BIT_EXT: Self = Self(8);
}

impl std::ops::BitOr for DescriptorBindingFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for DescriptorBindingFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for DescriptorBindingFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for DescriptorBindingFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VendorId(pub i32);
impl VendorId {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const KHRONOS: Self = Self(65536);
	pub const VIV: Self = Self(65537);
	pub const VSI: Self = Self(65538);
	pub const KAZAN: Self = Self(65539);
	pub const CODEPLAY: Self = Self(65540);
	pub const MESA: Self = Self(65541);
	pub const POCL: Self = Self(65542);
	pub const MOBILEYE: Self = Self(65543);
	pub const APE: Self = Self(65544);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DriverId(pub i32);
impl DriverId {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const AMD_PROPRIETARY: Self = Self(1);
	pub const AMD_OPEN_SOURCE: Self = Self(2);
	pub const MESA_RADV: Self = Self(3);
	pub const NVIDIA_PROPRIETARY: Self = Self(4);
	pub const INTEL_PROPRIETARY_WINDOWS: Self = Self(5);
	pub const INTEL_OPEN_SOURCE_MESA: Self = Self(6);
	pub const IMAGINATION_PROPRIETARY: Self = Self(7);
	pub const QUALCOMM_PROPRIETARY: Self = Self(8);
	pub const ARM_PROPRIETARY: Self = Self(9);
	pub const GOOGLE_SWIFTSHADER: Self = Self(10);
	pub const GGP_PROPRIETARY: Self = Self(11);
	pub const BROADCOM_PROPRIETARY: Self = Self(12);
	pub const MESA_LLVMPIPE: Self = Self(13);
	pub const MOLTENVK: Self = Self(14);
	pub const COREAVI_PROPRIETARY: Self = Self(15);
	pub const JUICE_PROPRIETARY: Self = Self(16);
	pub const VERISILICON_PROPRIETARY: Self = Self(17);
	pub const MESA_TURNIP: Self = Self(18);
	pub const MESA_V3DV: Self = Self(19);
	pub const MESA_PANVK: Self = Self(20);
	pub const SAMSUNG_PROPRIETARY: Self = Self(21);
	pub const MESA_VENUS: Self = Self(22);
	pub const MESA_DOZEN: Self = Self(23);
	pub const MESA_NVK: Self = Self(24);
	pub const IMAGINATION_OPEN_SOURCE_MESA: Self = Self(25);
	pub const MESA_HONEYKRISP: Self = Self(26);
	pub const VULKAN_SC_EMULATION_ON_VULKAN: Self = Self(27);
	pub const MESA_KOSMICKRISP: Self = Self(28);
	pub const MESA_GFXSTREAM: Self = Self(29);
	pub const APE_SOFT: Self = Self(30);
	pub const RESERVED_31: Self = Self(31);
	pub const AMD_PROPRIETARY_KHR: Self = Self(1);
	pub const AMD_OPEN_SOURCE_KHR: Self = Self(2);
	pub const MESA_RADV_KHR: Self = Self(3);
	pub const NVIDIA_PROPRIETARY_KHR: Self = Self(4);
	pub const INTEL_PROPRIETARY_WINDOWS_KHR: Self = Self(5);
	pub const INTEL_OPEN_SOURCE_MESA_KHR: Self = Self(6);
	pub const IMAGINATION_PROPRIETARY_KHR: Self = Self(7);
	pub const QUALCOMM_PROPRIETARY_KHR: Self = Self(8);
	pub const ARM_PROPRIETARY_KHR: Self = Self(9);
	pub const GOOGLE_SWIFTSHADER_KHR: Self = Self(10);
	pub const GGP_PROPRIETARY_KHR: Self = Self(11);
	pub const BROADCOM_PROPRIETARY_KHR: Self = Self(12);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ConditionalRenderingFlagBitsEXT(pub u32);
impl ConditionalRenderingFlagBitsEXT {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const INVERTED_BIT_EXT: Self = Self(1);
}

impl std::ops::BitOr for ConditionalRenderingFlagBitsEXT {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ConditionalRenderingFlagBitsEXT {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ConditionalRenderingFlagBitsEXT {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ConditionalRenderingFlagBitsEXT {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ResolveModeFlagBits(pub u32);
impl ResolveModeFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const NONE: Self = Self(0);
	pub const SAMPLE_ZERO_BIT: Self = Self(1);
	pub const AVERAGE_BIT: Self = Self(2);
	pub const MIN_BIT: Self = Self(4);
	pub const MAX_BIT: Self = Self(8);
	pub const NONE_KHR: Self = Self(0);
	pub const SAMPLE_ZERO_BIT_KHR: Self = Self(1);
	pub const AVERAGE_BIT_KHR: Self = Self(2);
	pub const MIN_BIT_KHR: Self = Self(4);
	pub const MAX_BIT_KHR: Self = Self(8);
	#[cfg(feature = "platform-android")]
	pub const EXTERNAL_FORMAT_DOWNSAMPLE_BIT_ANDROID: Self = Self(16);
	#[cfg(feature = "platform-android")]
	pub const EXTERNAL_FORMAT_DOWNSAMPLE_ANDROID: Self = Self(16);
	pub const CUSTOM_BIT_EXT: Self = Self(32);
}

impl std::ops::BitOr for ResolveModeFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ResolveModeFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ResolveModeFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ResolveModeFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ShadingRatePaletteEntryNV(pub i32);
impl ShadingRatePaletteEntryNV {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const NO_INVOCATIONS_NV: Self = Self(0);
	pub const _16_INVOCATIONS_PER_PIXEL_NV: Self = Self(1);
	pub const _8_INVOCATIONS_PER_PIXEL_NV: Self = Self(2);
	pub const _4_INVOCATIONS_PER_PIXEL_NV: Self = Self(3);
	pub const _2_INVOCATIONS_PER_PIXEL_NV: Self = Self(4);
	pub const _1_INVOCATION_PER_PIXEL_NV: Self = Self(5);
	pub const _1_INVOCATION_PER_2X1_PIXELS_NV: Self = Self(6);
	pub const _1_INVOCATION_PER_1X2_PIXELS_NV: Self = Self(7);
	pub const _1_INVOCATION_PER_2X2_PIXELS_NV: Self = Self(8);
	pub const _1_INVOCATION_PER_4X2_PIXELS_NV: Self = Self(9);
	pub const _1_INVOCATION_PER_2X4_PIXELS_NV: Self = Self(10);
	pub const _1_INVOCATION_PER_4X4_PIXELS_NV: Self = Self(11);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CoarseSampleOrderTypeNV(pub i32);
impl CoarseSampleOrderTypeNV {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEFAULT_NV: Self = Self(0);
	pub const CUSTOM_NV: Self = Self(1);
	pub const PIXEL_MAJOR_NV: Self = Self(2);
	pub const SAMPLE_MAJOR_NV: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct GeometryInstanceFlagBitsKHR(pub u32);
impl GeometryInstanceFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const TRIANGLE_FACING_CULL_DISABLE_BIT_KHR: Self = Self(1);
	pub const TRIANGLE_FLIP_FACING_BIT_KHR: Self = Self(2);
	pub const FORCE_OPAQUE_BIT_KHR: Self = Self(4);
	pub const FORCE_NO_OPAQUE_BIT_KHR: Self = Self(8);
	pub const TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR: Self = Self(2);
	pub const TRIANGLE_CULL_DISABLE_BIT_NV: Self = Self(1);
	pub const TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_NV: Self = Self(2);
	pub const FORCE_OPAQUE_BIT_NV: Self = Self(4);
	pub const FORCE_NO_OPAQUE_BIT_NV: Self = Self(8);
	pub const FORCE_OPACITY_MICROMAP_2_STATE_BIT_EXT: Self = Self(16);
	pub const FORCE_OPACITY_MICROMAP_2_STATE_EXT: Self = Self(16);
	pub const DISABLE_OPACITY_MICROMAPS_BIT_EXT: Self = Self(32);
	pub const DISABLE_OPACITY_MICROMAPS_EXT: Self = Self(32);
	pub const FORCE_OPACITY_MICROMAP_2_STATE_BIT_KHR: Self = Self(16);
	pub const DISABLE_OPACITY_MICROMAPS_BIT_KHR: Self = Self(32);
}

impl std::ops::BitOr for GeometryInstanceFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for GeometryInstanceFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for GeometryInstanceFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for GeometryInstanceFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct GeometryFlagBitsKHR(pub u32);
impl GeometryFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const OPAQUE_BIT_KHR: Self = Self(1);
	pub const NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR: Self = Self(2);
	pub const OPAQUE_BIT_NV: Self = Self(1);
	pub const NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_NV: Self = Self(2);
}

impl std::ops::BitOr for GeometryFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for GeometryFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for GeometryFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for GeometryFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BuildAccelerationStructureFlagBitsKHR(pub u32);
impl BuildAccelerationStructureFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ALLOW_UPDATE_BIT_KHR: Self = Self(1);
	pub const ALLOW_COMPACTION_BIT_KHR: Self = Self(2);
	pub const PREFER_FAST_TRACE_BIT_KHR: Self = Self(4);
	pub const PREFER_FAST_BUILD_BIT_KHR: Self = Self(8);
	pub const LOW_MEMORY_BIT_KHR: Self = Self(16);
	pub const ALLOW_UPDATE_BIT_NV: Self = Self(1);
	pub const ALLOW_COMPACTION_BIT_NV: Self = Self(2);
	pub const PREFER_FAST_TRACE_BIT_NV: Self = Self(4);
	pub const PREFER_FAST_BUILD_BIT_NV: Self = Self(8);
	pub const LOW_MEMORY_BIT_NV: Self = Self(16);
	pub const MOTION_BIT_NV: Self = Self(32);
	pub const ALLOW_OPACITY_MICROMAP_UPDATE_BIT_EXT: Self = Self(64);
	pub const ALLOW_OPACITY_MICROMAP_UPDATE_EXT: Self = Self(64);
	pub const ALLOW_DISABLE_OPACITY_MICROMAPS_BIT_EXT: Self = Self(128);
	pub const ALLOW_DISABLE_OPACITY_MICROMAPS_EXT: Self = Self(128);
	pub const ALLOW_OPACITY_MICROMAP_DATA_UPDATE_BIT_EXT: Self = Self(256);
	pub const ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT: Self = Self(256);
	#[cfg(feature = "provisional")]
	pub const ALLOW_DISPLACEMENT_MICROMAP_UPDATE_BIT_NV: Self = Self(512);
	#[cfg(feature = "provisional")]
	pub const ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV: Self = Self(512);
	pub const ALLOW_DATA_ACCESS_BIT_KHR: Self = Self(2048);
	pub const ALLOW_DATA_ACCESS_KHR: Self = Self(2048);
	pub const ALLOW_CLUSTER_OPACITY_MICROMAPS_BIT_NV: Self = Self(4096);
	pub const ALLOW_OPACITY_MICROMAP_UPDATE_BIT_KHR: Self = Self(64);
	pub const ALLOW_DISABLE_OPACITY_MICROMAPS_BIT_KHR: Self = Self(128);
	pub const MICROMAP_LOSSY_BIT_KHR: Self = Self(1024);
}

impl std::ops::BitOr for BuildAccelerationStructureFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for BuildAccelerationStructureFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for BuildAccelerationStructureFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for BuildAccelerationStructureFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AccelerationStructureCreateFlagBitsKHR(pub u32);
impl AccelerationStructureCreateFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR: Self = Self(1);
	pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT: Self = Self(8);
	pub const MOTION_BIT_NV: Self = Self(4);
}

impl std::ops::BitOr for AccelerationStructureCreateFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for AccelerationStructureCreateFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for AccelerationStructureCreateFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for AccelerationStructureCreateFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CopyAccelerationStructureModeKHR(pub i32);
impl CopyAccelerationStructureModeKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const CLONE_KHR: Self = Self(0);
	pub const COMPACT_KHR: Self = Self(1);
	pub const SERIALIZE_KHR: Self = Self(2);
	pub const DESERIALIZE_KHR: Self = Self(3);
	pub const CLONE_NV: Self = Self(0);
	pub const COMPACT_NV: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BuildAccelerationStructureModeKHR(pub i32);
impl BuildAccelerationStructureModeKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const BUILD_KHR: Self = Self(0);
	pub const UPDATE_KHR: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AccelerationStructureTypeKHR(pub i32);
impl AccelerationStructureTypeKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const TOP_LEVEL_KHR: Self = Self(0);
	pub const BOTTOM_LEVEL_KHR: Self = Self(1);
	pub const GENERIC_KHR: Self = Self(2);
	pub const TOP_LEVEL_NV: Self = Self(0);
	pub const BOTTOM_LEVEL_NV: Self = Self(1);
	pub const OPACITY_MICROMAP_KHR: Self = Self(1000623000);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct GeometryTypeKHR(pub i32);
impl GeometryTypeKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const TRIANGLES_KHR: Self = Self(0);
	pub const AABBS_KHR: Self = Self(1);
	pub const INSTANCES_KHR: Self = Self(2);
	pub const TRIANGLES_NV: Self = Self(0);
	pub const AABBS_NV: Self = Self(1);
	pub const SPHERES_NV: Self = Self(1000429004);
	pub const LINEAR_SWEPT_SPHERES_NV: Self = Self(1000429005);
	#[cfg(feature = "provisional")]
	pub const DENSE_GEOMETRY_FORMAT_TRIANGLES_AMDX: Self = Self(1000478000);
	pub const MICROMAP_KHR: Self = Self(1000623000);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AccelerationStructureMemoryRequirementsTypeNV(pub i32);
impl AccelerationStructureMemoryRequirementsTypeNV {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const OBJECT_NV: Self = Self(0);
	pub const BUILD_SCRATCH_NV: Self = Self(1);
	pub const UPDATE_SCRATCH_NV: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AccelerationStructureBuildTypeKHR(pub i32);
impl AccelerationStructureBuildTypeKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const HOST_KHR: Self = Self(0);
	pub const DEVICE_KHR: Self = Self(1);
	pub const HOST_OR_DEVICE_KHR: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct RayTracingShaderGroupTypeKHR(pub i32);
impl RayTracingShaderGroupTypeKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const GENERAL_KHR: Self = Self(0);
	pub const TRIANGLES_HIT_GROUP_KHR: Self = Self(1);
	pub const PROCEDURAL_HIT_GROUP_KHR: Self = Self(2);
	pub const GENERAL_NV: Self = Self(0);
	pub const TRIANGLES_HIT_GROUP_NV: Self = Self(1);
	pub const PROCEDURAL_HIT_GROUP_NV: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AccelerationStructureCompatibilityKHR(pub i32);
impl AccelerationStructureCompatibilityKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const COMPATIBLE_KHR: Self = Self(0);
	pub const INCOMPATIBLE_KHR: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ShaderGroupShaderKHR(pub i32);
impl ShaderGroupShaderKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const GENERAL_KHR: Self = Self(0);
	pub const CLOSEST_HIT_KHR: Self = Self(1);
	pub const ANY_HIT_KHR: Self = Self(2);
	pub const INTERSECTION_KHR: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct MemoryOverallocationBehaviorAMD(pub i32);
impl MemoryOverallocationBehaviorAMD {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEFAULT_AMD: Self = Self(0);
	pub const ALLOWED_AMD: Self = Self(1);
	pub const DISALLOWED_AMD: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FramebufferCreateFlagBits(pub u32);
impl FramebufferCreateFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const IMAGELESS_BIT: Self = Self(1);
	pub const IMAGELESS_BIT_KHR: Self = Self(1);
}

impl std::ops::BitOr for FramebufferCreateFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for FramebufferCreateFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for FramebufferCreateFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for FramebufferCreateFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct QueryPoolCreateFlagBits(pub u32);
impl QueryPoolCreateFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const RESET_BIT_KHR: Self = Self(1);
}

impl std::ops::BitOr for QueryPoolCreateFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for QueryPoolCreateFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for QueryPoolCreateFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for QueryPoolCreateFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DeviceDiagnosticsConfigFlagBitsNV(pub u32);
impl DeviceDiagnosticsConfigFlagBitsNV {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ENABLE_SHADER_DEBUG_INFO_BIT_NV: Self = Self(1);
	pub const ENABLE_RESOURCE_TRACKING_BIT_NV: Self = Self(2);
	pub const ENABLE_AUTOMATIC_CHECKPOINTS_BIT_NV: Self = Self(4);
	pub const ENABLE_SHADER_ERROR_REPORTING_BIT_NV: Self = Self(8);
}

impl std::ops::BitOr for DeviceDiagnosticsConfigFlagBitsNV {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for DeviceDiagnosticsConfigFlagBitsNV {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for DeviceDiagnosticsConfigFlagBitsNV {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for DeviceDiagnosticsConfigFlagBitsNV {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PipelineCreationFeedbackFlagBits(pub u32);
impl PipelineCreationFeedbackFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const VALID_BIT: Self = Self(1);
	pub const APPLICATION_PIPELINE_CACHE_HIT_BIT: Self = Self(2);
	pub const BASE_PIPELINE_ACCELERATION_BIT: Self = Self(4);
	pub const VALID_BIT_EXT: Self = Self(1);
	pub const APPLICATION_PIPELINE_CACHE_HIT_BIT_EXT: Self = Self(2);
	pub const BASE_PIPELINE_ACCELERATION_BIT_EXT: Self = Self(4);
}

impl std::ops::BitOr for PipelineCreationFeedbackFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for PipelineCreationFeedbackFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for PipelineCreationFeedbackFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for PipelineCreationFeedbackFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[cfg(feature = "platform-win32")]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FullScreenExclusiveEXT(pub i32);
#[cfg(feature = "platform-win32")]
impl FullScreenExclusiveEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEFAULT_EXT: Self = Self(0);
	pub const ALLOWED_EXT: Self = Self(1);
	pub const DISALLOWED_EXT: Self = Self(2);
	pub const APPLICATION_CONTROLLED_EXT: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PerformanceCounterScopeKHR(pub i32);
impl PerformanceCounterScopeKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const COMMAND_BUFFER_KHR: Self = Self(0);
	pub const RENDER_PASS_KHR: Self = Self(1);
	pub const COMMAND_KHR: Self = Self(2);
	pub const QUERY_SCOPE_COMMAND_BUFFER_KHR: Self = Self(0);
	pub const QUERY_SCOPE_RENDER_PASS_KHR: Self = Self(1);
	pub const QUERY_SCOPE_COMMAND_KHR: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct MemoryDecompressionMethodFlagBitsEXT(pub u64);
impl MemoryDecompressionMethodFlagBitsEXT {
	pub const fn from_raw(value: u64) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u64 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const GDEFLATE_1_0_BIT_EXT: Self = Self(1);
	pub const GDEFLATE_1_0_BIT_NV: Self = Self(1);
}

impl std::ops::BitOr for MemoryDecompressionMethodFlagBitsEXT {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for MemoryDecompressionMethodFlagBitsEXT {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for MemoryDecompressionMethodFlagBitsEXT {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for MemoryDecompressionMethodFlagBitsEXT {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PerformanceCounterUnitKHR(pub i32);
impl PerformanceCounterUnitKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const GENERIC_KHR: Self = Self(0);
	pub const PERCENTAGE_KHR: Self = Self(1);
	pub const NANOSECONDS_KHR: Self = Self(2);
	pub const BYTES_KHR: Self = Self(3);
	pub const BYTES_PER_SECOND_KHR: Self = Self(4);
	pub const KELVIN_KHR: Self = Self(5);
	pub const WATTS_KHR: Self = Self(6);
	pub const VOLTS_KHR: Self = Self(7);
	pub const AMPS_KHR: Self = Self(8);
	pub const HERTZ_KHR: Self = Self(9);
	pub const CYCLES_KHR: Self = Self(10);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PerformanceCounterStorageKHR(pub i32);
impl PerformanceCounterStorageKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const INT32_KHR: Self = Self(0);
	pub const INT64_KHR: Self = Self(1);
	pub const UINT32_KHR: Self = Self(2);
	pub const UINT64_KHR: Self = Self(3);
	pub const FLOAT32_KHR: Self = Self(4);
	pub const FLOAT64_KHR: Self = Self(5);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PerformanceCounterDescriptionFlagBitsKHR(pub u32);
impl PerformanceCounterDescriptionFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const PERFORMANCE_IMPACTING_BIT_KHR: Self = Self(1);
	pub const PERFORMANCE_IMPACTING_KHR: Self = Self(1);
	pub const CONCURRENTLY_IMPACTED_BIT_KHR: Self = Self(2);
	pub const CONCURRENTLY_IMPACTED_KHR: Self = Self(2);
}

impl std::ops::BitOr for PerformanceCounterDescriptionFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for PerformanceCounterDescriptionFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for PerformanceCounterDescriptionFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for PerformanceCounterDescriptionFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AcquireProfilingLockFlagBitsKHR(pub u32);
impl AcquireProfilingLockFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}
}

impl std::ops::BitOr for AcquireProfilingLockFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for AcquireProfilingLockFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for AcquireProfilingLockFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for AcquireProfilingLockFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ShaderCorePropertiesFlagBitsAMD(pub u32);
impl ShaderCorePropertiesFlagBitsAMD {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}
}

impl std::ops::BitOr for ShaderCorePropertiesFlagBitsAMD {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ShaderCorePropertiesFlagBitsAMD {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ShaderCorePropertiesFlagBitsAMD {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ShaderCorePropertiesFlagBitsAMD {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PerformanceConfigurationTypeINTEL(pub i32);
impl PerformanceConfigurationTypeINTEL {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct QueryPoolSamplingModeINTEL(pub i32);
impl QueryPoolSamplingModeINTEL {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const MANUAL_INTEL: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PerformanceOverrideTypeINTEL(pub i32);
impl PerformanceOverrideTypeINTEL {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const NULL_HARDWARE_INTEL: Self = Self(0);
	pub const FLUSH_GPU_CACHES_INTEL: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PerformanceParameterTypeINTEL(pub i32);
impl PerformanceParameterTypeINTEL {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const HW_COUNTERS_SUPPORTED_INTEL: Self = Self(0);
	pub const STREAM_MARKER_VALID_BITS_INTEL: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PerformanceValueTypeINTEL(pub i32);
impl PerformanceValueTypeINTEL {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const UINT32_INTEL: Self = Self(0);
	pub const UINT64_INTEL: Self = Self(1);
	pub const FLOAT_INTEL: Self = Self(2);
	pub const BOOL_INTEL: Self = Self(3);
	pub const STRING_INTEL: Self = Self(4);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ShaderFloatControlsIndependence(pub i32);
impl ShaderFloatControlsIndependence {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const _32_BIT_ONLY: Self = Self(0);
	pub const ALL: Self = Self(1);
	pub const NONE: Self = Self(2);
	pub const _32_BIT_ONLY_KHR: Self = Self(0);
	pub const ALL_KHR: Self = Self(1);
	pub const NONE_KHR: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PipelineExecutableStatisticFormatKHR(pub i32);
impl PipelineExecutableStatisticFormatKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const BOOL32_KHR: Self = Self(0);
	pub const INT64_KHR: Self = Self(1);
	pub const UINT64_KHR: Self = Self(2);
	pub const FLOAT64_KHR: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct LineRasterizationMode(pub i32);
impl LineRasterizationMode {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEFAULT: Self = Self(0);
	pub const RECTANGULAR: Self = Self(1);
	pub const BRESENHAM: Self = Self(2);
	pub const RECTANGULAR_SMOOTH: Self = Self(3);
	pub const DEFAULT_EXT: Self = Self(0);
	pub const RECTANGULAR_EXT: Self = Self(1);
	pub const BRESENHAM_EXT: Self = Self(2);
	pub const RECTANGULAR_SMOOTH_EXT: Self = Self(3);
	pub const DEFAULT_KHR: Self = Self(0);
	pub const RECTANGULAR_KHR: Self = Self(1);
	pub const BRESENHAM_KHR: Self = Self(2);
	pub const RECTANGULAR_SMOOTH_KHR: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PipelineCompilerControlFlagBitsAMD(pub u32);
impl PipelineCompilerControlFlagBitsAMD {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}
}

impl std::ops::BitOr for PipelineCompilerControlFlagBitsAMD {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for PipelineCompilerControlFlagBitsAMD {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for PipelineCompilerControlFlagBitsAMD {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for PipelineCompilerControlFlagBitsAMD {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ToolPurposeFlagBits(pub u32);
impl ToolPurposeFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const VALIDATION_BIT: Self = Self(1);
	pub const PROFILING_BIT: Self = Self(2);
	pub const TRACING_BIT: Self = Self(4);
	pub const ADDITIONAL_FEATURES_BIT: Self = Self(8);
	pub const MODIFYING_FEATURES_BIT: Self = Self(16);
	pub const VALIDATION_BIT_EXT: Self = Self(1);
	pub const PROFILING_BIT_EXT: Self = Self(2);
	pub const TRACING_BIT_EXT: Self = Self(4);
	pub const ADDITIONAL_FEATURES_BIT_EXT: Self = Self(8);
	pub const MODIFYING_FEATURES_BIT_EXT: Self = Self(16);
	pub const DEBUG_REPORTING_BIT_EXT: Self = Self(32);
	pub const DEBUG_MARKERS_BIT_EXT: Self = Self(64);
}

impl std::ops::BitOr for ToolPurposeFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ToolPurposeFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ToolPurposeFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ToolPurposeFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FragmentShadingRateCombinerOpKHR(pub i32);
impl FragmentShadingRateCombinerOpKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const KEEP_KHR: Self = Self(0);
	pub const REPLACE_KHR: Self = Self(1);
	pub const MIN_KHR: Self = Self(2);
	pub const MAX_KHR: Self = Self(3);
	pub const MUL_KHR: Self = Self(4);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FragmentShadingRateNV(pub i32);
impl FragmentShadingRateNV {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const _1_INVOCATION_PER_PIXEL_NV: Self = Self(0);
	pub const _1_INVOCATION_PER_1X2_PIXELS_NV: Self = Self(1);
	pub const _1_INVOCATION_PER_2X1_PIXELS_NV: Self = Self(4);
	pub const _1_INVOCATION_PER_2X2_PIXELS_NV: Self = Self(5);
	pub const _1_INVOCATION_PER_2X4_PIXELS_NV: Self = Self(6);
	pub const _1_INVOCATION_PER_4X2_PIXELS_NV: Self = Self(9);
	pub const _1_INVOCATION_PER_4X4_PIXELS_NV: Self = Self(10);
	pub const _2_INVOCATIONS_PER_PIXEL_NV: Self = Self(11);
	pub const _4_INVOCATIONS_PER_PIXEL_NV: Self = Self(12);
	pub const _8_INVOCATIONS_PER_PIXEL_NV: Self = Self(13);
	pub const _16_INVOCATIONS_PER_PIXEL_NV: Self = Self(14);
	pub const NO_INVOCATIONS_NV: Self = Self(15);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FragmentShadingRateTypeNV(pub i32);
impl FragmentShadingRateTypeNV {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const FRAGMENT_SIZE_NV: Self = Self(0);
	pub const ENUMS_NV: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SubpassMergeStatusEXT(pub i32);
impl SubpassMergeStatusEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const MERGED_EXT: Self = Self(0);
	pub const DISALLOWED_EXT: Self = Self(1);
	pub const NOT_MERGED_SIDE_EFFECTS_EXT: Self = Self(2);
	pub const NOT_MERGED_SAMPLES_MISMATCH_EXT: Self = Self(3);
	pub const NOT_MERGED_VIEWS_MISMATCH_EXT: Self = Self(4);
	pub const NOT_MERGED_ALIASING_EXT: Self = Self(5);
	pub const NOT_MERGED_DEPENDENCIES_EXT: Self = Self(6);
	pub const NOT_MERGED_INCOMPATIBLE_INPUT_ATTACHMENT_EXT: Self = Self(7);
	pub const NOT_MERGED_TOO_MANY_ATTACHMENTS_EXT: Self = Self(8);
	pub const NOT_MERGED_INSUFFICIENT_STORAGE_EXT: Self = Self(9);
	pub const NOT_MERGED_DEPTH_STENCIL_COUNT_EXT: Self = Self(10);
	pub const NOT_MERGED_RESOLVE_ATTACHMENT_REUSE_EXT: Self = Self(11);
	pub const NOT_MERGED_SINGLE_SUBPASS_EXT: Self = Self(12);
	pub const NOT_MERGED_UNSPECIFIED_EXT: Self = Self(13);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AccessFlagBits2(pub u64);
impl AccessFlagBits2 {
	pub const fn from_raw(value: u64) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u64 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const NONE: Self = Self(0);
	pub const INDIRECT_COMMAND_READ_BIT: Self = Self(1);
	pub const INDEX_READ_BIT: Self = Self(2);
	pub const VERTEX_ATTRIBUTE_READ_BIT: Self = Self(4);
	pub const UNIFORM_READ_BIT: Self = Self(8);
	pub const INPUT_ATTACHMENT_READ_BIT: Self = Self(16);
	pub const SHADER_READ_BIT: Self = Self(32);
	pub const SHADER_WRITE_BIT: Self = Self(64);
	pub const COLOR_ATTACHMENT_READ_BIT: Self = Self(128);
	pub const COLOR_ATTACHMENT_WRITE_BIT: Self = Self(256);
	pub const DEPTH_STENCIL_ATTACHMENT_READ_BIT: Self = Self(512);
	pub const DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: Self = Self(1024);
	pub const TRANSFER_READ_BIT: Self = Self(2048);
	pub const TRANSFER_WRITE_BIT: Self = Self(4096);
	pub const HOST_READ_BIT: Self = Self(8192);
	pub const HOST_WRITE_BIT: Self = Self(16384);
	pub const MEMORY_READ_BIT: Self = Self(32768);
	pub const MEMORY_WRITE_BIT: Self = Self(65536);
	pub const SHADER_SAMPLED_READ_BIT: Self = Self(4294967296);
	pub const SHADER_STORAGE_READ_BIT: Self = Self(8589934592);
	pub const SHADER_STORAGE_WRITE_BIT: Self = Self(17179869184);
	pub const VIDEO_DECODE_READ_BIT_KHR: Self = Self(34359738368);
	pub const VIDEO_DECODE_WRITE_BIT_KHR: Self = Self(68719476736);
	pub const SAMPLER_HEAP_READ_BIT_EXT: Self = Self(144115188075855872);
	pub const RESOURCE_HEAP_READ_BIT_EXT: Self = Self(288230376151711744);
	pub const VIDEO_ENCODE_READ_BIT_KHR: Self = Self(137438953472);
	pub const VIDEO_ENCODE_WRITE_BIT_KHR: Self = Self(274877906944);
	pub const SHADER_TILE_ATTACHMENT_READ_BIT_QCOM: Self = Self(2251799813685248);
	pub const SHADER_TILE_ATTACHMENT_WRITE_BIT_QCOM: Self = Self(4503599627370496);
	pub const NONE_KHR: Self = Self(0);
	pub const INDIRECT_COMMAND_READ_BIT_KHR: Self = Self(1);
	pub const INDEX_READ_BIT_KHR: Self = Self(2);
	pub const VERTEX_ATTRIBUTE_READ_BIT_KHR: Self = Self(4);
	pub const UNIFORM_READ_BIT_KHR: Self = Self(8);
	pub const INPUT_ATTACHMENT_READ_BIT_KHR: Self = Self(16);
	pub const SHADER_READ_BIT_KHR: Self = Self(32);
	pub const SHADER_WRITE_BIT_KHR: Self = Self(64);
	pub const COLOR_ATTACHMENT_READ_BIT_KHR: Self = Self(128);
	pub const COLOR_ATTACHMENT_WRITE_BIT_KHR: Self = Self(256);
	pub const DEPTH_STENCIL_ATTACHMENT_READ_BIT_KHR: Self = Self(512);
	pub const DEPTH_STENCIL_ATTACHMENT_WRITE_BIT_KHR: Self = Self(1024);
	pub const TRANSFER_READ_BIT_KHR: Self = Self(2048);
	pub const TRANSFER_WRITE_BIT_KHR: Self = Self(4096);
	pub const HOST_READ_BIT_KHR: Self = Self(8192);
	pub const HOST_WRITE_BIT_KHR: Self = Self(16384);
	pub const MEMORY_READ_BIT_KHR: Self = Self(32768);
	pub const MEMORY_WRITE_BIT_KHR: Self = Self(65536);
	pub const SHADER_SAMPLED_READ_BIT_KHR: Self = Self(4294967296);
	pub const SHADER_STORAGE_READ_BIT_KHR: Self = Self(8589934592);
	pub const SHADER_STORAGE_WRITE_BIT_KHR: Self = Self(17179869184);
	pub const TRANSFORM_FEEDBACK_WRITE_BIT_EXT: Self = Self(33554432);
	pub const TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT: Self = Self(67108864);
	pub const TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT: Self = Self(134217728);
	pub const CONDITIONAL_RENDERING_READ_BIT_EXT: Self = Self(1048576);
	pub const COMMAND_PREPROCESS_READ_BIT_NV: Self = Self(131072);
	pub const COMMAND_PREPROCESS_WRITE_BIT_NV: Self = Self(262144);
	pub const COMMAND_PREPROCESS_READ_BIT_EXT: Self = Self(131072);
	pub const COMMAND_PREPROCESS_WRITE_BIT_EXT: Self = Self(262144);
	pub const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR: Self = Self(8388608);
	pub const SHADING_RATE_IMAGE_READ_BIT_NV: Self = Self(8388608);
	pub const ACCELERATION_STRUCTURE_READ_BIT_KHR: Self = Self(2097152);
	pub const ACCELERATION_STRUCTURE_WRITE_BIT_KHR: Self = Self(4194304);
	pub const ACCELERATION_STRUCTURE_READ_BIT_NV: Self = Self(2097152);
	pub const ACCELERATION_STRUCTURE_WRITE_BIT_NV: Self = Self(4194304);
	pub const FRAGMENT_DENSITY_MAP_READ_BIT_EXT: Self = Self(16777216);
	pub const COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT: Self = Self(524288);
	pub const DESCRIPTOR_BUFFER_READ_BIT_EXT: Self = Self(2199023255552);
	pub const INVOCATION_MASK_READ_BIT_HUAWEI: Self = Self(549755813888);
	pub const SHADER_BINDING_TABLE_READ_BIT_KHR: Self = Self(1099511627776);
	pub const MICROMAP_READ_BIT_EXT: Self = Self(17592186044416);
	pub const MICROMAP_WRITE_BIT_EXT: Self = Self(35184372088832);
	pub const OPTICAL_FLOW_READ_BIT_NV: Self = Self(4398046511104);
	pub const OPTICAL_FLOW_WRITE_BIT_NV: Self = Self(8796093022208);
	pub const DATA_GRAPH_READ_BIT_ARM: Self = Self(140737488355328);
	pub const DATA_GRAPH_WRITE_BIT_ARM: Self = Self(281474976710656);
	pub const MEMORY_DECOMPRESSION_READ_BIT_EXT: Self = Self(36028797018963968);
	pub const MEMORY_DECOMPRESSION_WRITE_BIT_EXT: Self = Self(72057594037927936);
}

impl std::ops::BitOr for AccessFlagBits2 {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for AccessFlagBits2 {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for AccessFlagBits2 {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for AccessFlagBits2 {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PipelineStageFlagBits2(pub u64);
impl PipelineStageFlagBits2 {
	pub const fn from_raw(value: u64) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u64 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const NONE: Self = Self(0);
	pub const TOP_OF_PIPE_BIT: Self = Self(1);
	pub const DRAW_INDIRECT_BIT: Self = Self(2);
	pub const VERTEX_INPUT_BIT: Self = Self(4);
	pub const VERTEX_SHADER_BIT: Self = Self(8);
	pub const TESSELLATION_CONTROL_SHADER_BIT: Self = Self(16);
	pub const TESSELLATION_EVALUATION_SHADER_BIT: Self = Self(32);
	pub const GEOMETRY_SHADER_BIT: Self = Self(64);
	pub const FRAGMENT_SHADER_BIT: Self = Self(128);
	pub const EARLY_FRAGMENT_TESTS_BIT: Self = Self(256);
	pub const LATE_FRAGMENT_TESTS_BIT: Self = Self(512);
	pub const COLOR_ATTACHMENT_OUTPUT_BIT: Self = Self(1024);
	pub const COMPUTE_SHADER_BIT: Self = Self(2048);
	pub const ALL_TRANSFER_BIT: Self = Self(4096);
	pub const TRANSFER_BIT: Self = Self(4096);
	pub const BOTTOM_OF_PIPE_BIT: Self = Self(8192);
	pub const HOST_BIT: Self = Self(16384);
	pub const ALL_GRAPHICS_BIT: Self = Self(32768);
	pub const ALL_COMMANDS_BIT: Self = Self(65536);
	pub const COPY_BIT: Self = Self(4294967296);
	pub const RESOLVE_BIT: Self = Self(8589934592);
	pub const BLIT_BIT: Self = Self(17179869184);
	pub const CLEAR_BIT: Self = Self(34359738368);
	pub const INDEX_INPUT_BIT: Self = Self(68719476736);
	pub const VERTEX_ATTRIBUTE_INPUT_BIT: Self = Self(137438953472);
	pub const PRE_RASTERIZATION_SHADERS_BIT: Self = Self(274877906944);
	pub const VIDEO_DECODE_BIT_KHR: Self = Self(67108864);
	pub const VIDEO_ENCODE_BIT_KHR: Self = Self(134217728);
	pub const NONE_KHR: Self = Self(0);
	pub const TOP_OF_PIPE_BIT_KHR: Self = Self(1);
	pub const DRAW_INDIRECT_BIT_KHR: Self = Self(2);
	pub const VERTEX_INPUT_BIT_KHR: Self = Self(4);
	pub const VERTEX_SHADER_BIT_KHR: Self = Self(8);
	pub const TESSELLATION_CONTROL_SHADER_BIT_KHR: Self = Self(16);
	pub const TESSELLATION_EVALUATION_SHADER_BIT_KHR: Self = Self(32);
	pub const GEOMETRY_SHADER_BIT_KHR: Self = Self(64);
	pub const FRAGMENT_SHADER_BIT_KHR: Self = Self(128);
	pub const EARLY_FRAGMENT_TESTS_BIT_KHR: Self = Self(256);
	pub const LATE_FRAGMENT_TESTS_BIT_KHR: Self = Self(512);
	pub const COLOR_ATTACHMENT_OUTPUT_BIT_KHR: Self = Self(1024);
	pub const COMPUTE_SHADER_BIT_KHR: Self = Self(2048);
	pub const ALL_TRANSFER_BIT_KHR: Self = Self(4096);
	pub const TRANSFER_BIT_KHR: Self = Self(4096);
	pub const BOTTOM_OF_PIPE_BIT_KHR: Self = Self(8192);
	pub const HOST_BIT_KHR: Self = Self(16384);
	pub const ALL_GRAPHICS_BIT_KHR: Self = Self(32768);
	pub const ALL_COMMANDS_BIT_KHR: Self = Self(65536);
	pub const COPY_BIT_KHR: Self = Self(4294967296);
	pub const RESOLVE_BIT_KHR: Self = Self(8589934592);
	pub const BLIT_BIT_KHR: Self = Self(17179869184);
	pub const CLEAR_BIT_KHR: Self = Self(34359738368);
	pub const INDEX_INPUT_BIT_KHR: Self = Self(68719476736);
	pub const VERTEX_ATTRIBUTE_INPUT_BIT_KHR: Self = Self(137438953472);
	pub const PRE_RASTERIZATION_SHADERS_BIT_KHR: Self = Self(274877906944);
	pub const TRANSFORM_FEEDBACK_BIT_EXT: Self = Self(16777216);
	pub const CONDITIONAL_RENDERING_BIT_EXT: Self = Self(262144);
	pub const COMMAND_PREPROCESS_BIT_NV: Self = Self(131072);
	pub const COMMAND_PREPROCESS_BIT_EXT: Self = Self(131072);
	pub const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: Self = Self(4194304);
	pub const SHADING_RATE_IMAGE_BIT_NV: Self = Self(4194304);
	pub const ACCELERATION_STRUCTURE_BUILD_BIT_KHR: Self = Self(33554432);
	pub const RAY_TRACING_SHADER_BIT_KHR: Self = Self(2097152);
	pub const RAY_TRACING_SHADER_BIT_NV: Self = Self(2097152);
	pub const ACCELERATION_STRUCTURE_BUILD_BIT_NV: Self = Self(33554432);
	pub const FRAGMENT_DENSITY_PROCESS_BIT_EXT: Self = Self(8388608);
	pub const TASK_SHADER_BIT_NV: Self = Self(524288);
	pub const MESH_SHADER_BIT_NV: Self = Self(1048576);
	pub const TASK_SHADER_BIT_EXT: Self = Self(524288);
	pub const MESH_SHADER_BIT_EXT: Self = Self(1048576);
	pub const SUBPASS_SHADER_BIT_HUAWEI: Self = Self(549755813888);
	pub const SUBPASS_SHADING_BIT_HUAWEI: Self = Self(549755813888);
	pub const INVOCATION_MASK_BIT_HUAWEI: Self = Self(1099511627776);
	pub const ACCELERATION_STRUCTURE_COPY_BIT_KHR: Self = Self(268435456);
	pub const MICROMAP_BUILD_BIT_EXT: Self = Self(1073741824);
	pub const CLUSTER_CULLING_SHADER_BIT_HUAWEI: Self = Self(2199023255552);
	pub const OPTICAL_FLOW_BIT_NV: Self = Self(536870912);
	pub const CONVERT_COOPERATIVE_VECTOR_MATRIX_BIT_NV: Self = Self(17592186044416);
	pub const DATA_GRAPH_BIT_ARM: Self = Self(4398046511104);
	pub const COPY_INDIRECT_BIT_KHR: Self = Self(70368744177664);
	pub const MEMORY_DECOMPRESSION_BIT_EXT: Self = Self(35184372088832);
}

impl std::ops::BitOr for PipelineStageFlagBits2 {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for PipelineStageFlagBits2 {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for PipelineStageFlagBits2 {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for PipelineStageFlagBits2 {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SubmitFlagBits(pub u32);
impl SubmitFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const PROTECTED_BIT: Self = Self(1);
	pub const PROTECTED_BIT_KHR: Self = Self(1);
}

impl std::ops::BitOr for SubmitFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for SubmitFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for SubmitFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for SubmitFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct EventCreateFlagBits(pub u32);
impl EventCreateFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEVICE_ONLY_BIT: Self = Self(1);
	pub const DEVICE_ONLY_BIT_KHR: Self = Self(1);
}

impl std::ops::BitOr for EventCreateFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for EventCreateFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for EventCreateFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for EventCreateFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PipelineLayoutCreateFlagBits(pub u32);
impl PipelineLayoutCreateFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const INDEPENDENT_SETS_BIT_EXT: Self = Self(2);
	pub const NO_TASK_SHADER_BIT_KHR: Self = Self(4);
}

impl std::ops::BitOr for PipelineLayoutCreateFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for PipelineLayoutCreateFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for PipelineLayoutCreateFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for PipelineLayoutCreateFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProvokingVertexModeEXT(pub i32);
impl ProvokingVertexModeEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const FIRST_VERTEX_EXT: Self = Self(0);
	pub const LAST_VERTEX_EXT: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AccelerationStructureMotionInstanceTypeNV(pub i32);
impl AccelerationStructureMotionInstanceTypeNV {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const STATIC_NV: Self = Self(0);
	pub const MATRIX_MOTION_NV: Self = Self(1);
	pub const SRT_MOTION_NV: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PipelineColorBlendStateCreateFlagBits(pub u32);
impl PipelineColorBlendStateCreateFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_ARM: Self = Self(1);
	pub const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_EXT: Self = Self(1);
}

impl std::ops::BitOr for PipelineColorBlendStateCreateFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for PipelineColorBlendStateCreateFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for PipelineColorBlendStateCreateFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for PipelineColorBlendStateCreateFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PipelineDepthStencilStateCreateFlagBits(pub u32);
impl PipelineDepthStencilStateCreateFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM: Self = Self(1);
	pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM: Self = Self(2);
	pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT: Self = Self(1);
	pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT: Self = Self(2);
}

impl std::ops::BitOr for PipelineDepthStencilStateCreateFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for PipelineDepthStencilStateCreateFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for PipelineDepthStencilStateCreateFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for PipelineDepthStencilStateCreateFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct GraphicsPipelineLibraryFlagBitsEXT(pub u32);
impl GraphicsPipelineLibraryFlagBitsEXT {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const VERTEX_INPUT_INTERFACE_BIT_EXT: Self = Self(1);
	pub const PRE_RASTERIZATION_SHADERS_BIT_EXT: Self = Self(2);
	pub const FRAGMENT_SHADER_BIT_EXT: Self = Self(4);
	pub const FRAGMENT_OUTPUT_INTERFACE_BIT_EXT: Self = Self(8);
}

impl std::ops::BitOr for GraphicsPipelineLibraryFlagBitsEXT {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for GraphicsPipelineLibraryFlagBitsEXT {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for GraphicsPipelineLibraryFlagBitsEXT {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for GraphicsPipelineLibraryFlagBitsEXT {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct RenderingAttachmentFlagBitsKHR(pub u32);
impl RenderingAttachmentFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const INPUT_ATTACHMENT_FEEDBACK_BIT_KHR: Self = Self(1);
	pub const RESOLVE_SKIP_TRANSFER_FUNCTION_BIT_KHR: Self = Self(2);
	pub const RESOLVE_ENABLE_TRANSFER_FUNCTION_BIT_KHR: Self = Self(4);
}

impl std::ops::BitOr for RenderingAttachmentFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for RenderingAttachmentFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for RenderingAttachmentFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for RenderingAttachmentFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ResolveImageFlagBitsKHR(pub u32);
impl ResolveImageFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const SKIP_TRANSFER_FUNCTION_BIT_KHR: Self = Self(1);
	pub const ENABLE_TRANSFER_FUNCTION_BIT_KHR: Self = Self(2);
}

impl std::ops::BitOr for ResolveImageFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ResolveImageFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ResolveImageFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ResolveImageFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DeviceAddressBindingFlagBitsEXT(pub u32);
impl DeviceAddressBindingFlagBitsEXT {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const INTERNAL_OBJECT_BIT_EXT: Self = Self(1);
}

impl std::ops::BitOr for DeviceAddressBindingFlagBitsEXT {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for DeviceAddressBindingFlagBitsEXT {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for DeviceAddressBindingFlagBitsEXT {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for DeviceAddressBindingFlagBitsEXT {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DeviceAddressBindingTypeEXT(pub i32);
impl DeviceAddressBindingTypeEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const BIND_EXT: Self = Self(0);
	pub const UNBIND_EXT: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FrameBoundaryFlagBitsEXT(pub u32);
impl FrameBoundaryFlagBitsEXT {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const FRAME_END_BIT_EXT: Self = Self(1);
}

impl std::ops::BitOr for FrameBoundaryFlagBitsEXT {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for FrameBoundaryFlagBitsEXT {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for FrameBoundaryFlagBitsEXT {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for FrameBoundaryFlagBitsEXT {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PresentScalingFlagBitsKHR(pub u32);
impl PresentScalingFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ONE_TO_ONE_BIT_KHR: Self = Self(1);
	pub const ONE_TO_ONE_BIT_EXT: Self = Self(1);
	pub const ASPECT_RATIO_STRETCH_BIT_KHR: Self = Self(2);
	pub const ASPECT_RATIO_STRETCH_BIT_EXT: Self = Self(2);
	pub const STRETCH_BIT_KHR: Self = Self(4);
	pub const STRETCH_BIT_EXT: Self = Self(4);
}

impl std::ops::BitOr for PresentScalingFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for PresentScalingFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for PresentScalingFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for PresentScalingFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PresentGravityFlagBitsKHR(pub u32);
impl PresentGravityFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const MIN_BIT_KHR: Self = Self(1);
	pub const MIN_BIT_EXT: Self = Self(1);
	pub const MAX_BIT_KHR: Self = Self(2);
	pub const MAX_BIT_EXT: Self = Self(2);
	pub const CENTERED_BIT_KHR: Self = Self(4);
	pub const CENTERED_BIT_EXT: Self = Self(4);
}

impl std::ops::BitOr for PresentGravityFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for PresentGravityFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for PresentGravityFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for PresentGravityFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PhysicalDeviceSchedulingControlsFlagBitsARM(pub u64);
impl PhysicalDeviceSchedulingControlsFlagBitsARM {
	pub const fn from_raw(value: u64) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u64 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const SHADER_CORE_COUNT_ARM: Self = Self(1);
	pub const DISPATCH_PARAMETERS_ARM: Self = Self(2);
}

impl std::ops::BitOr for PhysicalDeviceSchedulingControlsFlagBitsARM {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for PhysicalDeviceSchedulingControlsFlagBitsARM {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for PhysicalDeviceSchedulingControlsFlagBitsARM {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for PhysicalDeviceSchedulingControlsFlagBitsARM {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PresentStageFlagBitsEXT(pub u32);
impl PresentStageFlagBitsEXT {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const QUEUE_OPERATIONS_END_BIT_EXT: Self = Self(1);
	pub const REQUEST_DEQUEUED_BIT_EXT: Self = Self(2);
	pub const IMAGE_FIRST_PIXEL_OUT_BIT_EXT: Self = Self(4);
	pub const IMAGE_FIRST_PIXEL_VISIBLE_BIT_EXT: Self = Self(8);
}

impl std::ops::BitOr for PresentStageFlagBitsEXT {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for PresentStageFlagBitsEXT {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for PresentStageFlagBitsEXT {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for PresentStageFlagBitsEXT {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PastPresentationTimingFlagBitsEXT(pub u32);
impl PastPresentationTimingFlagBitsEXT {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ALLOW_PARTIAL_RESULTS_BIT_EXT: Self = Self(1);
	pub const ALLOW_OUT_OF_ORDER_RESULTS_BIT_EXT: Self = Self(2);
}

impl std::ops::BitOr for PastPresentationTimingFlagBitsEXT {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for PastPresentationTimingFlagBitsEXT {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for PastPresentationTimingFlagBitsEXT {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for PastPresentationTimingFlagBitsEXT {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PresentTimingInfoFlagBitsEXT(pub u32);
impl PresentTimingInfoFlagBitsEXT {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const PRESENT_AT_RELATIVE_TIME_BIT_EXT: Self = Self(1);
	pub const PRESENT_AT_NEAREST_REFRESH_CYCLE_BIT_EXT: Self = Self(2);
}

impl std::ops::BitOr for PresentTimingInfoFlagBitsEXT {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for PresentTimingInfoFlagBitsEXT {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for PresentTimingInfoFlagBitsEXT {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for PresentTimingInfoFlagBitsEXT {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoCodecOperationFlagBitsKHR(pub u32);
impl VideoCodecOperationFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const NONE_KHR: Self = Self(0);
	pub const ENCODE_H264_BIT_KHR: Self = Self(65536);
	pub const ENCODE_H265_BIT_KHR: Self = Self(131072);
	pub const DECODE_H264_BIT_KHR: Self = Self(1);
	pub const DECODE_H265_BIT_KHR: Self = Self(2);
	pub const DECODE_AV1_BIT_KHR: Self = Self(4);
	pub const ENCODE_AV1_BIT_KHR: Self = Self(262144);
	pub const DECODE_VP9_BIT_KHR: Self = Self(8);
}

impl std::ops::BitOr for VideoCodecOperationFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoCodecOperationFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoCodecOperationFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoCodecOperationFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoChromaSubsamplingFlagBitsKHR(pub u32);
impl VideoChromaSubsamplingFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const INVALID_KHR: Self = Self(0);
	pub const MONOCHROME_BIT_KHR: Self = Self(1);
	pub const _420_BIT_KHR: Self = Self(2);
	pub const _422_BIT_KHR: Self = Self(4);
	pub const _444_BIT_KHR: Self = Self(8);
}

impl std::ops::BitOr for VideoChromaSubsamplingFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoChromaSubsamplingFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoChromaSubsamplingFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoChromaSubsamplingFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoComponentBitDepthFlagBitsKHR(pub u32);
impl VideoComponentBitDepthFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const INVALID_KHR: Self = Self(0);
	pub const _8_BIT_KHR: Self = Self(1);
	pub const _10_BIT_KHR: Self = Self(4);
	pub const _12_BIT_KHR: Self = Self(16);
}

impl std::ops::BitOr for VideoComponentBitDepthFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoComponentBitDepthFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoComponentBitDepthFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoComponentBitDepthFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoCapabilityFlagBitsKHR(pub u32);
impl VideoCapabilityFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const PROTECTED_CONTENT_BIT_KHR: Self = Self(1);
	pub const SEPARATE_REFERENCE_IMAGES_BIT_KHR: Self = Self(2);
}

impl std::ops::BitOr for VideoCapabilityFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoCapabilityFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoCapabilityFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoCapabilityFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoSessionCreateFlagBitsKHR(pub u32);
impl VideoSessionCreateFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const PROTECTED_CONTENT_BIT_KHR: Self = Self(1);
	pub const ALLOW_ENCODE_PARAMETER_OPTIMIZATIONS_BIT_KHR: Self = Self(2);
	pub const INLINE_QUERIES_BIT_KHR: Self = Self(4);
	pub const ALLOW_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR: Self = Self(8);
	pub const ALLOW_ENCODE_EMPHASIS_MAP_BIT_KHR: Self = Self(16);
	pub const INLINE_SESSION_PARAMETERS_BIT_KHR: Self = Self(32);
}

impl std::ops::BitOr for VideoSessionCreateFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoSessionCreateFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoSessionCreateFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoSessionCreateFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoSessionParametersCreateFlagBitsKHR(pub u32);
impl VideoSessionParametersCreateFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const QUANTIZATION_MAP_COMPATIBLE_BIT_KHR: Self = Self(1);
}

impl std::ops::BitOr for VideoSessionParametersCreateFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoSessionParametersCreateFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoSessionParametersCreateFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoSessionParametersCreateFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoDecodeH264PictureLayoutFlagBitsKHR(pub u32);
impl VideoDecodeH264PictureLayoutFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const PROGRESSIVE_KHR: Self = Self(0);
	pub const INTERLACED_INTERLEAVED_LINES_BIT_KHR: Self = Self(1);
	pub const INTERLACED_SEPARATE_PLANES_BIT_KHR: Self = Self(2);
}

impl std::ops::BitOr for VideoDecodeH264PictureLayoutFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoDecodeH264PictureLayoutFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoDecodeH264PictureLayoutFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoDecodeH264PictureLayoutFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoCodingControlFlagBitsKHR(pub u32);
impl VideoCodingControlFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const RESET_BIT_KHR: Self = Self(1);
	pub const ENCODE_RATE_CONTROL_BIT_KHR: Self = Self(2);
	pub const ENCODE_QUALITY_LEVEL_BIT_KHR: Self = Self(4);
}

impl std::ops::BitOr for VideoCodingControlFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoCodingControlFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoCodingControlFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoCodingControlFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct QueryResultStatusKHR(pub i32);
impl QueryResultStatusKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ERROR_KHR: Self = Self(-1);
	pub const NOT_READY_KHR: Self = Self(0);
	pub const COMPLETE_KHR: Self = Self(1);
	pub const INSUFFICIENT_BITSTREAM_BUFFER_RANGE_KHR: Self = Self(-1000299000);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoDecodeUsageFlagBitsKHR(pub u32);
impl VideoDecodeUsageFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEFAULT_KHR: Self = Self(0);
	pub const TRANSCODING_BIT_KHR: Self = Self(1);
	pub const OFFLINE_BIT_KHR: Self = Self(2);
	pub const STREAMING_BIT_KHR: Self = Self(4);
}

impl std::ops::BitOr for VideoDecodeUsageFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoDecodeUsageFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoDecodeUsageFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoDecodeUsageFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoDecodeCapabilityFlagBitsKHR(pub u32);
impl VideoDecodeCapabilityFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DPB_AND_OUTPUT_COINCIDE_BIT_KHR: Self = Self(1);
	pub const DPB_AND_OUTPUT_DISTINCT_BIT_KHR: Self = Self(2);
}

impl std::ops::BitOr for VideoDecodeCapabilityFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoDecodeCapabilityFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoDecodeCapabilityFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoDecodeCapabilityFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoEncodeFlagBitsKHR(pub u32);
impl VideoEncodeFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const INTRA_REFRESH_BIT_KHR: Self = Self(4);
	pub const WITH_QUANTIZATION_DELTA_MAP_BIT_KHR: Self = Self(1);
	pub const WITH_EMPHASIS_MAP_BIT_KHR: Self = Self(2);
}

impl std::ops::BitOr for VideoEncodeFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoEncodeFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoEncodeFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoEncodeFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CooperativeMatrixFlagBitsEXT(pub u32);
impl CooperativeMatrixFlagBitsEXT {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const SATURATING_ACCUMULATION_BIT_EXT: Self = Self(1);
}

impl std::ops::BitOr for CooperativeMatrixFlagBitsEXT {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for CooperativeMatrixFlagBitsEXT {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for CooperativeMatrixFlagBitsEXT {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for CooperativeMatrixFlagBitsEXT {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoEncodeUsageFlagBitsKHR(pub u32);
impl VideoEncodeUsageFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEFAULT_KHR: Self = Self(0);
	pub const TRANSCODING_BIT_KHR: Self = Self(1);
	pub const STREAMING_BIT_KHR: Self = Self(2);
	pub const RECORDING_BIT_KHR: Self = Self(4);
	pub const CONFERENCING_BIT_KHR: Self = Self(8);
}

impl std::ops::BitOr for VideoEncodeUsageFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoEncodeUsageFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoEncodeUsageFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoEncodeUsageFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoEncodeContentFlagBitsKHR(pub u32);
impl VideoEncodeContentFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEFAULT_KHR: Self = Self(0);
	pub const CAMERA_BIT_KHR: Self = Self(1);
	pub const DESKTOP_BIT_KHR: Self = Self(2);
	pub const RENDERED_BIT_KHR: Self = Self(4);
}

impl std::ops::BitOr for VideoEncodeContentFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoEncodeContentFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoEncodeContentFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoEncodeContentFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoEncodeTuningModeKHR(pub i32);
impl VideoEncodeTuningModeKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEFAULT_KHR: Self = Self(0);
	pub const HIGH_QUALITY_KHR: Self = Self(1);
	pub const LOW_LATENCY_KHR: Self = Self(2);
	pub const ULTRA_LOW_LATENCY_KHR: Self = Self(3);
	pub const LOSSLESS_KHR: Self = Self(4);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoEncodeCapabilityFlagBitsKHR(pub u32);
impl VideoEncodeCapabilityFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const PRECEDING_EXTERNALLY_ENCODED_BYTES_BIT_KHR: Self = Self(1);
	pub const INSUFFICIENT_BITSTREAM_BUFFER_RANGE_DETECTION_BIT_KHR: Self = Self(2);
	pub const QUANTIZATION_DELTA_MAP_BIT_KHR: Self = Self(4);
	pub const EMPHASIS_MAP_BIT_KHR: Self = Self(8);
}

impl std::ops::BitOr for VideoEncodeCapabilityFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoEncodeCapabilityFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoEncodeCapabilityFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoEncodeCapabilityFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoEncodeFeedbackFlagBitsKHR(pub u32);
impl VideoEncodeFeedbackFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const BITSTREAM_BUFFER_OFFSET_BIT_KHR: Self = Self(1);
	pub const BITSTREAM_BYTES_WRITTEN_BIT_KHR: Self = Self(2);
	pub const BITSTREAM_HAS_OVERRIDES_BIT_KHR: Self = Self(4);
	pub const AVERAGE_QUANTIZATION_BIT_KHR: Self = Self(8);
	pub const MIN_QUANTIZATION_BIT_KHR: Self = Self(16);
	pub const MAX_QUANTIZATION_BIT_KHR: Self = Self(32);
	pub const INTRA_PIXELS_BIT_KHR: Self = Self(64);
	pub const INTER_PIXELS_BIT_KHR: Self = Self(128);
	pub const SKIPPED_PIXELS_BIT_KHR: Self = Self(256);
	pub const PICTURE_PARTITION_COUNT_BIT_KHR: Self = Self(512);
}

impl std::ops::BitOr for VideoEncodeFeedbackFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoEncodeFeedbackFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoEncodeFeedbackFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoEncodeFeedbackFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoEncodePerPartitionFeedbackFlagBitsKHR(pub u32);
impl VideoEncodePerPartitionFeedbackFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const STATUS_BIT_KHR: Self = Self(1);
	pub const BITSTREAM_BUFFER_OFFSET_BIT_KHR: Self = Self(2);
	pub const BITSTREAM_BYTES_WRITTEN_BIT_KHR: Self = Self(4);
}

impl std::ops::BitOr for VideoEncodePerPartitionFeedbackFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoEncodePerPartitionFeedbackFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoEncodePerPartitionFeedbackFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoEncodePerPartitionFeedbackFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoEncodeRateControlModeFlagBitsKHR(pub u32);
impl VideoEncodeRateControlModeFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEFAULT_KHR: Self = Self(0);
	pub const DISABLED_BIT_KHR: Self = Self(1);
	pub const CBR_BIT_KHR: Self = Self(2);
	pub const VBR_BIT_KHR: Self = Self(4);
}

impl std::ops::BitOr for VideoEncodeRateControlModeFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoEncodeRateControlModeFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoEncodeRateControlModeFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoEncodeRateControlModeFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoEncodeIntraRefreshModeFlagBitsKHR(pub u32);
impl VideoEncodeIntraRefreshModeFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const NONE_KHR: Self = Self(0);
	pub const PER_PICTURE_PARTITION_BIT_KHR: Self = Self(1);
	pub const BLOCK_BASED_BIT_KHR: Self = Self(2);
	pub const BLOCK_ROW_BASED_BIT_KHR: Self = Self(4);
	pub const BLOCK_COLUMN_BASED_BIT_KHR: Self = Self(8);
}

impl std::ops::BitOr for VideoEncodeIntraRefreshModeFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoEncodeIntraRefreshModeFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoEncodeIntraRefreshModeFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoEncodeIntraRefreshModeFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoEncodeH264CapabilityFlagBitsKHR(pub u32);
impl VideoEncodeH264CapabilityFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const HRD_COMPLIANCE_BIT_KHR: Self = Self(1);
	pub const PREDICTION_WEIGHT_TABLE_GENERATED_BIT_KHR: Self = Self(2);
	pub const ROW_UNALIGNED_SLICE_BIT_KHR: Self = Self(4);
	pub const DIFFERENT_SLICE_TYPE_BIT_KHR: Self = Self(8);
	pub const B_FRAME_IN_L0_LIST_BIT_KHR: Self = Self(16);
	pub const B_FRAME_IN_L1_LIST_BIT_KHR: Self = Self(32);
	pub const PER_PICTURE_TYPE_MIN_MAX_QP_BIT_KHR: Self = Self(64);
	pub const PER_SLICE_CONSTANT_QP_BIT_KHR: Self = Self(128);
	pub const GENERATE_PREFIX_NALU_BIT_KHR: Self = Self(256);
	pub const B_PICTURE_INTRA_REFRESH_BIT_KHR: Self = Self(1024);
	pub const MB_QP_DIFF_WRAPAROUND_BIT_KHR: Self = Self(512);
}

impl std::ops::BitOr for VideoEncodeH264CapabilityFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoEncodeH264CapabilityFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoEncodeH264CapabilityFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoEncodeH264CapabilityFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoEncodeH264StdFlagBitsKHR(pub u32);
impl VideoEncodeH264StdFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const SEPARATE_COLOR_PLANE_FLAG_SET_BIT_KHR: Self = Self(1);
	pub const QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_BIT_KHR: Self = Self(2);
	pub const SCALING_MATRIX_PRESENT_FLAG_SET_BIT_KHR: Self = Self(4);
	pub const CHROMA_QP_INDEX_OFFSET_BIT_KHR: Self = Self(8);
	pub const SECOND_CHROMA_QP_INDEX_OFFSET_BIT_KHR: Self = Self(16);
	pub const PIC_INIT_QP_MINUS26_BIT_KHR: Self = Self(32);
	pub const WEIGHTED_PRED_FLAG_SET_BIT_KHR: Self = Self(64);
	pub const WEIGHTED_BIPRED_IDC_EXPLICIT_BIT_KHR: Self = Self(128);
	pub const WEIGHTED_BIPRED_IDC_IMPLICIT_BIT_KHR: Self = Self(256);
	pub const TRANSFORM_8X8_MODE_FLAG_SET_BIT_KHR: Self = Self(512);
	pub const DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_BIT_KHR: Self = Self(1024);
	pub const ENTROPY_CODING_MODE_FLAG_UNSET_BIT_KHR: Self = Self(2048);
	pub const ENTROPY_CODING_MODE_FLAG_SET_BIT_KHR: Self = Self(4096);
	pub const DIRECT_8X8_INFERENCE_FLAG_UNSET_BIT_KHR: Self = Self(8192);
	pub const CONSTRAINED_INTRA_PRED_FLAG_SET_BIT_KHR: Self = Self(16384);
	pub const DEBLOCKING_FILTER_DISABLED_BIT_KHR: Self = Self(32768);
	pub const DEBLOCKING_FILTER_ENABLED_BIT_KHR: Self = Self(65536);
	pub const DEBLOCKING_FILTER_PARTIAL_BIT_KHR: Self = Self(131072);
	pub const SLICE_QP_DELTA_BIT_KHR: Self = Self(524288);
	pub const DIFFERENT_SLICE_QP_DELTA_BIT_KHR: Self = Self(1048576);
}

impl std::ops::BitOr for VideoEncodeH264StdFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoEncodeH264StdFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoEncodeH264StdFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoEncodeH264StdFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoEncodeH264RateControlFlagBitsKHR(pub u32);
impl VideoEncodeH264RateControlFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ATTEMPT_HRD_COMPLIANCE_BIT_KHR: Self = Self(1);
	pub const REGULAR_GOP_BIT_KHR: Self = Self(2);
	pub const REFERENCE_PATTERN_FLAT_BIT_KHR: Self = Self(4);
	pub const REFERENCE_PATTERN_DYADIC_BIT_KHR: Self = Self(8);
	pub const TEMPORAL_LAYER_PATTERN_DYADIC_BIT_KHR: Self = Self(16);
}

impl std::ops::BitOr for VideoEncodeH264RateControlFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoEncodeH264RateControlFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoEncodeH264RateControlFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoEncodeH264RateControlFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct HostImageCopyFlagBits(pub u32);
impl HostImageCopyFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const MEMCPY_BIT: Self = Self(1);
	pub const MEMCPY: Self = Self(1);
	pub const MEMCPY_BIT_EXT: Self = Self(1);
	pub const MEMCPY_EXT: Self = Self(1);
}

impl std::ops::BitOr for HostImageCopyFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for HostImageCopyFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for HostImageCopyFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for HostImageCopyFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PartitionedAccelerationStructureOpTypeNV(pub i32);
impl PartitionedAccelerationStructureOpTypeNV {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const WRITE_INSTANCE_NV: Self = Self(0);
	pub const UPDATE_INSTANCE_NV: Self = Self(1);
	pub const WRITE_PARTITION_TRANSLATION_NV: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PartitionedAccelerationStructureInstanceFlagBitsNV(pub u32);
impl PartitionedAccelerationStructureInstanceFlagBitsNV {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const FLAG_TRIANGLE_FACING_CULL_DISABLE_BIT_NV: Self = Self(1);
	pub const FLAG_TRIANGLE_FLIP_FACING_BIT_NV: Self = Self(2);
	pub const FLAG_FORCE_OPAQUE_BIT_NV: Self = Self(4);
	pub const FLAG_FORCE_NO_OPAQUE_BIT_NV: Self = Self(8);
	pub const FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV: Self = Self(16);
}

impl std::ops::BitOr for PartitionedAccelerationStructureInstanceFlagBitsNV {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for PartitionedAccelerationStructureInstanceFlagBitsNV {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for PartitionedAccelerationStructureInstanceFlagBitsNV {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for PartitionedAccelerationStructureInstanceFlagBitsNV {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[cfg(feature = "platform-fuchsia")]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ImageConstraintsInfoFlagBitsFUCHSIA(pub u32);
#[cfg(feature = "platform-fuchsia")]
impl ImageConstraintsInfoFlagBitsFUCHSIA {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const CPU_READ_RARELY_FUCHSIA: Self = Self(1);
	pub const CPU_READ_OFTEN_FUCHSIA: Self = Self(2);
	pub const CPU_WRITE_RARELY_FUCHSIA: Self = Self(4);
	pub const CPU_WRITE_OFTEN_FUCHSIA: Self = Self(8);
	pub const PROTECTED_OPTIONAL_FUCHSIA: Self = Self(16);
}

#[cfg(feature = "platform-fuchsia")]
impl std::ops::BitOr for ImageConstraintsInfoFlagBitsFUCHSIA {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

#[cfg(feature = "platform-fuchsia")]
impl std::ops::BitAnd for ImageConstraintsInfoFlagBitsFUCHSIA {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

#[cfg(feature = "platform-fuchsia")]
impl std::ops::BitXor for ImageConstraintsInfoFlagBitsFUCHSIA {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

#[cfg(feature = "platform-fuchsia")]
impl std::ops::Not for ImageConstraintsInfoFlagBitsFUCHSIA {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FormatFeatureFlagBits2(pub u64);
impl FormatFeatureFlagBits2 {
	pub const fn from_raw(value: u64) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u64 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const SAMPLED_IMAGE_BIT: Self = Self(1);
	pub const STORAGE_IMAGE_BIT: Self = Self(2);
	pub const STORAGE_IMAGE_ATOMIC_BIT: Self = Self(4);
	pub const UNIFORM_TEXEL_BUFFER_BIT: Self = Self(8);
	pub const STORAGE_TEXEL_BUFFER_BIT: Self = Self(16);
	pub const STORAGE_TEXEL_BUFFER_ATOMIC_BIT: Self = Self(32);
	pub const VERTEX_BUFFER_BIT: Self = Self(64);
	pub const COLOR_ATTACHMENT_BIT: Self = Self(128);
	pub const COLOR_ATTACHMENT_BLEND_BIT: Self = Self(256);
	pub const DEPTH_STENCIL_ATTACHMENT_BIT: Self = Self(512);
	pub const BLIT_SRC_BIT: Self = Self(1024);
	pub const BLIT_DST_BIT: Self = Self(2048);
	pub const SAMPLED_IMAGE_FILTER_LINEAR_BIT: Self = Self(4096);
	pub const TRANSFER_SRC_BIT: Self = Self(16384);
	pub const TRANSFER_DST_BIT: Self = Self(32768);
	pub const SAMPLED_IMAGE_FILTER_MINMAX_BIT: Self = Self(65536);
	pub const MIDPOINT_CHROMA_SAMPLES_BIT: Self = Self(131072);
	pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT: Self = Self(262144);
	pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT: Self = Self(524288);
	pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT: Self = Self(1048576);
	pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT: Self =
		Self(2097152);
	pub const DISJOINT_BIT: Self = Self(4194304);
	pub const COSITED_CHROMA_SAMPLES_BIT: Self = Self(8388608);
	pub const STORAGE_READ_WITHOUT_FORMAT_BIT: Self = Self(2147483648);
	pub const STORAGE_WRITE_WITHOUT_FORMAT_BIT: Self = Self(4294967296);
	pub const SAMPLED_IMAGE_DEPTH_COMPARISON_BIT: Self = Self(8589934592);
	pub const SAMPLED_IMAGE_FILTER_CUBIC_BIT: Self = Self(8192);
	pub const HOST_IMAGE_TRANSFER_BIT: Self = Self(70368744177664);
	pub const VIDEO_DECODE_OUTPUT_BIT_KHR: Self = Self(33554432);
	pub const VIDEO_DECODE_DPB_BIT_KHR: Self = Self(67108864);
	pub const ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR: Self = Self(536870912);
	pub const FRAGMENT_DENSITY_MAP_BIT_EXT: Self = Self(16777216);
	pub const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: Self = Self(1073741824);
	pub const HOST_IMAGE_TRANSFER_BIT_EXT: Self = Self(70368744177664);
	pub const VIDEO_ENCODE_INPUT_BIT_KHR: Self = Self(134217728);
	pub const VIDEO_ENCODE_DPB_BIT_KHR: Self = Self(268435456);
	pub const BLOCK_MATCHING_SXD_BIT_QCOM: Self = Self(17592186044416);
	pub const SAMPLED_IMAGE_BIT_KHR: Self = Self(1);
	pub const STORAGE_IMAGE_BIT_KHR: Self = Self(2);
	pub const STORAGE_IMAGE_ATOMIC_BIT_KHR: Self = Self(4);
	pub const UNIFORM_TEXEL_BUFFER_BIT_KHR: Self = Self(8);
	pub const STORAGE_TEXEL_BUFFER_BIT_KHR: Self = Self(16);
	pub const STORAGE_TEXEL_BUFFER_ATOMIC_BIT_KHR: Self = Self(32);
	pub const VERTEX_BUFFER_BIT_KHR: Self = Self(64);
	pub const COLOR_ATTACHMENT_BIT_KHR: Self = Self(128);
	pub const COLOR_ATTACHMENT_BLEND_BIT_KHR: Self = Self(256);
	pub const DEPTH_STENCIL_ATTACHMENT_BIT_KHR: Self = Self(512);
	pub const BLIT_SRC_BIT_KHR: Self = Self(1024);
	pub const BLIT_DST_BIT_KHR: Self = Self(2048);
	pub const SAMPLED_IMAGE_FILTER_LINEAR_BIT_KHR: Self = Self(4096);
	pub const TRANSFER_SRC_BIT_KHR: Self = Self(16384);
	pub const TRANSFER_DST_BIT_KHR: Self = Self(32768);
	pub const MIDPOINT_CHROMA_SAMPLES_BIT_KHR: Self = Self(131072);
	pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR: Self = Self(262144);
	pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR: Self =
		Self(524288);
	pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR: Self =
		Self(1048576);
	pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR: Self =
		Self(2097152);
	pub const DISJOINT_BIT_KHR: Self = Self(4194304);
	pub const COSITED_CHROMA_SAMPLES_BIT_KHR: Self = Self(8388608);
	pub const STORAGE_READ_WITHOUT_FORMAT_BIT_KHR: Self = Self(2147483648);
	pub const STORAGE_WRITE_WITHOUT_FORMAT_BIT_KHR: Self = Self(4294967296);
	pub const SAMPLED_IMAGE_DEPTH_COMPARISON_BIT_KHR: Self = Self(8589934592);
	pub const SAMPLED_IMAGE_FILTER_MINMAX_BIT_KHR: Self = Self(65536);
	pub const SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT: Self = Self(8192);
	pub const ACCELERATION_STRUCTURE_RADIUS_BUFFER_BIT_NV: Self = Self(2251799813685248);
	pub const LINEAR_COLOR_ATTACHMENT_BIT_NV: Self = Self(274877906944);
	pub const WEIGHT_IMAGE_BIT_QCOM: Self = Self(17179869184);
	pub const WEIGHT_SAMPLED_IMAGE_BIT_QCOM: Self = Self(34359738368);
	pub const BLOCK_MATCHING_BIT_QCOM: Self = Self(68719476736);
	pub const BOX_FILTER_SAMPLED_BIT_QCOM: Self = Self(137438953472);
	pub const TENSOR_SHADER_BIT_ARM: Self = Self(549755813888);
	pub const TENSOR_IMAGE_ALIASING_BIT_ARM: Self = Self(8796093022208);
	pub const OPTICAL_FLOW_IMAGE_BIT_NV: Self = Self(1099511627776);
	pub const OPTICAL_FLOW_VECTOR_BIT_NV: Self = Self(2199023255552);
	pub const OPTICAL_FLOW_COST_BIT_NV: Self = Self(4398046511104);
	pub const TENSOR_DATA_GRAPH_BIT_ARM: Self = Self(281474976710656);
	pub const COPY_IMAGE_INDIRECT_DST_BIT_KHR: Self = Self(576460752303423488);
	pub const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR: Self = Self(562949953421312);
	pub const VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR: Self = Self(1125899906842624);
	pub const SAMPLED_IMAGE_FILTER_LINEAR_2D_BIT_IMG: Self = Self(35184372088832);
	pub const DEPTH_COPY_ON_COMPUTE_QUEUE_BIT_KHR: Self = Self(4503599627370496);
	pub const DEPTH_COPY_ON_TRANSFER_QUEUE_BIT_KHR: Self = Self(9007199254740992);
	pub const STENCIL_COPY_ON_COMPUTE_QUEUE_BIT_KHR: Self = Self(18014398509481984);
	pub const STENCIL_COPY_ON_TRANSFER_QUEUE_BIT_KHR: Self = Self(36028797018963968);
	pub const DATA_GRAPH_OPTICAL_FLOW_IMAGE_BIT_ARM: Self = Self(72057594037927936);
	pub const DATA_GRAPH_OPTICAL_FLOW_VECTOR_BIT_ARM: Self = Self(144115188075855872);
	pub const DATA_GRAPH_OPTICAL_FLOW_COST_BIT_ARM: Self = Self(288230376151711744);
}

impl std::ops::BitOr for FormatFeatureFlagBits2 {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for FormatFeatureFlagBits2 {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for FormatFeatureFlagBits2 {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for FormatFeatureFlagBits2 {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FormatFeatureFlagBits4KHR(pub u64);
impl FormatFeatureFlagBits4KHR {
	pub const fn from_raw(value: u64) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u64 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}
}

impl std::ops::BitOr for FormatFeatureFlagBits4KHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for FormatFeatureFlagBits4KHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for FormatFeatureFlagBits4KHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for FormatFeatureFlagBits4KHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct RenderingFlagBits(pub u32);
impl RenderingFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT: Self = Self(1);
	pub const SUSPENDING_BIT: Self = Self(2);
	pub const RESUMING_BIT: Self = Self(4);
	pub const CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT_KHR: Self = Self(1);
	pub const SUSPENDING_BIT_KHR: Self = Self(2);
	pub const RESUMING_BIT_KHR: Self = Self(4);
	pub const CONTENTS_INLINE_BIT_EXT: Self = Self(16);
	pub const ENABLE_LEGACY_DITHERING_BIT_EXT: Self = Self(8);
	pub const CONTENTS_INLINE_BIT_KHR: Self = Self(16);
	pub const PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE: Self = Self(32);
	pub const FRAGMENT_REGION_BIT_EXT: Self = Self(64);
	pub const CUSTOM_RESOLVE_BIT_EXT: Self = Self(128);
	pub const LOCAL_READ_CONCURRENT_ACCESS_CONTROL_BIT_KHR: Self = Self(256);
}

impl std::ops::BitOr for RenderingFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for RenderingFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for RenderingFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for RenderingFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoEncodeH265CapabilityFlagBitsKHR(pub u32);
impl VideoEncodeH265CapabilityFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const HRD_COMPLIANCE_BIT_KHR: Self = Self(1);
	pub const PREDICTION_WEIGHT_TABLE_GENERATED_BIT_KHR: Self = Self(2);
	pub const ROW_UNALIGNED_SLICE_SEGMENT_BIT_KHR: Self = Self(4);
	pub const DIFFERENT_SLICE_SEGMENT_TYPE_BIT_KHR: Self = Self(8);
	pub const B_FRAME_IN_L0_LIST_BIT_KHR: Self = Self(16);
	pub const B_FRAME_IN_L1_LIST_BIT_KHR: Self = Self(32);
	pub const PER_PICTURE_TYPE_MIN_MAX_QP_BIT_KHR: Self = Self(64);
	pub const PER_SLICE_SEGMENT_CONSTANT_QP_BIT_KHR: Self = Self(128);
	pub const MULTIPLE_TILES_PER_SLICE_SEGMENT_BIT_KHR: Self = Self(256);
	pub const MULTIPLE_SLICE_SEGMENTS_PER_TILE_BIT_KHR: Self = Self(512);
	pub const B_PICTURE_INTRA_REFRESH_BIT_KHR: Self = Self(2048);
	pub const CU_QP_DIFF_WRAPAROUND_BIT_KHR: Self = Self(1024);
}

impl std::ops::BitOr for VideoEncodeH265CapabilityFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoEncodeH265CapabilityFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoEncodeH265CapabilityFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoEncodeH265CapabilityFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoEncodeH265StdFlagBitsKHR(pub u32);
impl VideoEncodeH265StdFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const SEPARATE_COLOR_PLANE_FLAG_SET_BIT_KHR: Self = Self(1);
	pub const SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG_SET_BIT_KHR: Self = Self(2);
	pub const SCALING_LIST_DATA_PRESENT_FLAG_SET_BIT_KHR: Self = Self(4);
	pub const PCM_ENABLED_FLAG_SET_BIT_KHR: Self = Self(8);
	pub const SPS_TEMPORAL_MVP_ENABLED_FLAG_SET_BIT_KHR: Self = Self(16);
	pub const INIT_QP_MINUS26_BIT_KHR: Self = Self(32);
	pub const WEIGHTED_PRED_FLAG_SET_BIT_KHR: Self = Self(64);
	pub const WEIGHTED_BIPRED_FLAG_SET_BIT_KHR: Self = Self(128);
	pub const LOG2_PARALLEL_MERGE_LEVEL_MINUS2_BIT_KHR: Self = Self(256);
	pub const SIGN_DATA_HIDING_ENABLED_FLAG_SET_BIT_KHR: Self = Self(512);
	pub const TRANSFORM_SKIP_ENABLED_FLAG_SET_BIT_KHR: Self = Self(1024);
	pub const TRANSFORM_SKIP_ENABLED_FLAG_UNSET_BIT_KHR: Self = Self(2048);
	pub const PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG_SET_BIT_KHR: Self = Self(4096);
	pub const TRANSQUANT_BYPASS_ENABLED_FLAG_SET_BIT_KHR: Self = Self(8192);
	pub const CONSTRAINED_INTRA_PRED_FLAG_SET_BIT_KHR: Self = Self(16384);
	pub const ENTROPY_CODING_SYNC_ENABLED_FLAG_SET_BIT_KHR: Self = Self(32768);
	pub const DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG_SET_BIT_KHR: Self = Self(65536);
	pub const DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG_SET_BIT_KHR: Self = Self(131072);
	pub const DEPENDENT_SLICE_SEGMENT_FLAG_SET_BIT_KHR: Self = Self(262144);
	pub const SLICE_QP_DELTA_BIT_KHR: Self = Self(524288);
	pub const DIFFERENT_SLICE_QP_DELTA_BIT_KHR: Self = Self(1048576);
}

impl std::ops::BitOr for VideoEncodeH265StdFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoEncodeH265StdFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoEncodeH265StdFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoEncodeH265StdFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoEncodeH265RateControlFlagBitsKHR(pub u32);
impl VideoEncodeH265RateControlFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ATTEMPT_HRD_COMPLIANCE_BIT_KHR: Self = Self(1);
	pub const REGULAR_GOP_BIT_KHR: Self = Self(2);
	pub const REFERENCE_PATTERN_FLAT_BIT_KHR: Self = Self(4);
	pub const REFERENCE_PATTERN_DYADIC_BIT_KHR: Self = Self(8);
	pub const TEMPORAL_SUB_LAYER_PATTERN_DYADIC_BIT_KHR: Self = Self(16);
}

impl std::ops::BitOr for VideoEncodeH265RateControlFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoEncodeH265RateControlFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoEncodeH265RateControlFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoEncodeH265RateControlFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoEncodeH265CtbSizeFlagBitsKHR(pub u32);
impl VideoEncodeH265CtbSizeFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const _16_BIT_KHR: Self = Self(1);
	pub const _32_BIT_KHR: Self = Self(2);
	pub const _64_BIT_KHR: Self = Self(4);
}

impl std::ops::BitOr for VideoEncodeH265CtbSizeFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoEncodeH265CtbSizeFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoEncodeH265CtbSizeFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoEncodeH265CtbSizeFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoEncodeH265TransformBlockSizeFlagBitsKHR(pub u32);
impl VideoEncodeH265TransformBlockSizeFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const _4_BIT_KHR: Self = Self(1);
	pub const _8_BIT_KHR: Self = Self(2);
	pub const _16_BIT_KHR: Self = Self(4);
	pub const _32_BIT_KHR: Self = Self(8);
}

impl std::ops::BitOr for VideoEncodeH265TransformBlockSizeFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoEncodeH265TransformBlockSizeFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoEncodeH265TransformBlockSizeFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoEncodeH265TransformBlockSizeFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoEncodeAV1CapabilityFlagBitsKHR(pub u32);
impl VideoEncodeAV1CapabilityFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const PER_RATE_CONTROL_GROUP_MIN_MAX_Q_INDEX_BIT_KHR: Self = Self(1);
	pub const GENERATE_OBU_EXTENSION_HEADER_BIT_KHR: Self = Self(2);
	pub const PRIMARY_REFERENCE_CDF_ONLY_BIT_KHR: Self = Self(4);
	pub const FRAME_SIZE_OVERRIDE_BIT_KHR: Self = Self(8);
	pub const MOTION_VECTOR_SCALING_BIT_KHR: Self = Self(16);
	pub const COMPOUND_PREDICTION_INTRA_REFRESH_BIT_KHR: Self = Self(32);
}

impl std::ops::BitOr for VideoEncodeAV1CapabilityFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoEncodeAV1CapabilityFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoEncodeAV1CapabilityFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoEncodeAV1CapabilityFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoEncodeAV1StdFlagBitsKHR(pub u32);
impl VideoEncodeAV1StdFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const UNIFORM_TILE_SPACING_FLAG_SET_BIT_KHR: Self = Self(1);
	pub const SKIP_MODE_PRESENT_UNSET_BIT_KHR: Self = Self(2);
	pub const PRIMARY_REF_FRAME_BIT_KHR: Self = Self(4);
	pub const DELTA_Q_BIT_KHR: Self = Self(8);
}

impl std::ops::BitOr for VideoEncodeAV1StdFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoEncodeAV1StdFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoEncodeAV1StdFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoEncodeAV1StdFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoEncodeAV1RateControlFlagBitsKHR(pub u32);
impl VideoEncodeAV1RateControlFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const REGULAR_GOP_BIT_KHR: Self = Self(1);
	pub const TEMPORAL_LAYER_PATTERN_DYADIC_BIT_KHR: Self = Self(2);
	pub const REFERENCE_PATTERN_FLAT_BIT_KHR: Self = Self(4);
	pub const REFERENCE_PATTERN_DYADIC_BIT_KHR: Self = Self(8);
}

impl std::ops::BitOr for VideoEncodeAV1RateControlFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoEncodeAV1RateControlFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoEncodeAV1RateControlFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoEncodeAV1RateControlFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoEncodeAV1SuperblockSizeFlagBitsKHR(pub u32);
impl VideoEncodeAV1SuperblockSizeFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const _64_BIT_KHR: Self = Self(1);
	pub const _128_BIT_KHR: Self = Self(2);
}

impl std::ops::BitOr for VideoEncodeAV1SuperblockSizeFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoEncodeAV1SuperblockSizeFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoEncodeAV1SuperblockSizeFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoEncodeAV1SuperblockSizeFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoEncodeAV1PredictionModeKHR(pub i32);
impl VideoEncodeAV1PredictionModeKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const INTRA_ONLY_KHR: Self = Self(0);
	pub const SINGLE_REFERENCE_KHR: Self = Self(1);
	pub const UNIDIRECTIONAL_COMPOUND_KHR: Self = Self(2);
	pub const BIDIRECTIONAL_COMPOUND_KHR: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoEncodeAV1RateControlGroupKHR(pub i32);
impl VideoEncodeAV1RateControlGroupKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const INTRA_KHR: Self = Self(0);
	pub const PREDICTIVE_KHR: Self = Self(1);
	pub const BIPREDICTIVE_KHR: Self = Self(2);
}

#[cfg(feature = "platform-metal")]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ExportMetalObjectTypeFlagBitsEXT(pub u32);
#[cfg(feature = "platform-metal")]
impl ExportMetalObjectTypeFlagBitsEXT {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const METAL_DEVICE_BIT_EXT: Self = Self(1);
	pub const METAL_COMMAND_QUEUE_BIT_EXT: Self = Self(2);
	pub const METAL_BUFFER_BIT_EXT: Self = Self(4);
	pub const METAL_TEXTURE_BIT_EXT: Self = Self(8);
	pub const METAL_IOSURFACE_BIT_EXT: Self = Self(16);
	pub const METAL_SHARED_EVENT_BIT_EXT: Self = Self(32);
}

#[cfg(feature = "platform-metal")]
impl std::ops::BitOr for ExportMetalObjectTypeFlagBitsEXT {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

#[cfg(feature = "platform-metal")]
impl std::ops::BitAnd for ExportMetalObjectTypeFlagBitsEXT {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

#[cfg(feature = "platform-metal")]
impl std::ops::BitXor for ExportMetalObjectTypeFlagBitsEXT {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

#[cfg(feature = "platform-metal")]
impl std::ops::Not for ExportMetalObjectTypeFlagBitsEXT {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct InstanceCreateFlagBits(pub u32);
impl InstanceCreateFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ENUMERATE_PORTABILITY_BIT_KHR: Self = Self(1);
}

impl std::ops::BitOr for InstanceCreateFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for InstanceCreateFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for InstanceCreateFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for InstanceCreateFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ImageCompressionFlagBitsEXT(pub u32);
impl ImageCompressionFlagBitsEXT {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEFAULT_EXT: Self = Self(0);
	pub const FIXED_RATE_DEFAULT_EXT: Self = Self(1);
	pub const FIXED_RATE_EXPLICIT_EXT: Self = Self(2);
	pub const DISABLED_EXT: Self = Self(4);
}

impl std::ops::BitOr for ImageCompressionFlagBitsEXT {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ImageCompressionFlagBitsEXT {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ImageCompressionFlagBitsEXT {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ImageCompressionFlagBitsEXT {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ImageCompressionFixedRateFlagBitsEXT(pub u32);
impl ImageCompressionFixedRateFlagBitsEXT {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const NONE_EXT: Self = Self(0);
	pub const _1BPC_BIT_EXT: Self = Self(1);
	pub const _2BPC_BIT_EXT: Self = Self(2);
	pub const _3BPC_BIT_EXT: Self = Self(4);
	pub const _4BPC_BIT_EXT: Self = Self(8);
	pub const _5BPC_BIT_EXT: Self = Self(16);
	pub const _6BPC_BIT_EXT: Self = Self(32);
	pub const _7BPC_BIT_EXT: Self = Self(64);
	pub const _8BPC_BIT_EXT: Self = Self(128);
	pub const _9BPC_BIT_EXT: Self = Self(256);
	pub const _10BPC_BIT_EXT: Self = Self(512);
	pub const _11BPC_BIT_EXT: Self = Self(1024);
	pub const _12BPC_BIT_EXT: Self = Self(2048);
	pub const _13BPC_BIT_EXT: Self = Self(4096);
	pub const _14BPC_BIT_EXT: Self = Self(8192);
	pub const _15BPC_BIT_EXT: Self = Self(16384);
	pub const _16BPC_BIT_EXT: Self = Self(32768);
	pub const _17BPC_BIT_EXT: Self = Self(65536);
	pub const _18BPC_BIT_EXT: Self = Self(131072);
	pub const _19BPC_BIT_EXT: Self = Self(262144);
	pub const _20BPC_BIT_EXT: Self = Self(524288);
	pub const _21BPC_BIT_EXT: Self = Self(1048576);
	pub const _22BPC_BIT_EXT: Self = Self(2097152);
	pub const _23BPC_BIT_EXT: Self = Self(4194304);
	pub const _24BPC_BIT_EXT: Self = Self(8388608);
}

impl std::ops::BitOr for ImageCompressionFixedRateFlagBitsEXT {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ImageCompressionFixedRateFlagBitsEXT {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ImageCompressionFixedRateFlagBitsEXT {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ImageCompressionFixedRateFlagBitsEXT {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PipelineRobustnessBufferBehavior(pub i32);
impl PipelineRobustnessBufferBehavior {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEVICE_DEFAULT: Self = Self(0);
	pub const DISABLED: Self = Self(1);
	pub const ROBUST_BUFFER_ACCESS: Self = Self(2);
	pub const ROBUST_BUFFER_ACCESS_2: Self = Self(3);
	pub const DEVICE_DEFAULT_EXT: Self = Self(0);
	pub const DISABLED_EXT: Self = Self(1);
	pub const ROBUST_BUFFER_ACCESS_EXT: Self = Self(2);
	pub const ROBUST_BUFFER_ACCESS_2_EXT: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PipelineRobustnessImageBehavior(pub i32);
impl PipelineRobustnessImageBehavior {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEVICE_DEFAULT: Self = Self(0);
	pub const DISABLED: Self = Self(1);
	pub const ROBUST_IMAGE_ACCESS: Self = Self(2);
	pub const ROBUST_IMAGE_ACCESS_2: Self = Self(3);
	pub const DEVICE_DEFAULT_EXT: Self = Self(0);
	pub const DISABLED_EXT: Self = Self(1);
	pub const ROBUST_IMAGE_ACCESS_EXT: Self = Self(2);
	pub const ROBUST_IMAGE_ACCESS_2_EXT: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct OpticalFlowGridSizeFlagBitsNV(pub u32);
impl OpticalFlowGridSizeFlagBitsNV {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const UNKNOWN_NV: Self = Self(0);
	pub const _1X1_BIT_NV: Self = Self(1);
	pub const _2X2_BIT_NV: Self = Self(2);
	pub const _4X4_BIT_NV: Self = Self(4);
	pub const _8X8_BIT_NV: Self = Self(8);
}

impl std::ops::BitOr for OpticalFlowGridSizeFlagBitsNV {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for OpticalFlowGridSizeFlagBitsNV {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for OpticalFlowGridSizeFlagBitsNV {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for OpticalFlowGridSizeFlagBitsNV {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct OpticalFlowUsageFlagBitsNV(pub u32);
impl OpticalFlowUsageFlagBitsNV {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const UNKNOWN_NV: Self = Self(0);
	pub const INPUT_BIT_NV: Self = Self(1);
	pub const OUTPUT_BIT_NV: Self = Self(2);
	pub const HINT_BIT_NV: Self = Self(4);
	pub const COST_BIT_NV: Self = Self(8);
	pub const GLOBAL_FLOW_BIT_NV: Self = Self(16);
}

impl std::ops::BitOr for OpticalFlowUsageFlagBitsNV {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for OpticalFlowUsageFlagBitsNV {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for OpticalFlowUsageFlagBitsNV {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for OpticalFlowUsageFlagBitsNV {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct OpticalFlowPerformanceLevelNV(pub i32);
impl OpticalFlowPerformanceLevelNV {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const UNKNOWN_NV: Self = Self(0);
	pub const SLOW_NV: Self = Self(1);
	pub const MEDIUM_NV: Self = Self(2);
	pub const FAST_NV: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct OpticalFlowSessionBindingPointNV(pub i32);
impl OpticalFlowSessionBindingPointNV {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const UNKNOWN_NV: Self = Self(0);
	pub const INPUT_NV: Self = Self(1);
	pub const REFERENCE_NV: Self = Self(2);
	pub const HINT_NV: Self = Self(3);
	pub const FLOW_VECTOR_NV: Self = Self(4);
	pub const BACKWARD_FLOW_VECTOR_NV: Self = Self(5);
	pub const COST_NV: Self = Self(6);
	pub const BACKWARD_COST_NV: Self = Self(7);
	pub const GLOBAL_FLOW_NV: Self = Self(8);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct OpticalFlowSessionCreateFlagBitsNV(pub u32);
impl OpticalFlowSessionCreateFlagBitsNV {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ENABLE_HINT_BIT_NV: Self = Self(1);
	pub const ENABLE_COST_BIT_NV: Self = Self(2);
	pub const ENABLE_GLOBAL_FLOW_BIT_NV: Self = Self(4);
	pub const ALLOW_REGIONS_BIT_NV: Self = Self(8);
	pub const BOTH_DIRECTIONS_BIT_NV: Self = Self(16);
}

impl std::ops::BitOr for OpticalFlowSessionCreateFlagBitsNV {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for OpticalFlowSessionCreateFlagBitsNV {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for OpticalFlowSessionCreateFlagBitsNV {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for OpticalFlowSessionCreateFlagBitsNV {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct OpticalFlowExecuteFlagBitsNV(pub u32);
impl OpticalFlowExecuteFlagBitsNV {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DISABLE_TEMPORAL_HINTS_BIT_NV: Self = Self(1);
}

impl std::ops::BitOr for OpticalFlowExecuteFlagBitsNV {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for OpticalFlowExecuteFlagBitsNV {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for OpticalFlowExecuteFlagBitsNV {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for OpticalFlowExecuteFlagBitsNV {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct MicromapTypeEXT(pub i32);
impl MicromapTypeEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const OPACITY_MICROMAP_EXT: Self = Self(0);
	#[cfg(feature = "provisional")]
	pub const DISPLACEMENT_MICROMAP_NV: Self = Self(1000397000);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BuildMicromapFlagBitsEXT(pub u32);
impl BuildMicromapFlagBitsEXT {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const PREFER_FAST_TRACE_BIT_EXT: Self = Self(1);
	pub const PREFER_FAST_BUILD_BIT_EXT: Self = Self(2);
	pub const ALLOW_COMPACTION_BIT_EXT: Self = Self(4);
}

impl std::ops::BitOr for BuildMicromapFlagBitsEXT {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for BuildMicromapFlagBitsEXT {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for BuildMicromapFlagBitsEXT {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for BuildMicromapFlagBitsEXT {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct MicromapCreateFlagBitsEXT(pub u32);
impl MicromapCreateFlagBitsEXT {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT: Self = Self(1);
}

impl std::ops::BitOr for MicromapCreateFlagBitsEXT {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for MicromapCreateFlagBitsEXT {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for MicromapCreateFlagBitsEXT {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for MicromapCreateFlagBitsEXT {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CopyMicromapModeEXT(pub i32);
impl CopyMicromapModeEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const CLONE_EXT: Self = Self(0);
	pub const SERIALIZE_EXT: Self = Self(1);
	pub const DESERIALIZE_EXT: Self = Self(2);
	pub const COMPACT_EXT: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BuildMicromapModeEXT(pub i32);
impl BuildMicromapModeEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const BUILD_EXT: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct OpacityMicromapFormatKHR(pub i32);
impl OpacityMicromapFormatKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const _2_STATE_KHR: Self = Self(1);
	pub const _4_STATE_KHR: Self = Self(2);
	pub const _2_STATE_EXT: Self = Self(1);
	pub const _4_STATE_EXT: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct OpacityMicromapSpecialIndexKHR(pub i32);
impl OpacityMicromapSpecialIndexKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const FULLY_TRANSPARENT_KHR: Self = Self(-1);
	pub const FULLY_OPAQUE_KHR: Self = Self(-2);
	pub const FULLY_UNKNOWN_TRANSPARENT_KHR: Self = Self(-3);
	pub const FULLY_UNKNOWN_OPAQUE_KHR: Self = Self(-4);
	pub const FULLY_TRANSPARENT_EXT: Self = Self(-1);
	pub const FULLY_OPAQUE_EXT: Self = Self(-2);
	pub const FULLY_UNKNOWN_TRANSPARENT_EXT: Self = Self(-3);
	pub const FULLY_UNKNOWN_OPAQUE_EXT: Self = Self(-4);
	pub const CLUSTER_GEOMETRY_DISABLE_OPACITY_MICROMAP_NV: Self = Self(-5);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AccelerationStructureSerializedBlockTypeKHR(pub i32);
impl AccelerationStructureSerializedBlockTypeKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const OPACITY_MICROMAP_KHR: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DepthBiasRepresentationEXT(pub i32);
impl DepthBiasRepresentationEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const LEAST_REPRESENTABLE_VALUE_FORMAT_EXT: Self = Self(0);
	pub const LEAST_REPRESENTABLE_VALUE_FORCE_UNORM_EXT: Self = Self(1);
	pub const FLOAT_EXT: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DeviceFaultFlagBitsKHR(pub u32);
impl DeviceFaultFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const FLAG_DEVICE_LOST_KHR: Self = Self(1);
	pub const FLAG_MEMORY_ADDRESS_KHR: Self = Self(2);
	pub const FLAG_INSTRUCTION_ADDRESS_KHR: Self = Self(4);
	pub const FLAG_VENDOR_KHR: Self = Self(8);
	pub const FLAG_WATCHDOG_TIMEOUT_KHR: Self = Self(16);
	pub const FLAG_OVERFLOW_KHR: Self = Self(32);
}

impl std::ops::BitOr for DeviceFaultFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for DeviceFaultFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for DeviceFaultFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for DeviceFaultFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DeviceFaultAddressTypeKHR(pub i32);
impl DeviceFaultAddressTypeKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const NONE_KHR: Self = Self(0);
	pub const READ_INVALID_KHR: Self = Self(1);
	pub const WRITE_INVALID_KHR: Self = Self(2);
	pub const EXECUTE_INVALID_KHR: Self = Self(3);
	pub const INSTRUCTION_POINTER_UNKNOWN_KHR: Self = Self(4);
	pub const INSTRUCTION_POINTER_INVALID_KHR: Self = Self(5);
	pub const INSTRUCTION_POINTER_FAULT_KHR: Self = Self(6);
	pub const NONE_EXT: Self = Self(0);
	pub const READ_INVALID_EXT: Self = Self(1);
	pub const WRITE_INVALID_EXT: Self = Self(2);
	pub const EXECUTE_INVALID_EXT: Self = Self(3);
	pub const INSTRUCTION_POINTER_UNKNOWN_EXT: Self = Self(4);
	pub const INSTRUCTION_POINTER_INVALID_EXT: Self = Self(5);
	pub const INSTRUCTION_POINTER_FAULT_EXT: Self = Self(6);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DeviceFaultVendorBinaryHeaderVersionKHR(pub i32);
impl DeviceFaultVendorBinaryHeaderVersionKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ONE_KHR: Self = Self(1);
	pub const ONE_EXT: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct IndirectCommandsLayoutUsageFlagBitsEXT(pub u32);
impl IndirectCommandsLayoutUsageFlagBitsEXT {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const EXPLICIT_PREPROCESS_BIT_EXT: Self = Self(1);
	pub const UNORDERED_SEQUENCES_BIT_EXT: Self = Self(2);
}

impl std::ops::BitOr for IndirectCommandsLayoutUsageFlagBitsEXT {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for IndirectCommandsLayoutUsageFlagBitsEXT {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for IndirectCommandsLayoutUsageFlagBitsEXT {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for IndirectCommandsLayoutUsageFlagBitsEXT {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct IndirectExecutionSetInfoTypeEXT(pub i32);
impl IndirectExecutionSetInfoTypeEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const PIPELINES_EXT: Self = Self(0);
	pub const SHADER_OBJECTS_EXT: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct IndirectCommandsInputModeFlagBitsEXT(pub u32);
impl IndirectCommandsInputModeFlagBitsEXT {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const VULKAN_INDEX_BUFFER_EXT: Self = Self(1);
	pub const DXGI_INDEX_BUFFER_EXT: Self = Self(2);
}

impl std::ops::BitOr for IndirectCommandsInputModeFlagBitsEXT {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for IndirectCommandsInputModeFlagBitsEXT {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for IndirectCommandsInputModeFlagBitsEXT {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for IndirectCommandsInputModeFlagBitsEXT {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct IndirectCommandsTokenTypeEXT(pub i32);
impl IndirectCommandsTokenTypeEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const EXECUTION_SET_EXT: Self = Self(0);
	pub const PUSH_CONSTANT_EXT: Self = Self(1);
	pub const SEQUENCE_INDEX_EXT: Self = Self(2);
	pub const INDEX_BUFFER_EXT: Self = Self(3);
	pub const VERTEX_BUFFER_EXT: Self = Self(4);
	pub const DRAW_INDEXED_EXT: Self = Self(5);
	pub const DRAW_EXT: Self = Self(6);
	pub const DRAW_INDEXED_COUNT_EXT: Self = Self(7);
	pub const DRAW_COUNT_EXT: Self = Self(8);
	pub const DISPATCH_EXT: Self = Self(9);
	pub const PUSH_DATA_EXT: Self = Self(1000135000);
	pub const PUSH_DATA_SEQUENCE_INDEX_EXT: Self = Self(1000135001);
	pub const DRAW_MESH_TASKS_NV_EXT: Self = Self(1000202002);
	pub const DRAW_MESH_TASKS_COUNT_NV_EXT: Self = Self(1000202003);
	pub const DRAW_MESH_TASKS_EXT: Self = Self(1000328000);
	pub const DRAW_MESH_TASKS_COUNT_EXT: Self = Self(1000328001);
	pub const TRACE_RAYS2_EXT: Self = Self(1000386004);
}

#[cfg(feature = "provisional")]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DisplacementMicromapFormatNV(pub i32);
#[cfg(feature = "provisional")]
impl DisplacementMicromapFormatNV {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const _64_TRIANGLES_64_BYTES_NV: Self = Self(1);
	pub const _256_TRIANGLES_128_BYTES_NV: Self = Self(2);
	pub const _1024_TRIANGLES_128_BYTES_NV: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ShaderCreateFlagBitsEXT(pub u32);
impl ShaderCreateFlagBitsEXT {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const LINK_STAGE_BIT_EXT: Self = Self(1);
	pub const DESCRIPTOR_HEAP_BIT_EXT: Self = Self(1024);
	pub const INSTRUMENT_SHADER_BIT_ARM: Self = Self(2048);
	pub const ALLOW_VARYING_SUBGROUP_SIZE_BIT_EXT: Self = Self(2);
	pub const REQUIRE_FULL_SUBGROUPS_BIT_EXT: Self = Self(4);
	pub const NO_TASK_SHADER_BIT_EXT: Self = Self(8);
	pub const DISPATCH_BASE_BIT_EXT: Self = Self(16);
	pub const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_EXT: Self = Self(32);
	pub const FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT: Self = Self(64);
	pub const INDIRECT_BINDABLE_BIT_EXT: Self = Self(128);
	pub const OPACITY_MICROMAP_DISALLOW_MIXED_SPECIAL_INDEX_BIT_EXT: Self = Self(4096);
	pub const _64_BIT_INDEXING_BIT_EXT: Self = Self(32768);
	pub const INDEPENDENT_SETS_BIT_KHR: Self = Self(262144);
}

impl std::ops::BitOr for ShaderCreateFlagBitsEXT {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for ShaderCreateFlagBitsEXT {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for ShaderCreateFlagBitsEXT {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for ShaderCreateFlagBitsEXT {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ShaderCodeTypeEXT(pub i32);
impl ShaderCodeTypeEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const BINARY_EXT: Self = Self(0);
	pub const SPIRV_EXT: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ScopeKHR(pub i32);
impl ScopeKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEVICE_KHR: Self = Self(1);
	pub const WORKGROUP_KHR: Self = Self(2);
	pub const SUBGROUP_KHR: Self = Self(3);
	pub const QUEUE_FAMILY_KHR: Self = Self(5);
	pub const DEVICE_NV: Self = Self(1);
	pub const WORKGROUP_NV: Self = Self(2);
	pub const SUBGROUP_NV: Self = Self(3);
	pub const QUEUE_FAMILY_NV: Self = Self(5);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ComponentTypeKHR(pub i32);
impl ComponentTypeKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const FLOAT16_KHR: Self = Self(0);
	pub const FLOAT32_KHR: Self = Self(1);
	pub const FLOAT64_KHR: Self = Self(2);
	pub const SINT8_KHR: Self = Self(3);
	pub const SINT16_KHR: Self = Self(4);
	pub const SINT32_KHR: Self = Self(5);
	pub const SINT64_KHR: Self = Self(6);
	pub const UINT8_KHR: Self = Self(7);
	pub const UINT16_KHR: Self = Self(8);
	pub const UINT32_KHR: Self = Self(9);
	pub const UINT64_KHR: Self = Self(10);
	pub const BFLOAT16_KHR: Self = Self(1000141000);
	pub const FLOAT16_NV: Self = Self(0);
	pub const FLOAT32_NV: Self = Self(1);
	pub const FLOAT64_NV: Self = Self(2);
	pub const SINT8_NV: Self = Self(3);
	pub const SINT16_NV: Self = Self(4);
	pub const SINT32_NV: Self = Self(5);
	pub const SINT64_NV: Self = Self(6);
	pub const UINT8_NV: Self = Self(7);
	pub const UINT16_NV: Self = Self(8);
	pub const UINT32_NV: Self = Self(9);
	pub const UINT64_NV: Self = Self(10);
	pub const SINT8_PACKED_NV: Self = Self(1000491000);
	pub const UINT8_PACKED_NV: Self = Self(1000491001);
	pub const FLOAT_E4M3_NV: Self = Self(1000491002);
	pub const FLOAT_E5M2_NV: Self = Self(1000491003);
	pub const FLOAT8_E4M3_EXT: Self = Self(1000491002);
	pub const FLOAT8_E5M2_EXT: Self = Self(1000491003);
	pub const FLOAT6_E2M3_EXT: Self = Self(1000672000);
	pub const FLOAT6_E3M2_EXT: Self = Self(1000672001);
	pub const FLOAT4_E2M1_EXT: Self = Self(1000672002);
	pub const FLOAT8_UNSIGNED_E8M0_EXT: Self = Self(1000672003);
	pub const MXINT8_EXT: Self = Self(1000672004);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CubicFilterWeightsQCOM(pub i32);
impl CubicFilterWeightsQCOM {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const CATMULL_ROM_QCOM: Self = Self(0);
	pub const ZERO_TANGENT_CARDINAL_QCOM: Self = Self(1);
	pub const B_SPLINE_QCOM: Self = Self(2);
	pub const MITCHELL_NETRAVALI_QCOM: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BlockMatchWindowCompareModeQCOM(pub i32);
impl BlockMatchWindowCompareModeQCOM {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const MIN_QCOM: Self = Self(0);
	pub const MAX_QCOM: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PhysicalDeviceLayeredApiKHR(pub i32);
impl PhysicalDeviceLayeredApiKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const VULKAN_KHR: Self = Self(0);
	pub const D3D12_KHR: Self = Self(1);
	pub const METAL_KHR: Self = Self(2);
	pub const OPENGL_KHR: Self = Self(3);
	pub const OPENGLES_KHR: Self = Self(4);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct LayeredDriverUnderlyingApiMSFT(pub i32);
impl LayeredDriverUnderlyingApiMSFT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const LAYERED_DRIVER_UNDERLYING_API_NONE_MSFT: Self = Self(0);
	pub const LAYERED_DRIVER_UNDERLYING_API_D3D12_MSFT: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct LatencyMarkerNV(pub i32);
impl LatencyMarkerNV {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const SIMULATION_START_NV: Self = Self(0);
	pub const SIMULATION_END_NV: Self = Self(1);
	pub const RENDERSUBMIT_START_NV: Self = Self(2);
	pub const RENDERSUBMIT_END_NV: Self = Self(3);
	pub const PRESENT_START_NV: Self = Self(4);
	pub const PRESENT_END_NV: Self = Self(5);
	pub const INPUT_SAMPLE_NV: Self = Self(6);
	pub const TRIGGER_FLASH_NV: Self = Self(7);
	pub const OUT_OF_BAND_RENDERSUBMIT_START_NV: Self = Self(8);
	pub const OUT_OF_BAND_RENDERSUBMIT_END_NV: Self = Self(9);
	pub const OUT_OF_BAND_PRESENT_START_NV: Self = Self(10);
	pub const OUT_OF_BAND_PRESENT_END_NV: Self = Self(11);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct OutOfBandQueueTypeNV(pub i32);
impl OutOfBandQueueTypeNV {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const RENDER_NV: Self = Self(0);
	pub const PRESENT_NV: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct MemoryUnmapFlagBits(pub u32);
impl MemoryUnmapFlagBits {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const RESERVE_BIT_EXT: Self = Self(1);
}

impl std::ops::BitOr for MemoryUnmapFlagBits {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for MemoryUnmapFlagBits {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for MemoryUnmapFlagBits {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for MemoryUnmapFlagBits {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[cfg(feature = "provisional")]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CompressedTriangleFormatAMDX(pub i32);
#[cfg(feature = "provisional")]
impl CompressedTriangleFormatAMDX {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const COMPRESSED_TRIANGLE_FORMAT_DGF1_AMDX: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DepthClampModeEXT(pub i32);
impl DepthClampModeEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const VIEWPORT_RANGE_EXT: Self = Self(0);
	pub const USER_DEFINED_RANGE_EXT: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AccessFlagBits3KHR(pub u64);
impl AccessFlagBits3KHR {
	pub const fn from_raw(value: u64) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u64 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const NONE_KHR: Self = Self(0);
}

impl std::ops::BitOr for AccessFlagBits3KHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for AccessFlagBits3KHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for AccessFlagBits3KHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for AccessFlagBits3KHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TileShadingRenderPassFlagBitsQCOM(pub u32);
impl TileShadingRenderPassFlagBitsQCOM {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ENABLE_BIT_QCOM: Self = Self(1);
	pub const PER_TILE_EXECUTION_BIT_QCOM: Self = Self(2);
}

impl std::ops::BitOr for TileShadingRenderPassFlagBitsQCOM {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for TileShadingRenderPassFlagBitsQCOM {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for TileShadingRenderPassFlagBitsQCOM {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for TileShadingRenderPassFlagBitsQCOM {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CooperativeVectorMatrixLayoutNV(pub i32);
impl CooperativeVectorMatrixLayoutNV {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ROW_MAJOR_NV: Self = Self(0);
	pub const COLUMN_MAJOR_NV: Self = Self(1);
	pub const INFERENCING_OPTIMAL_NV: Self = Self(2);
	pub const TRAINING_OPTIMAL_NV: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AddressCopyFlagBitsKHR(pub u32);
impl AddressCopyFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEVICE_LOCAL_BIT_KHR: Self = Self(1);
	pub const SPARSE_BIT_KHR: Self = Self(2);
	pub const PROTECTED_BIT_KHR: Self = Self(4);
}

impl std::ops::BitOr for AddressCopyFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for AddressCopyFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for AddressCopyFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for AddressCopyFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TensorCreateFlagBitsARM(pub u64);
impl TensorCreateFlagBitsARM {
	pub const fn from_raw(value: u64) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u64 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const MUTABLE_FORMAT_BIT_ARM: Self = Self(1);
	pub const PROTECTED_BIT_ARM: Self = Self(2);
	pub const DESCRIPTOR_HEAP_CAPTURE_REPLAY_BIT_ARM: Self = Self(8);
	pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_ARM: Self = Self(4);
}

impl std::ops::BitOr for TensorCreateFlagBitsARM {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for TensorCreateFlagBitsARM {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for TensorCreateFlagBitsARM {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for TensorCreateFlagBitsARM {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TensorUsageFlagBitsARM(pub u64);
impl TensorUsageFlagBitsARM {
	pub const fn from_raw(value: u64) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u64 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const SHADER_BIT_ARM: Self = Self(2);
	pub const TRANSFER_SRC_BIT_ARM: Self = Self(4);
	pub const TRANSFER_DST_BIT_ARM: Self = Self(8);
	pub const IMAGE_ALIASING_BIT_ARM: Self = Self(16);
	pub const DATA_GRAPH_BIT_ARM: Self = Self(32);
}

impl std::ops::BitOr for TensorUsageFlagBitsARM {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for TensorUsageFlagBitsARM {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for TensorUsageFlagBitsARM {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for TensorUsageFlagBitsARM {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TensorTilingARM(pub i32);
impl TensorTilingARM {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const OPTIMAL_ARM: Self = Self(0);
	pub const LINEAR_ARM: Self = Self(1);
	pub const BRICK_16_WIDE_ARM: Self = Self(1000565000);
	pub const BRICK_8_WIDE_ARM: Self = Self(1000565001);
	pub const BRICK_4_WIDE_ARM: Self = Self(1000565002);
	pub const BLOCK_U_INTERLEAVED_ARM: Self = Self(1000565003);
	pub const BLOCK_U_INTERLEAVED_64K_ARM: Self = Self(1000565004);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TensorViewCreateFlagBitsARM(pub u64);
impl TensorViewCreateFlagBitsARM {
	pub const fn from_raw(value: u64) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u64 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_ARM: Self = Self(1);
}

impl std::ops::BitOr for TensorViewCreateFlagBitsARM {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for TensorViewCreateFlagBitsARM {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for TensorViewCreateFlagBitsARM {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for TensorViewCreateFlagBitsARM {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DefaultVertexAttributeValueKHR(pub i32);
impl DefaultVertexAttributeValueKHR {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ZERO_ZERO_ZERO_ZERO_KHR: Self = Self(0);
	pub const ZERO_ZERO_ZERO_ONE_KHR: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DataGraphPipelineSessionCreateFlagBitsARM(pub u64);
impl DataGraphPipelineSessionCreateFlagBitsARM {
	pub const fn from_raw(value: u64) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u64 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const PROTECTED_BIT_ARM: Self = Self(1);
	pub const OPTICAL_FLOW_CACHE_BIT_ARM: Self = Self(2);
}

impl std::ops::BitOr for DataGraphPipelineSessionCreateFlagBitsARM {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for DataGraphPipelineSessionCreateFlagBitsARM {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for DataGraphPipelineSessionCreateFlagBitsARM {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for DataGraphPipelineSessionCreateFlagBitsARM {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DataGraphPipelineSessionBindPointARM(pub i32);
impl DataGraphPipelineSessionBindPointARM {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const TRANSIENT_ARM: Self = Self(0);
	pub const OPTICAL_FLOW_CACHE_ARM: Self = Self(1000631001);
	pub const NEURAL_ACCELERATOR_STATISTICS_ARM: Self = Self(1000676000);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DataGraphPipelineSessionBindPointTypeARM(pub i32);
impl DataGraphPipelineSessionBindPointTypeARM {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const MEMORY_ARM: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DataGraphPipelinePropertyARM(pub i32);
impl DataGraphPipelinePropertyARM {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const CREATION_LOG_ARM: Self = Self(0);
	pub const IDENTIFIER_ARM: Self = Self(1);
	pub const NEURAL_ACCELERATOR_DEBUG_DATABASE_ARM: Self = Self(1000676000);
	pub const NEURAL_ACCELERATOR_STATISTICS_INFO_ARM: Self = Self(1000676001);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DataGraphPipelineDispatchFlagBitsARM(pub u64);
impl DataGraphPipelineDispatchFlagBitsARM {
	pub const fn from_raw(value: u64) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u64 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}
}

impl std::ops::BitOr for DataGraphPipelineDispatchFlagBitsARM {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for DataGraphPipelineDispatchFlagBitsARM {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for DataGraphPipelineDispatchFlagBitsARM {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for DataGraphPipelineDispatchFlagBitsARM {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PhysicalDeviceDataGraphProcessingEngineTypeARM(pub i32);
impl PhysicalDeviceDataGraphProcessingEngineTypeARM {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEFAULT_ARM: Self = Self(0);
	pub const NEURAL_QCOM: Self = Self(1000629000);
	pub const COMPUTE_QCOM: Self = Self(1000629001);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PhysicalDeviceDataGraphOperationTypeARM(pub i32);
impl PhysicalDeviceDataGraphOperationTypeARM {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const SPIRV_EXTENDED_INSTRUCTION_SET_ARM: Self = Self(0);
	pub const NEURAL_MODEL_QCOM: Self = Self(1000629000);
	pub const BUILTIN_MODEL_QCOM: Self = Self(1000629001);
	pub const OPTICAL_FLOW_ARM: Self = Self(1000631000);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DataGraphModelCacheTypeQCOM(pub i32);
impl DataGraphModelCacheTypeQCOM {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const GENERIC_BINARY_QCOM: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PerfHintTypeQCOM(pub i32);
impl PerfHintTypeQCOM {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEFAULT_QCOM: Self = Self(0);
	pub const FREQUENCY_MIN_QCOM: Self = Self(1);
	pub const FREQUENCY_MAX_QCOM: Self = Self(2);
	pub const FREQUENCY_SCALED_QCOM: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ThrottleHintTypeSEC(pub i32);
impl ThrottleHintTypeSEC {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const THROTTLE_HINT_TYPE_DEFAULT_SEC: Self = Self(0);
	pub const THROTTLE_HINT_TYPE_LOW_SEC: Self = Self(1);
	pub const THROTTLE_HINT_TYPE_HIGH_SEC: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoEncodeRgbModelConversionFlagBitsVALVE(pub u32);
impl VideoEncodeRgbModelConversionFlagBitsVALVE {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const RGB_IDENTITY_BIT_VALVE: Self = Self(1);
	pub const YCBCR_IDENTITY_BIT_VALVE: Self = Self(2);
	pub const YCBCR_709_BIT_VALVE: Self = Self(4);
	pub const YCBCR_601_BIT_VALVE: Self = Self(8);
	pub const YCBCR_2020_BIT_VALVE: Self = Self(16);
}

impl std::ops::BitOr for VideoEncodeRgbModelConversionFlagBitsVALVE {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoEncodeRgbModelConversionFlagBitsVALVE {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoEncodeRgbModelConversionFlagBitsVALVE {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoEncodeRgbModelConversionFlagBitsVALVE {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoEncodeRgbRangeCompressionFlagBitsVALVE(pub u32);
impl VideoEncodeRgbRangeCompressionFlagBitsVALVE {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const FULL_RANGE_BIT_VALVE: Self = Self(1);
	pub const NARROW_RANGE_BIT_VALVE: Self = Self(2);
}

impl std::ops::BitOr for VideoEncodeRgbRangeCompressionFlagBitsVALVE {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoEncodeRgbRangeCompressionFlagBitsVALVE {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoEncodeRgbRangeCompressionFlagBitsVALVE {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoEncodeRgbRangeCompressionFlagBitsVALVE {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VideoEncodeRgbChromaOffsetFlagBitsVALVE(pub u32);
impl VideoEncodeRgbChromaOffsetFlagBitsVALVE {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const COSITED_EVEN_BIT_VALVE: Self = Self(1);
	pub const MIDPOINT_BIT_VALVE: Self = Self(2);
}

impl std::ops::BitOr for VideoEncodeRgbChromaOffsetFlagBitsVALVE {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for VideoEncodeRgbChromaOffsetFlagBitsVALVE {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for VideoEncodeRgbChromaOffsetFlagBitsVALVE {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for VideoEncodeRgbChromaOffsetFlagBitsVALVE {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DescriptorMappingSourceEXT(pub i32);
impl DescriptorMappingSourceEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const HEAP_WITH_CONSTANT_OFFSET_EXT: Self = Self(0);
	pub const HEAP_WITH_PUSH_INDEX_EXT: Self = Self(1);
	pub const HEAP_WITH_INDIRECT_INDEX_EXT: Self = Self(2);
	pub const HEAP_WITH_INDIRECT_INDEX_ARRAY_EXT: Self = Self(3);
	pub const RESOURCE_HEAP_DATA_EXT: Self = Self(4);
	pub const PUSH_DATA_EXT: Self = Self(5);
	pub const PUSH_ADDRESS_EXT: Self = Self(6);
	pub const INDIRECT_ADDRESS_EXT: Self = Self(7);
	pub const HEAP_WITH_SHADER_RECORD_INDEX_EXT: Self = Self(8);
	pub const SHADER_RECORD_DATA_EXT: Self = Self(9);
	pub const SHADER_RECORD_ADDRESS_EXT: Self = Self(10);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SpirvResourceTypeFlagBitsEXT(pub u32);
impl SpirvResourceTypeFlagBitsEXT {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ALL_EXT: Self = Self(2147483647);
	pub const SAMPLER_BIT_EXT: Self = Self(1);
	pub const SAMPLED_IMAGE_BIT_EXT: Self = Self(2);
	pub const READ_ONLY_IMAGE_BIT_EXT: Self = Self(4);
	pub const READ_WRITE_IMAGE_BIT_EXT: Self = Self(8);
	pub const COMBINED_SAMPLED_IMAGE_BIT_EXT: Self = Self(16);
	pub const UNIFORM_BUFFER_BIT_EXT: Self = Self(32);
	pub const READ_ONLY_STORAGE_BUFFER_BIT_EXT: Self = Self(64);
	pub const READ_WRITE_STORAGE_BUFFER_BIT_EXT: Self = Self(128);
	pub const ACCELERATION_STRUCTURE_BIT_EXT: Self = Self(256);
	pub const TENSOR_BIT_ARM: Self = Self(512);
}

impl std::ops::BitOr for SpirvResourceTypeFlagBitsEXT {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for SpirvResourceTypeFlagBitsEXT {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for SpirvResourceTypeFlagBitsEXT {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for SpirvResourceTypeFlagBitsEXT {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct GpaSqShaderStageFlagBitsAMD(pub u32);
impl GpaSqShaderStageFlagBitsAMD {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const PS_BIT_AMD: Self = Self(1);
	pub const VS_BIT_AMD: Self = Self(2);
	pub const GS_BIT_AMD: Self = Self(4);
	pub const ES_BIT_AMD: Self = Self(8);
	pub const HS_BIT_AMD: Self = Self(16);
	pub const LS_BIT_AMD: Self = Self(32);
	pub const CS_BIT_AMD: Self = Self(64);
}

impl std::ops::BitOr for GpaSqShaderStageFlagBitsAMD {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for GpaSqShaderStageFlagBitsAMD {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for GpaSqShaderStageFlagBitsAMD {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for GpaSqShaderStageFlagBitsAMD {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct GpaPerfBlockAMD(pub i32);
impl GpaPerfBlockAMD {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const CPF_AMD: Self = Self(0);
	pub const IA_AMD: Self = Self(1);
	pub const VGT_AMD: Self = Self(2);
	pub const PA_AMD: Self = Self(3);
	pub const SC_AMD: Self = Self(4);
	pub const SPI_AMD: Self = Self(5);
	pub const SQ_AMD: Self = Self(6);
	pub const SX_AMD: Self = Self(7);
	pub const TA_AMD: Self = Self(8);
	pub const TD_AMD: Self = Self(9);
	pub const TCP_AMD: Self = Self(10);
	pub const TCC_AMD: Self = Self(11);
	pub const TCA_AMD: Self = Self(12);
	pub const DB_AMD: Self = Self(13);
	pub const CB_AMD: Self = Self(14);
	pub const GDS_AMD: Self = Self(15);
	pub const SRBM_AMD: Self = Self(16);
	pub const GRBM_AMD: Self = Self(17);
	pub const GRBM_SE_AMD: Self = Self(18);
	pub const RLC_AMD: Self = Self(19);
	pub const DMA_AMD: Self = Self(20);
	pub const MC_AMD: Self = Self(21);
	pub const CPG_AMD: Self = Self(22);
	pub const CPC_AMD: Self = Self(23);
	pub const WD_AMD: Self = Self(24);
	pub const TCS_AMD: Self = Self(25);
	pub const ATC_AMD: Self = Self(26);
	pub const ATC_L2_AMD: Self = Self(27);
	pub const MC_VM_L2_AMD: Self = Self(28);
	pub const EA_AMD: Self = Self(29);
	pub const RPB_AMD: Self = Self(30);
	pub const RMI_AMD: Self = Self(31);
	pub const UMCCH_AMD: Self = Self(32);
	pub const GE_AMD: Self = Self(33);
	pub const GL1A_AMD: Self = Self(34);
	pub const GL1C_AMD: Self = Self(35);
	pub const GL1CG_AMD: Self = Self(36);
	pub const GL2A_AMD: Self = Self(37);
	pub const GL2C_AMD: Self = Self(38);
	pub const CHA_AMD: Self = Self(39);
	pub const CHC_AMD: Self = Self(40);
	pub const CHCG_AMD: Self = Self(41);
	pub const GUS_AMD: Self = Self(42);
	pub const GCR_AMD: Self = Self(43);
	pub const PH_AMD: Self = Self(44);
	pub const UTCL1_AMD: Self = Self(45);
	pub const GE1_AMD: Self = Self(33);
	pub const GE_DIST_AMD: Self = Self(46);
	pub const GE_SE_AMD: Self = Self(47);
	pub const DF_MALL_AMD: Self = Self(48);
	pub const SQ_WGP_AMD: Self = Self(49);
	pub const PC_AMD: Self = Self(50);
	pub const GL1XA_AMD: Self = Self(51);
	pub const GL1XC_AMD: Self = Self(52);
	pub const WGS_AMD: Self = Self(53);
	pub const EACPWD_AMD: Self = Self(54);
	pub const EASE_AMD: Self = Self(55);
	pub const RLCUSER_AMD: Self = Self(56);
	pub const RLCLOCAL_AMD: Self = Self(56);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct GpaSampleTypeAMD(pub i32);
impl GpaSampleTypeAMD {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const CUMULATIVE_AMD: Self = Self(0);
	pub const TRACE_AMD: Self = Self(1);
	pub const TIMING_AMD: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct GpaDeviceClockModeAMD(pub i32);
impl GpaDeviceClockModeAMD {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEFAULT_AMD: Self = Self(0);
	pub const QUERY_AMD: Self = Self(1);
	pub const PROFILING_AMD: Self = Self(2);
	pub const MIN_MEMORY_AMD: Self = Self(3);
	pub const MIN_ENGINE_AMD: Self = Self(4);
	pub const PEAK_AMD: Self = Self(5);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AddressCommandFlagBitsKHR(pub u32);
impl AddressCommandFlagBitsKHR {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const PROTECTED_BIT_KHR: Self = Self(1);
	pub const FULLY_BOUND_BIT_KHR: Self = Self(2);
	pub const STORAGE_BUFFER_USAGE_BIT_KHR: Self = Self(4);
	pub const UNKNOWN_STORAGE_BUFFER_USAGE_BIT_KHR: Self = Self(8);
	pub const TRANSFORM_FEEDBACK_BUFFER_USAGE_BIT_KHR: Self = Self(16);
	pub const UNKNOWN_TRANSFORM_FEEDBACK_BUFFER_USAGE_BIT_KHR: Self = Self(32);
}

impl std::ops::BitOr for AddressCommandFlagBitsKHR {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for AddressCommandFlagBitsKHR {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for AddressCommandFlagBitsKHR {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for AddressCommandFlagBitsKHR {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DataGraphTOSAQualityFlagBitsARM(pub u32);
impl DataGraphTOSAQualityFlagBitsARM {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DATA_GRAPH_TOSA_QUALITY_ACCELERATED_ARM: Self = Self(1);
	pub const DATA_GRAPH_TOSA_QUALITY_CONFORMANT_ARM: Self = Self(2);
	pub const DATA_GRAPH_TOSA_QUALITY_EXPERIMENTAL_ARM: Self = Self(4);
	pub const DATA_GRAPH_TOSA_QUALITY_DEPRECATED_ARM: Self = Self(8);
}

impl std::ops::BitOr for DataGraphTOSAQualityFlagBitsARM {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for DataGraphTOSAQualityFlagBitsARM {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for DataGraphTOSAQualityFlagBitsARM {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for DataGraphTOSAQualityFlagBitsARM {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DataGraphTOSALevelARM(pub i32);
impl DataGraphTOSALevelARM {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DATA_GRAPH_TOSA_LEVEL_NONE_ARM: Self = Self(0);
	pub const DATA_GRAPH_TOSA_LEVEL_8K_ARM: Self = Self(1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DataGraphOpticalFlowGridSizeFlagBitsARM(pub u32);
impl DataGraphOpticalFlowGridSizeFlagBitsARM {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const UNKNOWN_ARM: Self = Self(0);
	pub const _1X1_BIT_ARM: Self = Self(1);
	pub const _2X2_BIT_ARM: Self = Self(2);
	pub const _4X4_BIT_ARM: Self = Self(4);
	pub const _8X8_BIT_ARM: Self = Self(8);
}

impl std::ops::BitOr for DataGraphOpticalFlowGridSizeFlagBitsARM {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for DataGraphOpticalFlowGridSizeFlagBitsARM {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for DataGraphOpticalFlowGridSizeFlagBitsARM {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for DataGraphOpticalFlowGridSizeFlagBitsARM {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DataGraphOpticalFlowImageUsageFlagBitsARM(pub u32);
impl DataGraphOpticalFlowImageUsageFlagBitsARM {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const UNKNOWN_ARM: Self = Self(0);
	pub const INPUT_BIT_ARM: Self = Self(1);
	pub const OUTPUT_BIT_ARM: Self = Self(2);
	pub const HINT_BIT_ARM: Self = Self(4);
	pub const COST_BIT_ARM: Self = Self(8);
}

impl std::ops::BitOr for DataGraphOpticalFlowImageUsageFlagBitsARM {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for DataGraphOpticalFlowImageUsageFlagBitsARM {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for DataGraphOpticalFlowImageUsageFlagBitsARM {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for DataGraphOpticalFlowImageUsageFlagBitsARM {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DataGraphOpticalFlowPerformanceLevelARM(pub i32);
impl DataGraphOpticalFlowPerformanceLevelARM {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const UNKNOWN_ARM: Self = Self(0);
	pub const SLOW_ARM: Self = Self(1);
	pub const MEDIUM_ARM: Self = Self(2);
	pub const FAST_ARM: Self = Self(3);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DataGraphPipelineNodeConnectionTypeARM(pub i32);
impl DataGraphPipelineNodeConnectionTypeARM {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const OPTICAL_FLOW_INPUT_ARM: Self = Self(1000631000);
	pub const OPTICAL_FLOW_REFERENCE_ARM: Self = Self(1000631001);
	pub const OPTICAL_FLOW_HINT_ARM: Self = Self(1000631002);
	pub const OPTICAL_FLOW_FLOW_VECTOR_ARM: Self = Self(1000631003);
	pub const OPTICAL_FLOW_COST_ARM: Self = Self(1000631004);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DataGraphPipelineNodeTypeARM(pub i32);
impl DataGraphPipelineNodeTypeARM {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const OPTICAL_FLOW_ARM: Self = Self(1000631000);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DataGraphOpticalFlowCreateFlagBitsARM(pub u32);
impl DataGraphOpticalFlowCreateFlagBitsARM {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ENABLE_HINT_BIT_ARM: Self = Self(1);
	pub const ENABLE_COST_BIT_ARM: Self = Self(2);
	pub const RESERVED_30_BIT_ARM: Self = Self(1073741824);
}

impl std::ops::BitOr for DataGraphOpticalFlowCreateFlagBitsARM {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for DataGraphOpticalFlowCreateFlagBitsARM {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for DataGraphOpticalFlowCreateFlagBitsARM {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for DataGraphOpticalFlowCreateFlagBitsARM {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DataGraphOpticalFlowExecuteFlagBitsARM(pub u32);
impl DataGraphOpticalFlowExecuteFlagBitsARM {
	pub const fn from_raw(value: u32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> u32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DISABLE_TEMPORAL_HINTS_BIT_ARM: Self = Self(1);
	pub const INPUT_UNCHANGED_BIT_ARM: Self = Self(2);
	pub const REFERENCE_UNCHANGED_BIT_ARM: Self = Self(4);
	pub const INPUT_IS_PREVIOUS_REFERENCE_BIT_ARM: Self = Self(8);
	pub const REFERENCE_IS_PREVIOUS_INPUT_BIT_ARM: Self = Self(16);
}

impl std::ops::BitOr for DataGraphOpticalFlowExecuteFlagBitsARM {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self {
		Self(self.0 | rhs.0)
	}
}

impl std::ops::BitAnd for DataGraphOpticalFlowExecuteFlagBitsARM {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitXor for DataGraphOpticalFlowExecuteFlagBitsARM {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self {
		Self(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for DataGraphOpticalFlowExecuteFlagBitsARM {
	type Output = Self;
	fn not(self) -> Self {
		Self(!self.0)
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct NeuralAcceleratorStatisticsModeARM(pub i32);
impl NeuralAcceleratorStatisticsModeARM {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DISABLED_ARM: Self = Self(0);
	pub const STATISTICS0_ARM: Self = Self(1);
	pub const STATISTICS1_ARM: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ImageTilingControlEXT(pub i32);
impl ImageTilingControlEXT {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEFAULT_EXT: Self = Self(0);
	pub const MIN_SIZE_EXT: Self = Self(1);
	pub const MAX_PERFORMANCE_EXT: Self = Self(2);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoH264ChromaFormatIdc(pub i32);
impl StdVideoH264ChromaFormatIdc {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const MONOCHROME: Self = Self(0);
	pub const _420: Self = Self(1);
	pub const _422: Self = Self(2);
	pub const _444: Self = Self(3);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoH264ProfileIdc(pub i32);
impl StdVideoH264ProfileIdc {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const BASELINE: Self = Self(66);
	pub const MAIN: Self = Self(77);
	pub const HIGH: Self = Self(100);
	pub const HIGH_10: Self = Self(110);
	pub const HIGH_422: Self = Self(122);
	pub const HIGH_444_PREDICTIVE: Self = Self(244);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoH264LevelIdc(pub i32);
impl StdVideoH264LevelIdc {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const _1_0: Self = Self(0);
	pub const _1_1: Self = Self(1);
	pub const _1_2: Self = Self(2);
	pub const _1_3: Self = Self(3);
	pub const _2_0: Self = Self(4);
	pub const _2_1: Self = Self(5);
	pub const _2_2: Self = Self(6);
	pub const _3_0: Self = Self(7);
	pub const _3_1: Self = Self(8);
	pub const _3_2: Self = Self(9);
	pub const _4_0: Self = Self(10);
	pub const _4_1: Self = Self(11);
	pub const _4_2: Self = Self(12);
	pub const _5_0: Self = Self(13);
	pub const _5_1: Self = Self(14);
	pub const _5_2: Self = Self(15);
	pub const _6_0: Self = Self(16);
	pub const _6_1: Self = Self(17);
	pub const _6_2: Self = Self(18);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoH264PocType(pub i32);
impl StdVideoH264PocType {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const _0: Self = Self(0);
	pub const _1: Self = Self(1);
	pub const _2: Self = Self(2);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoH264AspectRatioIdc(pub i32);
impl StdVideoH264AspectRatioIdc {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const UNSPECIFIED: Self = Self(0);
	pub const SQUARE: Self = Self(1);
	pub const _12_11: Self = Self(2);
	pub const _10_11: Self = Self(3);
	pub const _16_11: Self = Self(4);
	pub const _40_33: Self = Self(5);
	pub const _24_11: Self = Self(6);
	pub const _20_11: Self = Self(7);
	pub const _32_11: Self = Self(8);
	pub const _80_33: Self = Self(9);
	pub const _18_11: Self = Self(10);
	pub const _15_11: Self = Self(11);
	pub const _64_33: Self = Self(12);
	pub const _160_99: Self = Self(13);
	pub const _4_3: Self = Self(14);
	pub const _3_2: Self = Self(15);
	pub const _2_1: Self = Self(16);
	pub const EXTENDED_SAR: Self = Self(255);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoH264WeightedBipredIdc(pub i32);
impl StdVideoH264WeightedBipredIdc {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DEFAULT: Self = Self(0);
	pub const EXPLICIT: Self = Self(1);
	pub const IMPLICIT: Self = Self(2);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoH264ModificationOfPicNumsIdc(pub i32);
impl StdVideoH264ModificationOfPicNumsIdc {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const SHORT_TERM_SUBTRACT: Self = Self(0);
	pub const SHORT_TERM_ADD: Self = Self(1);
	pub const LONG_TERM: Self = Self(2);
	pub const END: Self = Self(3);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoH264MemMgmtControlOp(pub i32);
impl StdVideoH264MemMgmtControlOp {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const END: Self = Self(0);
	pub const UNMARK_SHORT_TERM: Self = Self(1);
	pub const UNMARK_LONG_TERM: Self = Self(2);
	pub const MARK_LONG_TERM: Self = Self(3);
	pub const SET_MAX_LONG_TERM_INDEX: Self = Self(4);
	pub const UNMARK_ALL: Self = Self(5);
	pub const MARK_CURRENT_AS_LONG_TERM: Self = Self(6);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoH264CabacInitIdc(pub i32);
impl StdVideoH264CabacInitIdc {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const _0: Self = Self(0);
	pub const _1: Self = Self(1);
	pub const _2: Self = Self(2);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoH264DisableDeblockingFilterIdc(pub i32);
impl StdVideoH264DisableDeblockingFilterIdc {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const DISABLED: Self = Self(0);
	pub const ENABLED: Self = Self(1);
	pub const PARTIAL: Self = Self(2);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoH264SliceType(pub i32);
impl StdVideoH264SliceType {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const P: Self = Self(0);
	pub const B: Self = Self(1);
	pub const I: Self = Self(2);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoH264PictureType(pub i32);
impl StdVideoH264PictureType {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const P: Self = Self(0);
	pub const B: Self = Self(1);
	pub const I: Self = Self(2);
	pub const IDR: Self = Self(5);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoH264NonVclNaluType(pub i32);
impl StdVideoH264NonVclNaluType {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const SPS: Self = Self(0);
	pub const PPS: Self = Self(1);
	pub const AUD: Self = Self(2);
	pub const PREFIX: Self = Self(3);
	pub const END_OF_SEQUENCE: Self = Self(4);
	pub const END_OF_STREAM: Self = Self(5);
	pub const PRECODED: Self = Self(6);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoDecodeH264FieldOrderCount(pub i32);
impl StdVideoDecodeH264FieldOrderCount {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const TOP: Self = Self(0);
	pub const BOTTOM: Self = Self(1);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoH265ChromaFormatIdc(pub i32);
impl StdVideoH265ChromaFormatIdc {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const MONOCHROME: Self = Self(0);
	pub const _420: Self = Self(1);
	pub const _422: Self = Self(2);
	pub const _444: Self = Self(3);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoH265ProfileIdc(pub i32);
impl StdVideoH265ProfileIdc {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const MAIN: Self = Self(1);
	pub const MAIN_10: Self = Self(2);
	pub const MAIN_STILL_PICTURE: Self = Self(3);
	pub const FORMAT_RANGE_EXTENSIONS: Self = Self(4);
	pub const SCC_EXTENSIONS: Self = Self(9);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoH265LevelIdc(pub i32);
impl StdVideoH265LevelIdc {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const _1_0: Self = Self(0);
	pub const _2_0: Self = Self(1);
	pub const _2_1: Self = Self(2);
	pub const _3_0: Self = Self(3);
	pub const _3_1: Self = Self(4);
	pub const _4_0: Self = Self(5);
	pub const _4_1: Self = Self(6);
	pub const _5_0: Self = Self(7);
	pub const _5_1: Self = Self(8);
	pub const _5_2: Self = Self(9);
	pub const _6_0: Self = Self(10);
	pub const _6_1: Self = Self(11);
	pub const _6_2: Self = Self(12);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoH265SliceType(pub i32);
impl StdVideoH265SliceType {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const B: Self = Self(0);
	pub const P: Self = Self(1);
	pub const I: Self = Self(2);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoH265PictureType(pub i32);
impl StdVideoH265PictureType {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const P: Self = Self(0);
	pub const B: Self = Self(1);
	pub const I: Self = Self(2);
	pub const IDR: Self = Self(3);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoH265AspectRatioIdc(pub i32);
impl StdVideoH265AspectRatioIdc {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const UNSPECIFIED: Self = Self(0);
	pub const SQUARE: Self = Self(1);
	pub const _12_11: Self = Self(2);
	pub const _10_11: Self = Self(3);
	pub const _16_11: Self = Self(4);
	pub const _40_33: Self = Self(5);
	pub const _24_11: Self = Self(6);
	pub const _20_11: Self = Self(7);
	pub const _32_11: Self = Self(8);
	pub const _80_33: Self = Self(9);
	pub const _18_11: Self = Self(10);
	pub const _15_11: Self = Self(11);
	pub const _64_33: Self = Self(12);
	pub const _160_99: Self = Self(13);
	pub const _4_3: Self = Self(14);
	pub const _3_2: Self = Self(15);
	pub const _2_1: Self = Self(16);
	pub const EXTENDED_SAR: Self = Self(255);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoVP9Profile(pub i32);
impl StdVideoVP9Profile {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const _0: Self = Self(0);
	pub const _1: Self = Self(1);
	pub const _2: Self = Self(2);
	pub const _3: Self = Self(3);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoVP9Level(pub i32);
impl StdVideoVP9Level {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const _1_0: Self = Self(0);
	pub const _1_1: Self = Self(1);
	pub const _2_0: Self = Self(2);
	pub const _2_1: Self = Self(3);
	pub const _3_0: Self = Self(4);
	pub const _3_1: Self = Self(5);
	pub const _4_0: Self = Self(6);
	pub const _4_1: Self = Self(7);
	pub const _5_0: Self = Self(8);
	pub const _5_1: Self = Self(9);
	pub const _5_2: Self = Self(10);
	pub const _6_0: Self = Self(11);
	pub const _6_1: Self = Self(12);
	pub const _6_2: Self = Self(13);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoVP9FrameType(pub i32);
impl StdVideoVP9FrameType {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const KEY: Self = Self(0);
	pub const NON_KEY: Self = Self(1);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoVP9ReferenceName(pub i32);
impl StdVideoVP9ReferenceName {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const INTRA_FRAME: Self = Self(0);
	pub const LAST_FRAME: Self = Self(1);
	pub const GOLDEN_FRAME: Self = Self(2);
	pub const ALTREF_FRAME: Self = Self(3);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoVP9InterpolationFilter(pub i32);
impl StdVideoVP9InterpolationFilter {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const EIGHTTAP: Self = Self(0);
	pub const EIGHTTAP_SMOOTH: Self = Self(1);
	pub const EIGHTTAP_SHARP: Self = Self(2);
	pub const BILINEAR: Self = Self(3);
	pub const SWITCHABLE: Self = Self(4);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoVP9ColorSpace(pub i32);
impl StdVideoVP9ColorSpace {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const UNKNOWN: Self = Self(0);
	pub const BT_601: Self = Self(1);
	pub const BT_709: Self = Self(2);
	pub const SMPTE_170: Self = Self(3);
	pub const SMPTE_240: Self = Self(4);
	pub const BT_2020: Self = Self(5);
	pub const RESERVED: Self = Self(6);
	pub const RGB: Self = Self(7);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoAV1Profile(pub i32);
impl StdVideoAV1Profile {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const MAIN: Self = Self(0);
	pub const HIGH: Self = Self(1);
	pub const PROFESSIONAL: Self = Self(2);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoAV1Level(pub i32);
impl StdVideoAV1Level {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const _2_0: Self = Self(0);
	pub const _2_1: Self = Self(1);
	pub const _2_2: Self = Self(2);
	pub const _2_3: Self = Self(3);
	pub const _3_0: Self = Self(4);
	pub const _3_1: Self = Self(5);
	pub const _3_2: Self = Self(6);
	pub const _3_3: Self = Self(7);
	pub const _4_0: Self = Self(8);
	pub const _4_1: Self = Self(9);
	pub const _4_2: Self = Self(10);
	pub const _4_3: Self = Self(11);
	pub const _5_0: Self = Self(12);
	pub const _5_1: Self = Self(13);
	pub const _5_2: Self = Self(14);
	pub const _5_3: Self = Self(15);
	pub const _6_0: Self = Self(16);
	pub const _6_1: Self = Self(17);
	pub const _6_2: Self = Self(18);
	pub const _6_3: Self = Self(19);
	pub const _7_0: Self = Self(20);
	pub const _7_1: Self = Self(21);
	pub const _7_2: Self = Self(22);
	pub const _7_3: Self = Self(23);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoAV1FrameType(pub i32);
impl StdVideoAV1FrameType {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const KEY: Self = Self(0);
	pub const INTER: Self = Self(1);
	pub const INTRA_ONLY: Self = Self(2);
	pub const SWITCH: Self = Self(3);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoAV1ReferenceName(pub i32);
impl StdVideoAV1ReferenceName {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const INTRA_FRAME: Self = Self(0);
	pub const LAST_FRAME: Self = Self(1);
	pub const LAST2_FRAME: Self = Self(2);
	pub const LAST3_FRAME: Self = Self(3);
	pub const GOLDEN_FRAME: Self = Self(4);
	pub const BWDREF_FRAME: Self = Self(5);
	pub const ALTREF2_FRAME: Self = Self(6);
	pub const ALTREF_FRAME: Self = Self(7);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoAV1InterpolationFilter(pub i32);
impl StdVideoAV1InterpolationFilter {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const EIGHTTAP: Self = Self(0);
	pub const EIGHTTAP_SMOOTH: Self = Self(1);
	pub const EIGHTTAP_SHARP: Self = Self(2);
	pub const BILINEAR: Self = Self(3);
	pub const SWITCHABLE: Self = Self(4);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoAV1TxMode(pub i32);
impl StdVideoAV1TxMode {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const ONLY_4X4: Self = Self(0);
	pub const LARGEST: Self = Self(1);
	pub const SELECT: Self = Self(2);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoAV1FrameRestorationType(pub i32);
impl StdVideoAV1FrameRestorationType {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const NONE: Self = Self(0);
	pub const WIENER: Self = Self(1);
	pub const SGRPROJ: Self = Self(2);
	pub const SWITCHABLE: Self = Self(3);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoAV1ColorPrimaries(pub i32);
impl StdVideoAV1ColorPrimaries {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const BT_709: Self = Self(1);
	pub const UNSPECIFIED: Self = Self(2);
	pub const BT_UNSPECIFIED: Self = Self(2);
	pub const BT_470_M: Self = Self(4);
	pub const BT_470_B_G: Self = Self(5);
	pub const BT_601: Self = Self(6);
	pub const SMPTE_240: Self = Self(7);
	pub const GENERIC_FILM: Self = Self(8);
	pub const BT_2020: Self = Self(9);
	pub const XYZ: Self = Self(10);
	pub const SMPTE_431: Self = Self(11);
	pub const SMPTE_432: Self = Self(12);
	pub const EBU_3213: Self = Self(22);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoAV1TransferCharacteristics(pub i32);
impl StdVideoAV1TransferCharacteristics {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const RESERVED_0: Self = Self(0);
	pub const BT_709: Self = Self(1);
	pub const UNSPECIFIED: Self = Self(2);
	pub const RESERVED_3: Self = Self(3);
	pub const BT_470_M: Self = Self(4);
	pub const BT_470_B_G: Self = Self(5);
	pub const BT_601: Self = Self(6);
	pub const SMPTE_240: Self = Self(7);
	pub const LINEAR: Self = Self(8);
	pub const LOG_100: Self = Self(9);
	pub const LOG_100_SQRT10: Self = Self(10);
	pub const IEC_61966: Self = Self(11);
	pub const BT_1361: Self = Self(12);
	pub const SRGB: Self = Self(13);
	pub const BT_2020_10_BIT: Self = Self(14);
	pub const BT_2020_12_BIT: Self = Self(15);
	pub const SMPTE_2084: Self = Self(16);
	pub const SMPTE_428: Self = Self(17);
	pub const HLG: Self = Self(18);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoAV1MatrixCoefficients(pub i32);
impl StdVideoAV1MatrixCoefficients {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const IDENTITY: Self = Self(0);
	pub const BT_709: Self = Self(1);
	pub const UNSPECIFIED: Self = Self(2);
	pub const RESERVED_3: Self = Self(3);
	pub const FCC: Self = Self(4);
	pub const BT_470_B_G: Self = Self(5);
	pub const BT_601: Self = Self(6);
	pub const SMPTE_240: Self = Self(7);
	pub const SMPTE_YCGCO: Self = Self(8);
	pub const BT_2020_NCL: Self = Self(9);
	pub const BT_2020_CL: Self = Self(10);
	pub const SMPTE_2085: Self = Self(11);
	pub const CHROMAT_NCL: Self = Self(12);
	pub const CHROMAT_CL: Self = Self(13);
	pub const ICTCP: Self = Self(14);
	pub const INVALID: Self = Self(2147483647);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdVideoAV1ChromaSamplePosition(pub i32);
impl StdVideoAV1ChromaSamplePosition {
	pub const fn from_raw(value: i32) -> Self {
		Self(value)
	}

	pub const fn as_raw(self) -> i32 {
		self.0
	}

	pub const fn empty() -> Self {
		Self(0)
	}

	pub const UNKNOWN: Self = Self(0);
	pub const VERTICAL: Self = Self(1);
	pub const COLOCATED: Self = Self(2);
	pub const RESERVED: Self = Self(3);
	pub const INVALID: Self = Self(2147483647);
}

impl Result {
	pub const fn is_success(self) -> bool {
		self.0 >= 0
	}

	pub const fn is_err(self) -> bool {
		self.0 < 0
	}
}
