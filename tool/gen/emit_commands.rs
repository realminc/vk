//! Registry-derived native operations and external API contract tests.
//!
//! Reviewed recipes adapt selected command shapes. All other commands preserve
//! native raw-pointer/count semantics; Vulkan validity is always caller-owned.
use crate::{
	emit_pfn::base_type,
	model::{CommandDecl, DispatchLevel, Parameter, Registry, TypeKind},
	names, protect,
};
use anyhow::{Context, Result, ensure};
use std::{collections::BTreeMap, fmt::Write};

#[derive(Clone, Copy)]
enum Recipe {
	Create,
	Forward,
	Enumerate,
	Counted,
	TemplateUpdate,
	Scalar,
	Native,
}

struct Enumeration {
	item: String,
	helper: &'static str,
}

struct Argument {
	name: String,
	ty: String,
	fixture: String,
}

struct Operation {
	owner: &'static str,
	context: &'static str,
	arguments: Vec<Argument>,
	native: Vec<String>,
	checks: String,
	output: Option<String>,
	status: bool,
	enumeration: Option<Enumeration>,
	counted: bool,
	scalar: Option<String>,
}

/// The admission policy owns recipes, not a second copy of registry prototypes.
fn selected() -> Vec<(String, Recipe)> {
	let mut commands = Vec::new();
	for object in [
		"ShaderModule",
		"DescriptorSetLayout",
		"PipelineLayout",
		"DescriptorPool",
		"PipelineCache",
		"Image",
		"ImageView",
		"BufferView",
		"Sampler",
		"Semaphore",
		"Event",
		"QueryPool",
		"Framebuffer",
		"RenderPass",
	] {
		commands.push((format!("vkCreate{object}"), Recipe::Create));
		commands.push((format!("vkDestroy{object}"), Recipe::Forward));
	}
	for name in [
		"vkDestroyPipeline",
		"vkCreateComputePipelines",
		"vkAllocateDescriptorSets",
		"vkFreeDescriptorSets",
		"vkResetDescriptorPool",
		"vkUpdateDescriptorSets",
		"vkCmdBindPipeline",
		"vkCmdBindDescriptorSets",
		"vkCmdPipelineBarrier",
		"vkCmdCopyBuffer",
		"vkCmdCopyImage",
		"vkCmdCopyBufferToImage",
		"vkCmdCopyImageToBuffer",
		"vkGetImageMemoryRequirements",
		"vkBindImageMemory",
		"vkDeviceWaitIdle",
	] {
		commands.push((name.into(), Recipe::Forward));
	}
	for name in [
		"vkCreateGraphicsPipelines",
		"vkGetPhysicalDeviceFormatProperties",
		"vkGetPhysicalDeviceImageFormatProperties",
		"vkGetDeviceMemoryCommitment",
		"vkGetImageSubresourceLayout",
		"vkGetFenceStatus",
		"vkGetEventStatus",
		"vkSetEvent",
		"vkResetEvent",
		"vkGetQueryPoolResults",
		"vkMergePipelineCaches",
		"vkGetRenderAreaGranularity",
		"vkResetCommandPool",
		"vkResetCommandBuffer",
		"vkQueueBindSparse",
		"vkCmdBindIndexBuffer",
		"vkCmdBindVertexBuffers",
		"vkCmdSetViewport",
		"vkCmdSetScissor",
		"vkCmdSetLineWidth",
		"vkCmdSetDepthBias",
		"vkCmdSetBlendConstants",
		"vkCmdSetDepthBounds",
		"vkCmdSetStencilCompareMask",
		"vkCmdSetStencilWriteMask",
		"vkCmdSetStencilReference",
		"vkCmdDraw",
		"vkCmdDrawIndexed",
		"vkCmdDrawIndirect",
		"vkCmdDrawIndexedIndirect",
		"vkCmdDispatchIndirect",
		"vkCmdBlitImage",
		"vkCmdResolveImage",
		"vkCmdUpdateBuffer",
		"vkCmdPushConstants",
		"vkCmdClearColorImage",
		"vkCmdClearDepthStencilImage",
		"vkCmdClearAttachments",
		"vkCmdSetEvent",
		"vkCmdResetEvent",
		"vkCmdWaitEvents",
		"vkCmdBeginQuery",
		"vkCmdEndQuery",
		"vkCmdResetQueryPool",
		"vkCmdWriteTimestamp",
		"vkCmdCopyQueryPoolResults",
		"vkCmdBeginRenderPass",
		"vkCmdNextSubpass",
		"vkCmdEndRenderPass",
		"vkCmdExecuteCommands",
	] {
		commands.push((name.into(), Recipe::Forward));
	}
	for name in [
		"vkEnumerateDeviceLayerProperties",
		"vkGetImageSparseMemoryRequirements",
		"vkGetPhysicalDeviceSparseImageFormatProperties",
		"vkGetPipelineCacheData",
	] {
		commands.push((name.into(), Recipe::Enumerate));
	}
	for name in [
		"vkGetBufferMemoryRequirements2",
		"vkGetImageMemoryRequirements2",
		"vkGetPhysicalDeviceFeatures2",
		"vkGetPhysicalDeviceProperties2",
		"vkGetPhysicalDeviceMemoryProperties2",
		"vkBindBufferMemory2",
		"vkBindImageMemory2",
	] {
		commands.push((name.into(), Recipe::Forward));
		commands.push((format!("{name}KHR"), Recipe::Forward));
	}
	for name in [
		"vkGetSemaphoreCounterValue",
		"vkWaitSemaphores",
		"vkSignalSemaphore",
		"vkQueueSubmit2",
		"vkCmdSetEvent2",
		"vkCmdResetEvent2",
		"vkCmdWaitEvents2",
		"vkCmdPipelineBarrier2",
		"vkCmdWriteTimestamp2",
		"vkCmdCopyBuffer2",
		"vkCmdCopyImage2",
		"vkCmdCopyBufferToImage2",
		"vkCmdCopyImageToBuffer2",
		"vkCmdBlitImage2",
		"vkCmdResolveImage2",
	] {
		commands.push((name.into(), Recipe::Forward));
		commands.push((format!("{name}KHR"), Recipe::Forward));
	}
	for object in ["DescriptorUpdateTemplate", "SamplerYcbcrConversion"] {
		for suffix in ["", "KHR"] {
			commands.push((format!("vkCreate{object}{suffix}"), Recipe::Create));
			commands.push((format!("vkDestroy{object}{suffix}"), Recipe::Forward));
		}
	}
	for name in [
		"vkGetDeviceGroupPeerMemoryFeatures",
		"vkCmdSetDeviceMask",
		"vkGetPhysicalDeviceFormatProperties2",
		"vkGetPhysicalDeviceImageFormatProperties2",
		"vkTrimCommandPool",
		"vkGetPhysicalDeviceExternalBufferProperties",
		"vkGetPhysicalDeviceExternalFenceProperties",
		"vkGetPhysicalDeviceExternalSemaphoreProperties",
		"vkCmdDispatchBase",
		"vkGetDescriptorSetLayoutSupport",
	] {
		commands.push((name.into(), Recipe::Forward));
		commands.push((format!("{name}KHR"), Recipe::Forward));
	}
	commands.push(("vkGetDeviceQueue2".into(), Recipe::Forward));
	for name in [
		"vkEnumeratePhysicalDeviceGroups",
		"vkGetImageSparseMemoryRequirements2",
		"vkGetPhysicalDeviceQueueFamilyProperties2",
		"vkGetPhysicalDeviceSparseImageFormatProperties2",
	] {
		commands.push((name.into(), Recipe::Counted));
		commands.push((format!("{name}KHR"), Recipe::Counted));
	}
	for suffix in ["", "KHR"] {
		commands.push((
			format!("vkUpdateDescriptorSetWithTemplate{suffix}"),
			Recipe::TemplateUpdate,
		));
	}
	for suffix in ["", "EXT"] {
		commands.push((format!("vkResetQueryPool{suffix}"), Recipe::Forward));
	}
	for name in ["vkCmdDrawIndirectCount", "vkCmdDrawIndexedIndirectCount"] {
		for suffix in ["", "KHR", "AMD"] {
			commands.push((format!("{name}{suffix}"), Recipe::Forward));
		}
	}
	for suffix in ["", "KHR"] {
		commands.push((format!("vkCreateRenderPass2{suffix}"), Recipe::Create));
		for name in [
			"vkCmdBeginRenderPass2",
			"vkCmdNextSubpass2",
			"vkCmdEndRenderPass2",
		] {
			commands.push((format!("{name}{suffix}"), Recipe::Forward));
		}
		for name in [
			"vkGetBufferDeviceAddress",
			"vkGetBufferOpaqueCaptureAddress",
			"vkGetDeviceMemoryOpaqueCaptureAddress",
		] {
			commands.push((format!("{name}{suffix}"), Recipe::Scalar));
		}
	}
	commands.push(("vkGetBufferDeviceAddressEXT".into(), Recipe::Scalar));
	commands
}

