#!/usr/bin/env node
// Exercise the pinned rustdoc search engine and its emitted index, without a browser.
const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const vm = require("node:vm");

const doc_root = path.resolve(process.argv[2] || "target/doc");
assert(process.argv.length <= 3, "usage: node tool/verify_docs.cjs [rustdoc-directory]");
const index_html = fs.readFileSync(path.join(doc_root, "vk/index.html"), "utf8");

function script_path(attribute) {
	const name = index_html.match(new RegExp(`${attribute}="([a-z0-9-]+\\.js)"`))?.[1];
	assert(name, `missing rustdoc script attribute: ${attribute}`);
	return path.join(doc_root, "static.files", name);
}

function evaluate(file) {
	// Only execute JavaScript emitted by the local, pinned rustdoc build.
	vm.runInThisContext(fs.readFileSync(file, "utf8"), { filename: file });
}

function check_reference(file, target) {
	const html = fs.readFileSync(path.join(doc_root, "vk", file), "utf8");
	assert(html.includes(`href="${target}"`), `${file} is missing ${target}`);
}

async function main() {
	const { Stringdex, RoaringBitmap } = require(script_path("data-stringdex-js"));
	const { initSearch } = require(script_path("data-search-js"));
	// Browser rustdoc normally supplies this checked identity helper globally.
	globalThis.nonnull = value => {
		assert(value !== null && value !== undefined, "unexpected null rustdoc entry");
		return value;
	};
	const { docSearch, DocSearch } = await initSearch(Stringdex, RoaringBitmap, {
		loadRoot(callbacks) {
			Object.assign(globalThis, callbacks);
			evaluate(path.join(doc_root, "search.index/root.js"));
		},
		loadTreeByHash(hash) {
			evaluate(path.join(doc_root, "search.index", `${hash}.js`));
		},
		loadDataByNameAndHash(name, hash) {
			evaluate(path.join(doc_root, "search.index", name, `${hash}.js`));
		},
	});
	const searches = [
		["VkDevice", "struct.DeviceHandle.html"],
		["VkInstance", "struct.InstanceHandle.html"],
		["VkApplicationInfo", "struct.ApplicationInfo.html"],
		["vkCreateInstance", "fn.create_instance.html"],
		["vkCmdDispatch", "fn.cmd_dispatch.html"],
		["vkCmdSetDispatchParametersARM", "fn.cmd_set_dispatch_parameters_arm.html"],
		["VK_SUCCESS", "struct.Result.html#associatedconstant.SUCCESS"],
		["pApplicationInfo", "struct.InstanceCreateInfo.html#structfield.p_application_info"],
		// This common member has more hits than rustdoc's result limit. Require
		// a native-spelling match without requiring one particular parent record.
		["sType", "#structfield.s_type"],
	];
	for (const [query, expected] of searches) {
		const result = await docSearch.execQuery(DocSearch.parseQuery(query), "vk", "vk");
		let found = false;
		for await (const item of result.others) {
			// CamelCase member queries can also match snake_case through rustdoc's
			// normalizer. Native Vk/VK names must exercise an actual alias entry.
			if (item.href.startsWith("../vk/") && item.href.endsWith(expected)
				&& (item.is_alias || /^[ps][A-Z]/.test(query))) {
				found = true;
			}
		}
		assert(found, `native search failed: ${query} -> ${expected}`);
		console.log(`${query} -> vk::${expected}`);
	}
	const refpages = "https://docs.vulkan.org/refpages/latest/refpages/source";
	for (const [file, native] of [
		["struct.DeviceHandle.html", "VkDevice"],
		["struct.InstanceHandle.html", "VkInstance"],
		["struct.ApplicationInfo.html", "VkApplicationInfo"],
		["struct.Result.html", "VkResult"],
		["fn.create_instance.html", "vkCreateInstance"],
		["struct.Instance.html", "vkCreateInstance"],
		["fn.cmd_bind_pipeline.html", "vkCmdBindPipeline"],
		["type.PFN_vkCreateDevice.html", "vkCreateDevice"],
		["type.PFN_vkAllocationFunction.html", "PFN_vkAllocationFunction"],
	]) {
		check_reference(file, `${refpages}/${native}.html`);
	}
	const provenance = fs.readFileSync(path.join(__dirname, "../registry/SOURCES.md"), "utf8");
	const revision = provenance.match(/\| Commit \| `([a-f0-9]{40})` \|/)[1];
	check_reference("struct.StdVideoDecodeVP9PictureInfo.html",
		`https://github.com/KhronosGroup/Vulkan-Headers/blob/${revision}/registry/video.xml`);
	check_reference("constant.WHOLE_SIZE.html",
		`https://github.com/KhronosGroup/Vulkan-Headers/blob/${revision}/registry/vk.xml`);
	for (const heading of ["Two calling styles", "Wrappers and raw handles", "Native Vulkan, Rust spelling"]) {
		assert(index_html.includes(heading), `missing crate introduction: ${heading}`);
	}
	console.log("Rendered Khronos links, flat public paths and crate introduction verified.");
}

main().catch(error => {
	console.error(error.message);
	process.exitCode = 1;
});
