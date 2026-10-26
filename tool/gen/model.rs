#![allow(dead_code)]
//! Intermediate representation for the Vulkan registry.
//!
//! The IR captures all information needed to emit ABI-correct Rust from
//! `vk.xml` and `video.xml`.  It is intentionally free of codegen details so
//! that each emit pass can query only what it needs.

// ── Dispatch level ────────────────────────────────────────────────────────────

/// Which dispatch table owns a command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DispatchLevel {
	/// Loaded via a null instance (e.g. `vkCreateInstance`).
	Global,
	/// Loaded via `vkGetInstanceProcAddr` with a real instance.
	Instance,
	/// Loaded via `vkGetDeviceProcAddr`.
	Device,
}

// ── Type references ────────────────────────────────────────────────────────────

/// A reference to a Vulkan type, including pointer qualifiers.
#[derive(Debug, Clone)]
pub struct TypeRef {
	/// The base type name as it appears in the registry, e.g. `VkInstance`.
	pub name: String,
	/// Pointee constness for each `*`, from the base outwards.
	pub pointers: Vec<bool>,
	/// Fixed array extents, in C declaration order.
	pub arrays: Vec<String>,
	/// Constness of an array element before C parameter decay.
	pub array_element_const: bool,
}

// ── Parameters ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Parameter {
	pub name: String,
	pub ty: TypeRef,
	/// Optional `len` attribute describing the array length parameter name.
	pub len: Option<String>,
	/// Whether the parameter is optional (pointer may be null).
	pub optional: bool,
	/// `externsync` hint from the registry.
	pub extern_sync: bool,
}

// ── Commands ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CommandDecl {
	/// C name, e.g. `vkCreateInstance`.
	pub c_name: String,
	/// Rust snake_case name, e.g. `create_instance`.
	pub rust_name: String,
	pub return_type: TypeRef,
	/// Registry-declared non-error results; aliases inherit their target's list.
	pub success_codes: Vec<String>,
	pub parameters: Vec<Parameter>,
	pub dispatch: DispatchLevel,
	/// If this command is an alias, the original C name.
	pub alias_of: Option<String>,
	/// Platform / feature guard tokens (e.g. `VK_USE_PLATFORM_WIN32_KHR`).
	pub guards: Vec<String>,
	/// Vulkan feature version string that defined or promoted this command.
	pub feature: Option<String>,
}

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum TypeKind {
	/// `typedef uint32_t VkFlags;` etc.
	Alias { target: String },
	/// Opaque dispatchable handle: `VK_DEFINE_HANDLE`.
	DispatchableHandle,
	/// Non-dispatchable handle: `VK_DEFINE_NON_DISPATCHABLE_HANDLE`.
	NonDispatchableHandle,
	/// C enum (extensible integer).
	Enum,
	/// Bitmask backed by a flags type.
	Bitmask {
		requires: Option<String>,
		base: String,
	},
	/// `#[repr(C)]` struct.
	Struct {
		members: Vec<StructMember>,
		extends: Vec<String>,
		returned_only: bool,
	},
	/// `#[repr(C)]` union.
	Union { members: Vec<StructMember> },
	/// External / platform type (e.g. `HWND`, `xcb_connection_t`).
	ExternalType { requires: Option<String> },
	/// Function pointer typedef.
	FuncPointer {
		return_type: TypeRef,
		parameters: Vec<Parameter>,
	},
	/// Base C type that maps 1:1 to a Rust primitive.
	BaseType { ty: TypeRef },
}

#[derive(Debug, Clone)]
pub struct StructMember {
	pub name: String,
	pub ty: TypeRef,
	pub values: Option<String>, // e.g. sType default constant
	pub len: Option<String>,
	pub optional: bool,
	/// Machine-readable alternative to a registry math length expression.
	pub alt_len: Option<String>,
	pub selector: Option<String>, // union discriminant
	pub selection: Option<String>,
	pub no_auto_validity: bool,
	/// C bitfield width (e.g. `3` for `: 3`), if present.
	pub bitfield: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct TypeDecl {
	/// Handle ancestry from the registry, used for dispatch classification.
	pub parent: Option<String>,
	/// C name as in the registry.
	pub c_name: String,
	pub kind: TypeKind,
	/// Platform guard tokens.
	pub guards: Vec<String>,
	/// Alias target name if this is a promoted/aliased type.
	pub alias_of: Option<String>,
	/// Originating feature.
	pub feature: Option<String>,
	/// Whether this came from `video.xml`.
	pub from_video: bool,
}

// ── Constants ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ConstantDecl {
	pub name: String,
	pub value: String,
	pub alias_of: Option<String>,
	pub ty: Option<String>,
	pub guards: Vec<String>,
}

// ── Enum values ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct EnumValue {
	pub name: String,
	/// Numeric value as a string (decimal, hex, or negative).
	pub value: String,
	pub alias_of: Option<String>,
	pub guards: Vec<String>,
	pub comment: Option<String>,
	/// Extension number that contributed this value, if any.
	pub extension_number: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct EnumDecl {
	pub c_name: String,
	/// Exact integer storage from registry kind/bitwidth metadata.
	pub backing: String,
	pub values: Vec<EnumValue>,
	pub guards: Vec<String>,
}

// ── Extensions ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ExtensionDecl {
	pub platform: Option<String>,
	pub provisional: bool,
	pub promoted_to: Option<String>,
	pub depends: Option<String>,
	pub from_video: bool,
	pub name: String,
	pub number: u32,
	pub kind: String, // "instance" | "device"
	pub guards: Vec<String>,
	/// Types and commands added by this extension.
	pub requires_types: Vec<String>,
	pub requires_commands: Vec<String>,
}

// ── Features ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct FeatureDecl {
	pub name: String,   // e.g. "VK_VERSION_1_4"
	pub number: String, // e.g. "1.4"
	pub requires_types: Vec<String>,
	pub requires_commands: Vec<String>,
}

// ── Registry root ─────────────────────────────────────────────────────────────

/// The complete parsed and resolved intermediate representation.
#[derive(Debug, Default, Clone)]
pub struct Registry {
	/// Records/aliases retaining construction borrows (resolved transitively).
	pub borrowed: std::collections::BTreeSet<String>,
	/// Records/aliases retaining exclusive construction borrows.
	pub exclusive: std::collections::BTreeSet<String>,
	/// Command spelling -> alternative registry provider/dependency expressions.
	pub command_requirements: std::collections::BTreeMap<String, Vec<String>>,
	/// Registry requirement alternatives: each inner list is a conjunction of Cargo features.
	pub provisions: std::collections::BTreeMap<String, Vec<Vec<String>>>,
	pub protections: std::collections::BTreeMap<String, String>,
	pub types: Vec<TypeDecl>,
	pub enums: Vec<EnumDecl>,
	pub commands: Vec<CommandDecl>,
	pub constants: Vec<ConstantDecl>,
	pub extensions: Vec<ExtensionDecl>,
	pub features: Vec<FeatureDecl>,
}