/// These commands have canonical handwritten lifecycle, resolver or query bodies.
/// Every other registry command must receive a generated operation.
fn handwritten(name: &str) -> bool {
	matches!(
		name,
		"vkAllocateCommandBuffers"
			| "vkAllocateMemory"
			| "vkBeginCommandBuffer"
			| "vkBindBufferMemory"
			| "vkCmdDispatch"
			| "vkCmdFillBuffer"
			| "vkCreateBuffer"
			| "vkCreateCommandPool"
			| "vkCreateDevice"
			| "vkCreateFence"
			| "vkCreateInstance"
			| "vkDestroyBuffer"
			| "vkDestroyCommandPool"
			| "vkDestroyDevice"
			| "vkDestroyFence"
			| "vkDestroyInstance"
			| "vkEndCommandBuffer"
			| "vkEnumerateDeviceExtensionProperties"
			| "vkEnumerateInstanceExtensionProperties"
			| "vkEnumerateInstanceLayerProperties"
			| "vkEnumerateInstanceVersion"
			| "vkEnumeratePhysicalDevices"
			| "vkFlushMappedMemoryRanges"
			| "vkFreeCommandBuffers"
			| "vkFreeMemory"
			| "vkGetBufferMemoryRequirements"
			| "vkGetDeviceProcAddr"
			| "vkGetDeviceQueue"
			| "vkGetInstanceProcAddr"
			| "vkGetPhysicalDeviceFeatures"
			| "vkGetPhysicalDeviceMemoryProperties"
			| "vkGetPhysicalDeviceProperties"
			| "vkGetPhysicalDeviceQueueFamilyProperties"
			| "vkInvalidateMappedMemoryRanges"
			| "vkMapMemory"
			| "vkQueueSubmit"
			| "vkQueueWaitIdle"
			| "vkResetFences"
			| "vkUnmapMemory"
			| "vkWaitForFences"
	)
}

fn abi_type(name: &str) -> String {
	let base = base_type(name);
	if name.starts_with("Vk") {
		format!("vk::{base}")
	} else {
		base
	}
}

fn argument_name(parameter: &Parameter) -> String {
	let name = names::member_name(&parameter.name);
	if parameter.ty.pointers.is_empty() {
		name
	} else {
		name.strip_prefix("p_").unwrap_or(&name).to_owned()
	}
}

fn length_check(out: &mut String, parameter: &str, expected: &str) {
	writeln!(out, "if {parameter}.len() != {expected} {{ return Err(CallError::SliceLengthMismatch {{ parameter: {parameter:?}, expected: {expected}, actual: {parameter}.len() }}); }}").unwrap();
}

fn operation(reg: &Registry, command: &CommandDecl, recipe: Recipe) -> Result<Operation> {
	let (owner, context) = match command.dispatch {
		DispatchLevel::Instance => ("Instance", "instance"),
		DispatchLevel::Device => ("Device", "device"),
		DispatchLevel::Global => anyhow::bail!("global operation recipe not admitted"),
	};
	let mut operation = Operation {
		owner,
		context,
		arguments: Vec::new(),
		native: Vec::new(),
		checks: String::new(),
		output: None,
		status: command.success_codes.iter().any(|c| c != "VK_SUCCESS"),
		enumeration: None,
		counted: matches!(recipe, Recipe::Counted),
		scalar: None,
	};
	if matches!(recipe, Recipe::Native) {
		// Native forwarding deliberately retains raw pointers and explicit counts.
		// No interpretation of len expressions, optionality, chains or outputs occurs.
		ensure!(
			command.return_type.pointers.is_empty()
				&& command.return_type.arrays.is_empty()
				&& matches!(
					command.return_type.name.as_str(),
					"void"
						| "VkResult"
						| "VkBool32"
						| "VkDeviceAddress"
						| "VkDeviceSize"
						| "uint32_t"
						| "uint64_t"
				),
			"unsupported native return declarator"
		);
		ensure!(
			command.return_type.name != "VkResult" || !command.success_codes.is_empty(),
			"missing native success-code policy"
		);
		operation.status = command.return_type.name == "VkResult";
		if !matches!(command.return_type.name.as_str(), "void" | "VkResult") {
			operation.scalar = Some(crate::emit_pfn::qualified_type(&command.return_type));
		}
		for (index, parameter) in command.parameters.iter().enumerate() {
			if index == 0 && matches!(parameter.ty.name.as_str(), "VkInstance" | "VkDevice") {
				ensure!(
					parameter.ty.pointers.is_empty() && parameter.ty.arrays.is_empty(),
					"invalid native dispatch handle"
				);
				operation.native.push(format!("{context}.handle()"));
				continue;
			}
			let name = names::member_name(&parameter.name);
			ensure!(
				!operation.arguments.iter().any(|a| a.name == name) && name != context,
				"native argument name collision: {name}"
			);
			let ty = crate::emit_pfn::qualified_parameter_type(&parameter.ty);
			operation.arguments.push(Argument {
				name: name.clone(),
				fixture: format!("std::mem::zeroed::<{ty}>()"),
				ty,
			});
			operation.native.push(name);
		}
		return Ok(operation);
	}

	ensure!(
		command.return_type.pointers.is_empty()
			&& command.return_type.arrays.is_empty()
			&& if matches!(recipe, Recipe::Scalar) {
				matches!(
					command.return_type.name.as_str(),
					"VkDeviceAddress" | "uint64_t"
				)
			} else {
				matches!(command.return_type.name.as_str(), "void" | "VkResult")
			},
		"unsupported native return type"
	);
	if matches!(recipe, Recipe::Scalar) {
		ensure!(
			command.dispatch == DispatchLevel::Device
				&& command.success_codes.is_empty()
				&& command.parameters.len() == 2,
			"unsupported scalar query signature/status policy"
		);
		let device = &command.parameters[0];
		let info = &command.parameters[1];
		ensure!(
			device.ty.name == "VkDevice"
				&& device.ty.pointers.is_empty()
				&& device.ty.arrays.is_empty()
				&& device.len.is_none()
				&& !device.optional
				&& info.ty.pointers == [true]
				&& info.ty.arrays.is_empty()
				&& info.len.is_none()
				&& !info.optional
				&& reg
					.types
					.iter()
					.any(|ty| ty.c_name == info.ty.name && matches!(ty.kind, TypeKind::Struct { .. })),
			"scalar query requires a device and a required input record"
		);
		operation.scalar = Some(abi_type(&command.return_type.name));
	}
	if command.return_type.name == "VkResult" {
		ensure!(
			!command.success_codes.is_empty(),
			"missing success-code policy"
		);
	}
	if matches!(recipe, Recipe::TemplateUpdate) {
		ensure!(
			command.return_type.name == "void"
				&& command.parameters.len() == 4
				&& command
					.parameters
					.iter()
					.take(3)
					.map(|p| p.ty.name.as_str())
					.eq(["VkDevice", "VkDescriptorSet", "VkDescriptorUpdateTemplate"])
				&& command
					.parameters
					.iter()
					.take(3)
					.all(|p| p.ty.pointers.is_empty() && p.ty.arrays.is_empty() && p.len.is_none())
				&& command.parameters[3].ty.name == "void",
			"unsupported descriptor template update signature"
		);
		let payload = &command.parameters[3];
		ensure!(
			payload.ty.pointers == [true]
				&& payload.ty.arrays.is_empty()
				&& payload.len.is_none()
				&& !payload.optional,
			"unsupported opaque template payload"
		);
	}
	let output = if matches!(recipe, Recipe::Create) {
		ensure!(
			command.return_type.name == "VkResult" && !operation.status,
			"single-object creation must have VK_SUCCESS as its only success result"
		);
		ensure!(
			command.parameters.len() == 4,
			"unsupported create signature"
		);
		let input = &command.parameters[1];
		let allocator = &command.parameters[2];
		ensure!(
			input.ty.pointers == [true] && input.len.is_none() && !input.optional,
			"creation input must be a required record reference"
		);
		ensure!(
			allocator.ty.name == "VkAllocationCallbacks"
				&& allocator.ty.pointers == [true]
				&& allocator.len.is_none()
				&& allocator.optional,
			"creation allocator must be optional Vulkan callbacks"
		);
		let parameter = &command.parameters[3];
		ensure!(
			parameter.ty.pointers == [false] && parameter.len.is_none(),
			"creation output must be a single writable handle"
		);
		ensure!(
			reg.types.iter().any(|ty| ty.c_name == parameter.ty.name
				&& matches!(
					ty.kind,
					TypeKind::NonDispatchableHandle | TypeKind::DispatchableHandle
				)),
			"creation output is not an admitted handle"
		);
		operation.output = Some(abi_type(&parameter.ty.name));
		Some(parameter.name.as_str())
	} else {
		None
	};
	let enumeration_parameters = if matches!(recipe, Recipe::Enumerate | Recipe::Counted) {
		ensure!(
			command.parameters.len() >= 3,
			"missing enumeration parameters"
		);
		let count = &command.parameters[command.parameters.len() - 2];
		let data = &command.parameters[command.parameters.len() - 1];
		ensure!(
			count.ty.pointers == [false]
				&& count.ty.arrays.is_empty()
				&& count.len.is_none()
				&& !count.optional
				&& data.ty.pointers == [false]
				&& data.ty.arrays.is_empty()
				&& data.optional
				&& data.len.as_deref() == Some(count.name.as_str()),
			"unsupported enumeration count/data protocol"
		);
		if operation.counted {
			ensure!(
				count.ty.name == "uint32_t",
				"counted output count must be u32"
			);
			ensure!(
				command.return_type.name == "void"
					|| command.success_codes == ["VK_SUCCESS", "VK_INCOMPLETE"],
				"unsupported counted output status policy"
			);
			let record = reg
				.types
				.iter()
				.find(|ty| ty.c_name == data.ty.name)
				.context("unknown counted output record")?;
			let TypeKind::Struct { members, .. } = &record.kind else {
				anyhow::bail!("counted output must be a record");
			};
			ensure!(
				members.first().is_some_and(|m| m.name == "sType"
					&& m.ty.name == "VkStructureType"
					&& m.ty.pointers.is_empty()
					&& m.ty.arrays.is_empty())
					&& members.get(1).is_some_and(|m| m.name == "pNext"
						&& m.ty.name == "void"
						&& m.ty.pointers == [false]
						&& m.ty.arrays.is_empty()),
				"counted output requires initialized sType/pNext headers"
			);
		} else {
			let helper = match (
				count.ty.name.as_str(),
				data.ty.name.as_str(),
				command.return_type.name.as_str(),
			) {
				("uint32_t", _, "VkResult") => "enumerate",
				("uint32_t", _, "void") => "enumerate_void",
				("size_t", "void", "VkResult") => "enumerate_bytes",
				_ => anyhow::bail!("unsupported enumeration count/return type"),
			};
			if helper == "enumerate" || helper == "enumerate_bytes" {
				ensure!(
					command
						.success_codes
						.iter()
						.all(|code| matches!(code.as_str(), "VK_SUCCESS" | "VK_INCOMPLETE")),
					"unsupported enumeration status policy"
				);
			}
			// These helpers allocate uninitialized output storage. Chained or opaque
			// outputs need their own recipe with caller-initialized headers/storage.
			let item = if helper == "enumerate_bytes" {
				"u8".into()
			} else {
				let ty = reg
					.types
					.iter()
					.find(|ty| ty.c_name == data.ty.name)
					.context("unknown enumeration output record")?;
				let TypeKind::Struct { members, .. } = &ty.kind else {
					anyhow::bail!("enumeration output must be a plain record");
				};
				ensure!(
					!members
						.iter()
						.any(|member| matches!(member.name.as_str(), "sType" | "pNext")),
					"chained enumeration outputs require initialized headers"
				);
				abi_type(&data.ty.name)
			};
			operation.enumeration = Some(Enumeration { item, helper });
		}
		Some((count.name.as_str(), data.name.as_str()))
	} else {
		None
	};
	// Only count parameters referenced by admitted arrays become inferred lengths.
	let mut counts: BTreeMap<&str, Vec<&Parameter>> = BTreeMap::new();
	for parameter in &command.parameters {
		if enumeration_parameters.is_some_and(|(count, _)| parameter.len.as_deref() == Some(count)) {
			continue;
		}
		if let Some(len) = &parameter.len
			&& command.parameters.iter().any(|p| p.name == *len)
		{
			counts.entry(len).or_default().push(parameter);
		}
	}
	for (index, parameter) in command.parameters.iter().enumerate() {
		let name = argument_name(parameter);
		if index == 0 && matches!(parameter.ty.name.as_str(), "VkDevice" | "VkInstance") {
			ensure!(
				parameter.ty.pointers.is_empty(),
				"invalid dispatch parameter"
			);
			operation.native.push(format!("{context}.handle()"));
			continue;
		}
		if Some(parameter.name.as_str()) == output {
			operation.native.push("&mut output".into());
			continue;
		}
		if let Some((count, data)) = enumeration_parameters {
			if parameter.name == count {
				if operation.counted {
					operation.arguments.push(Argument {
						name: name.clone(),
						ty: "&mut u32".into(),
						fixture: "&mut 0".into(),
					});
				}
				operation.native.push(if operation.counted {
					name
				} else {
					"count".into()
				});
				continue;
			}
			if parameter.name == data {
				if operation.counted {
					let count_name = argument_name(&command.parameters[command.parameters.len() - 2]);
					let base = abi_type(&parameter.ty.name);
					operation.arguments.push(Argument {
						name: name.clone(),
						ty: format!("Option<&mut [{base}]>"),
						fixture: "None".into(),
					});
					writeln!(operation.checks, "let capacity = {name}.as_ref().map(|values| values.len());\nif let Some(capacity) = capacity {{ *{count_name} = u32::try_from(capacity).map_err(|_| CallError::CountOverflow(capacity))?; }}\nlet values = {name}.map_or(std::ptr::null_mut(), |values| values.as_mut_ptr());").unwrap();
					operation.native.push("values".into());
					continue;
				}
				operation.native.push(
					if parameter.ty.name == "void" {
						"values.cast()"
					} else {
						"values"
					}
					.into(),
				);
				continue;
			}
		}
		if let Some(arrays) = counts.get(parameter.name.as_str()) {
			ensure!(
				matches!(
					parameter.ty.name.as_str(),
					"uint32_t" | "size_t" | "VkDeviceSize"
				) && parameter.ty.pointers.is_empty()
					&& parameter.ty.arrays.is_empty(),
				"unsupported array count type"
			);
			let first = argument_name(arrays[0]);
			let count_type = abi_type(&parameter.ty.name);
			if parameter.ty.name == "size_t" {
				writeln!(operation.checks, "let {name} = {first}.len();").unwrap();
			} else {
				writeln!(operation.checks,
				"let {name} = {count_type}::try_from({first}.len()).map_err(|_| CallError::CountOverflow({first}.len()))?;").unwrap();
			}
			for other in arrays.iter().skip(1) {
				length_check(
					&mut operation.checks,
					&argument_name(other),
					&format!("{first}.len()"),
				);
			}
			operation.native.push(name);
			continue;
		}
		let (ty, native, fixture) = if !parameter.ty.arrays.is_empty() {
			ensure!(
				parameter.ty.arrays.len() == 1
					&& parameter.ty.pointers.is_empty()
					&& parameter.ty.array_element_const
					&& parameter.len.is_none()
					&& !parameter.optional,
				"unsupported fixed array declarator"
			);
			let extent: usize = parameter.ty.arrays[0]
				.parse()
				.context("fixed array extent must be numeric")?;
			ensure!(extent > 0, "fixed array extent must be positive");
			let base = abi_type(&parameter.ty.name);
			(
				format!("&[{base}; {extent}]"),
				format!("{name}.as_ptr()"),
				format!("&[Default::default(); {extent}]"),
			)
		} else if parameter.ty.pointers.is_empty() {
			(
				abi_type(&parameter.ty.name),
				name.clone(),
				"Default::default()".into(),
			)
		} else if matches!(recipe, Recipe::TemplateUpdate) && parameter.ty.name == "void" {
			(
				"*const std::ffi::c_void".into(),
				name.clone(),
				"std::ptr::null()".into(),
			)
		} else {
			ensure!(
				parameter.ty.pointers.len() == 1
					&& (parameter.ty.name != "void" || parameter.len.is_some()),
				"nested/unsized void pointer recipe not admitted"
			);
			let base = if parameter.ty.name == "void" {
				"u8".into()
			} else {
				abi_type(&parameter.ty.name)
			};
			let cast = if parameter.ty.name == "void" {
				".cast()"
			} else {
				""
			};
			let constant = parameter.ty.pointers[0];
			if let Some(len) = &parameter.len {
				if !counts.contains_key(len.as_str()) {
					let (record, field) = len
						.split_once("->")
						.context("unsupported array length expression")?;
					let input = command
						.parameters
						.iter()
						.find(|p| p.name == record)
						.context("array length refers to unknown parameter")?;
					ensure!(
						input.ty.pointers == [true] && !input.optional,
						"length record must be a required input reference"
					);
					ensure!(
						!field.is_empty() && field.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'),
						"unsupported array count member"
					);
					let record = reg
						.types
						.iter()
						.find(|ty| ty.c_name == input.ty.name)
						.context("unknown array length record")?;
					let TypeKind::Struct { members, .. } = &record.kind else {
						anyhow::bail!("array length source is not a record");
					};
					ensure!(
						members.iter().any(|member| member.name == field
							&& member.ty.name == "uint32_t"
							&& member.ty.pointers.is_empty()
							&& member.ty.arrays.is_empty()),
						"array length member must be a native u32"
					);
					let expected = format!("{name}_expected");
					writeln!(
						operation.checks,
						"let {expected} = usize::try_from({}.{}).map_err(|_| CallError::CountOverflow(usize::MAX))?;",
						argument_name(input),
						names::member_name(field)
					)
					.unwrap();
					length_check(&mut operation.checks, &name, &expected);
				}
				if constant {
					(
						format!("&[{base}]"),
						format!("{name}.as_ptr(){cast}"),
						"&[]".into(),
					)
				} else {
					(
						format!("&mut [{base}]"),
						format!("{name}.as_mut_ptr(){cast}"),
						"&mut []".into(),
					)
				}
			} else if constant && parameter.optional {
				(
					format!("Option<&{base}>"),
					format!("{name}.map_or(std::ptr::null(), |value| value)"),
					"None".into(),
				)
			} else if constant {
				(
					format!("&{base}"),
					name.clone(),
					format!("&std::mem::zeroed::<{base}>()"),
				)
			} else {
				ensure!(!parameter.optional, "optional output recipe not admitted");
				(
					format!("&mut {base}"),
					name.clone(),
					format!("&mut std::mem::zeroed::<{base}>()"),
				)
			}
		};
		ensure!(
			!operation.arguments.iter().any(|a| a.name == name),
			"Rust argument name collision: {name}"
		);
		operation.arguments.push(Argument { name, ty, fixture });
		operation.native.push(native);
	}
	Ok(operation)
}

/// Mock-only nonzero identities. These never reach a native implementation or
/// get dereferenced; comparisons run inside exact-prototype mock functions.
fn native_fixture(reg: &Registry, parameter: &Parameter, index: usize) -> Result<String> {
	let ty = crate::emit_pfn::qualified_parameter_type(&parameter.ty);
	let seed = 0x1234_0000_u64 + index as u64 * 0x100;
	if !parameter.ty.pointers.is_empty() || !parameter.ty.arrays.is_empty() {
		let pointer = format!("std::ptr::without_provenance_mut::<std::ffi::c_void>({seed})");
		return Ok(if ty == "*mut std::ffi::c_void" {
			pointer
		} else {
			format!("{pointer} as {ty}")
		});
	}
	if let Some(decl) = reg
		.types
		.iter()
		.find(|decl| decl.c_name == parameter.ty.name)
	{
		if let Some(target) = &decl.alias_of {
			let mut target_parameter = parameter.clone();
			target_parameter.ty.name = target.clone();
			return native_fixture(reg, &target_parameter, index);
		}
		match &decl.kind {
			TypeKind::DispatchableHandle
			| TypeKind::NonDispatchableHandle
			| TypeKind::Enum
			| TypeKind::Bitmask { .. } => return Ok(format!("{ty}::from_raw({seed}_u64 as _)")),
			TypeKind::Alias { target } => {
				let mut target_parameter = parameter.clone();
				target_parameter.ty.name = target.clone();
				return native_fixture(reg, &target_parameter, index);
			}
			TypeKind::BaseType { .. } => {}
			TypeKind::ExternalType { .. } => return Ok("Default::default()".into()),
			_ => anyhow::bail!(
				"non-scalar by-value native mock parameter: {}",
				parameter.ty.name
			),
		}
	}
	Ok(format!("{seed}_u64 as {ty}"))
}

fn emit_native_mock_test(
	reg: &Registry,
	command: &CommandDecl,
	operation: &Operation,
	tests: &mut String,
) -> Result<()> {
	let name = &command.c_name;
	let field = names::command_name(name);
	let owner = operation.owner;
	let context = operation.context;
	let cfg = protect::cfg(&command.guards);
	let return_type = crate::emit_pfn::qualified_type(&command.return_type);
	let mut parameters = Vec::new();
	let mut equal = Vec::new();
	let mut inputs = Vec::new();
	for (index, parameter) in command.parameters.iter().enumerate() {
		let local = format!("arg_{index}");
		parameters.push(format!(
			"{local}: {}",
			crate::emit_pfn::qualified_parameter_type(&parameter.ty)
		));
		let expected = if index == 0 && matches!(parameter.ty.name.as_str(), "VkDevice" | "VkInstance")
		{
			if operation.owner == "Device" {
				"vk::DeviceHandle::from_raw(0x2000)".into()
			} else {
				"vk::InstanceHandle::from_raw(0x1000)".into()
			}
		} else {
			let value = native_fixture(reg, parameter, index)?;
			inputs.push(value.clone());
			value
		};
		if !parameter.ty.pointers.is_empty() || !parameter.ty.arrays.is_empty() {
			equal.push(format!("std::ptr::eq({local}, {expected})"));
		} else {
			equal.push(format!("{local} == {expected}"));
		}
	}
	let native_result = match command.return_type.name.as_str() {
		"void" => String::new(),
		"VkResult" => "STATUS.get()".into(),
		"uint64_t" => "0xfedc_ba98_7654_3210_u64".into(),
		_ => format!("0xfedc_ba98_7654_3210_u64 as {return_type}"),
	};
	let value = match command.return_type.name.as_str() {
		"void" => "()".into(),
		"VkResult" => "status".into(),
		_ => native_result.clone(),
	};
	let args = inputs.join(", ");
	let mock_return = if return_type == "()" {
		String::new()
	} else {
		format!(" -> {return_type}")
	};
	let attribute = if parameters.len() > 7 {
		"#[allow(clippy::too_many_arguments)]\n"
	} else {
		""
	};
	writeln!(
		tests,
		"{cfg}#[test]\nfn {field}_native_forwarding() {{\nthread_local! {{ static CALLS: std::cell::Cell<usize> = const {{ std::cell::Cell::new(0) }}; static MATCHED: std::cell::Cell<bool> = const {{ std::cell::Cell::new(true) }}; static STATUS: std::cell::Cell<vk::Result> = const {{ std::cell::Cell::new(vk::Result::SUCCESS) }}; }}\n{attribute}unsafe extern \"system\" fn mock({}){mock_return} {{\nCALLS.set(CALLS.get() + 1);\nMATCHED.set(MATCHED.get() && ({}));\n{native_result}\n}}\n// SAFETY: the exact-ABI mock only compares synthetic tokens and never accesses pointers.\nunsafe {{\nlet typed: vk::PFN_{name} = Some(mock);\nlet erased = std::mem::transmute::<vk::PFN_{name}, vk::PFN_vkVoidFunction>(typed);\nlet fixture = Fixture::new_with_command(c\"{name}\", erased);\nlet resolutions = Fixture::query_count();",
		parameters.join(", "),
		equal.join(" && ")
	)?;
	if command.return_type.name == "VkResult" {
		writeln!(
			tests,
			"for status in [vk::Result::SUCCESS, vk::Result::INCOMPLETE, vk::Result::OPERATION_DEFERRED_KHR, vk::Result::from_raw(i32::MAX), vk::Result::ERROR_DEVICE_LOST, vk::Result::from_raw(i32::MIN)] {{\nSTATUS.set(status);\nlet expected = if status.is_err() {{ Err(CallError::Vulkan(status)) }} else {{ Ok(status) }};\nassert_eq!(vk::{field}(&fixture.{context}, {args}), expected);\nassert_eq!(vk::{owner}::{field}(&fixture.{context}, {args}), expected);\n}}\nassert_eq!(CALLS.get(), 12);"
		)?;
	} else if command.return_type.name == "void" {
		writeln!(
			tests,
			"vk::{field}(&fixture.{context}, {args}).unwrap();\nvk::{owner}::{field}(&fixture.{context}, {args}).unwrap();\nassert_eq!(CALLS.get(), 2);"
		)?;
	} else {
		writeln!(
			tests,
			"assert_eq!(vk::{field}(&fixture.{context}, {args}).unwrap(), {value});\nassert_eq!(vk::{owner}::{field}(&fixture.{context}, {args}).unwrap(), {value});\nassert_eq!(CALLS.get(), 2);"
		)?;
	}
	writeln!(
		tests,
		"assert!(MATCHED.get());\nassert_eq!(Fixture::query_count(), resolutions);\n}}\n}}\n"
	)?;
	Ok(())
}

fn return_type(operation: &Operation) -> String {
	if let Some(enumeration) = &operation.enumeration {
		return format!("Vec<{}>", enumeration.item);
	}
	operation
		.output
		.as_deref()
		.or(operation.scalar.as_deref())
		.unwrap_or(if operation.status { "vk::Result" } else { "()" })
		.to_owned()
}

pub fn emit_commands(reg: &Registry) -> Result<(String, String)> {
	let mut source = String::from(
		"// @generated by vk-gen — admitted native operations.\nuse crate::{self as vk, load::CallError};\nuse super::{Device, Instance};\n\n",
	);
	let mut tests = String::from(
		"// @generated by vk-gen — API signatures and missing-command contracts.\n// These share registry metadata with the emitter; independent ABI/behavior proofs remain separate.\nuse ::vk::{self, CallError};\nuse super::command::Fixture;\n\n",
	);
	let recipes: BTreeMap<_, _> = selected().into_iter().collect();
	for name in recipes.keys() {
		ensure!(
			reg.commands.iter().any(|command| command.c_name == *name),
			"admitted operation missing from registry: {name}"
		);
	}
	for command in &reg.commands {
		if handwritten(&command.c_name) {
			ensure!(
				!recipes.contains_key(&command.c_name),
				"duplicate handwritten/generated operation"
			);
			continue;
		}
		let recipe = recipes
			.get(&command.c_name)
			.copied()
			.unwrap_or(Recipe::Native);
		let name = &command.c_name;
		let operation = operation(reg, command, recipe).with_context(|| format!("operation {name}"))?;
		let cfg = protect::cfg(&command.guards);
		let field = names::command_name(&command.c_name);
		let owner = operation.owner;
		let context = operation.context;
		let result = return_type(&operation);
		let declarations = operation
			.arguments
			.iter()
			.map(|a| format!("{}: {}", a.name, a.ty))
			.collect::<Vec<_>>()
			.join(", ");
		let errors = if matches!(recipe, Recipe::Native) {
			"Reports a missing command or a negative native VkResult; non-error results are preserved."
		} else if operation.scalar.is_some() {
			"Reports a missing command; the native scalar is never converted to a status."
		} else {
			"Reports a missing command, invalid slice lengths/counts or a native error."
		};
		let mut docs = format!(
			"/// Call `{name}` using this context's cached function pointer.\n///\n/// # Errors\n/// {errors}\n///\n/// # Safety\n/// The context and all handles must be live and belong to the same Vulkan\n/// implementation/device. Meet `{name}` valid usage, enabled API/extension/feature,\n/// pointer-chain lifetime, callback compatibility and host/GPU synchronization\n/// requirements. Native resources require explicit destruction after completion.\n"
		);
		if matches!(
			field.as_str(),
			"create_compute_pipelines" | "create_graphics_pipelines"
		) {
			docs.push_str("///\n/// Returns non-error native status codes unchanged, including compile-required.\n/// On native failure, output slots can contain successfully created pipelines;\n/// the caller must inspect and destroy every non-null handle. Slots are not reset\n/// before the call: their previous contents must not own live resources.\n");
		}
		if operation.counted {
			docs.push_str("///\n/// Pass None to query the count, or Some with fully initialized output records\n/// and live, valid output chains. The slice length supplies native capacity;\n/// count receives the native number available/written. Some(empty) is a zero-\n/// capacity fill, not a count query. No allocation, retry, header reset or chain\n/// replacement occurs. Handle INCOMPLETE (result queries) or re-query for growth\n/// (void queries) explicitly. Output contents after native errors follow Vulkan.\n");
		}
		if matches!(recipe, Recipe::TemplateUpdate) {
			docs.push_str("///\n/// Data must address initialized descriptor payloads with the alignment, offsets,\n/// strides and extent declared by descriptor_update_template. No byte size can\n/// be inferred from this pointer; the caller guarantees the entire native read.\n");
		}
		if operation.scalar.is_some() && !matches!(recipe, Recipe::Native) {
			docs.push_str("///\n/// Returns the native 64-bit value unchanged, including zero. This is not a\n/// VkResult status; opaque capture values are tokens, not dereferenceable addresses.\n/// This call neither owns memory nor establishes its lifetime or device accessibility.\n");
		}
		if matches!(recipe, Recipe::Native) {
			docs.push_str("///\n/// Preserves native raw pointers, explicit counts, optional nulls and return values.\n/// The caller supplies correctly aligned live storage, initialized output-chain\n/// headers and every extent implied by native counts, strides or nested records.\n/// No allocation, retry, output initialization, ownership transfer or alias fallback\n/// occurs in this wrapper. Outputs, including partial outputs on native failure,\n/// remain caller-owned and must be inspected/cleaned up according to Vulkan.\n");
		}
		if let Some(enumeration) = &operation.enumeration {
			writeln!(docs, "///\n/// Retries incomplete/growing results with a bounded budget; returns an error\n/// if the query does not stabilize. Allocates the returned vector.").unwrap();
			ensure!(
				enumeration.helper != "enumerate_bytes" || enumeration.item == "u8",
				"invalid byte enumeration"
			);
		}
		if command
			.parameters
			.iter()
			.any(|parameter| parameter.ty.name == "void" && parameter.len.is_some())
			&& operation.enumeration.is_none()
			&& !matches!(recipe, Recipe::Native)
		{
			docs.push_str("///\n/// Byte slices determine native byte counts. The caller must also satisfy the\n/// command's data alignment, format, size-multiple and stride requirements;\n/// a byte slice alone does not establish those properties.\n");
		}
		let attributes = if operation.arguments.len() + 1 > 7 {
			"#[allow(clippy::too_many_arguments)] // Preserve the native operation's parameter groups.\n"
		} else {
			""
		};
		crate::doc::command(&mut docs, name, &field);
		writeln!(source, "{cfg}{docs}{attributes}pub unsafe fn {field}({context}: &{owner}, {declarations}) -> Result<{result}, CallError> {{").unwrap();
		writeln!(
			source,
			"let __vk_function = {context}.fp().{field}.ok_or(CallError::MissingCommand({name:?}))?;"
		)
		.unwrap();
		source.push_str(&operation.checks);
		if let Some(output) = &operation.output {
			writeln!(source, "let mut output = {output}::NULL;").unwrap();
		}
		let assignment = if command.return_type.name == "VkResult" || operation.scalar.is_some() {
			"let result = "
		} else {
			""
		};
		if let Some(enumeration) = &operation.enumeration {
			writeln!(source, "// SAFETY: native enumeration initializes the reported plain records/bytes within capacity; the caller establishes handle validity.\nunsafe {{ crate::load::enumerate::{}(|count, values| __vk_function({})) }}\n}}\n", enumeration.helper, operation.native.join(", ")).unwrap();
		} else {
			writeln!(source, "// SAFETY: the caller establishes the native valid-usage contract; argument validity and storage extents follow the documented operation contract.\n{assignment}unsafe {{ __vk_function({}) }};", operation.native.join(", ")).unwrap();
			if command.return_type.name == "VkResult" {
				let failure = if operation.status {
					"result.is_err()"
				} else {
					"result != vk::Result::SUCCESS"
				};
				writeln!(
					source,
					"if {failure} {{ return Err(CallError::Vulkan(result)); }}"
				)
				.unwrap();
			}
			if operation.counted {
				let count = argument_name(&command.parameters[command.parameters.len() - 2]);
				writeln!(source, "if capacity.is_some_and(|capacity| u64::from(*{count}) > capacity as u64) {{ return Err(CallError::InvalidOutput({name:?})); }}").unwrap();
			}
			let value = if operation.output.is_some() {
				"output"
			} else if operation.status || operation.scalar.is_some() {
				"result"
			} else {
				"()"
			};
			writeln!(source, "Ok({value})\n}}\n").unwrap();
		}
		let forwarded = operation
			.arguments
			.iter()
			.map(|a| a.name.as_str())
			.collect::<Vec<_>>()
			.join(", ");
		writeln!(source, "{cfg}impl {owner} {{\n{docs}{attributes}#[inline]\npub unsafe fn {field}(&self, {declarations}) -> Result<{result}, CallError> {{\n// SAFETY: the caller provides the canonical operation's native guarantees.\nunsafe {{ {field}(self, {forwarded}) }}\n}}\n}}\n").unwrap();
		// Type checks cover root/module/method identity without executing native calls.
		let types = operation
			.arguments
			.iter()
			.map(|a| a.ty.as_str())
			.collect::<Vec<_>>()
			.join(", ");
		let args = operation
			.arguments
			.iter()
			.map(|a| a.fixture.as_str())
			.collect::<Vec<_>>()
			.join(", ");

		writeln!(tests, "{cfg}#[test]\nfn {field}_signature_and_missing_command() {{\ntype Signature = unsafe fn(&vk::{owner}, {types}) -> Result<{result}, CallError>;\nlet _: Signature = vk::{field};\nlet _: Signature = vk::{owner}::{field};\nlet fixture = Fixture::new();\n// SAFETY: the exact-ABI mock context has no native resources; the missing PFN returns before accessing arguments. Zeroed fixtures contain only zero-valid generated ABI values and are not native create/query inputs.\nunsafe {{\nassert!(matches!(vk::{field}(&fixture.{context}, {args}), Err(CallError::MissingCommand({name:?}))));\nassert!(matches!(fixture.{context}.{field}({args}), Err(CallError::MissingCommand({name:?}))));\n}}\n}}\n").unwrap();
		if matches!(recipe, Recipe::Native) {
			emit_native_mock_test(reg, command, &operation, &mut tests)?;
		}
	}
	Ok((source, tests))
}

#[cfg(test)]
#[path = "../../test/rs/tool/gen/test_emit_commands.rs"]
mod tests;
