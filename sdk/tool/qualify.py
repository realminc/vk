#!/usr/bin/env python3
"""Run named real-driver profiles; never count a skipped test as hardware evidence."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import sys

import bench

ROOT = bench.ROOT
CORE = [
    ("load", "entry::headless_instance_create_destroy", "VULKAN_AVAILABLE"),
    ("load", "entry::native_device_dispatch_caps_requested_version_by_selected_physical_device", "VULKAN_AVAILABLE"),
    ("load", "entry::native_vulkan_1_1_feature_chain_and_output_queries", "VULKAN_1_1_AVAILABLE"),
    ("mem", "allocator::compute_shader_storage_buffer_round_trip", "VULKAN_AVAILABLE"),
    ("mem", "allocator::descriptor_template_and_base_dispatch_storage_buffer_round_trip", "VULKAN_1_1_AVAILABLE"),
    ("mem", "allocator::pooled_legacy_storage_buffer_round_trip", "VULKAN_AVAILABLE"),
    ("mem", "allocator::pooled_core_persistent_storage_buffer_round_trip", "VULKAN_1_1_AVAILABLE"),
    ("mem", "allocator::custom_pool_persistent_storage_buffer_round_trip", "VULKAN_1_1_AVAILABLE"),
    ("mem", "allocator::automatic_pool_persistent_storage_buffer_round_trip", "VULKAN_1_1_AVAILABLE"),
    ("mem", "allocator::linear_pool_upper_persistent_storage_buffer_round_trip", "VULKAN_1_1_AVAILABLE"),
    ("mem", "defragmentation::native_buffer_copy_commit_and_readback", "VULKAN_1_1_AVAILABLE"),
    ("mem", "defragmentation::native_image_copy_commit_and_readback", "VULKAN_1_1_AVAILABLE"),
    ("mem", "defragmentation::native_multi_pass_extensive_commit_and_readback", "VULKAN_1_1_AVAILABLE"),
    ("mem", "defragmentation::native_intra_block_balanced_commit_and_readback", "VULKAN_1_1_AVAILABLE"),
    ("mem", "defragmentation::native_intra_block_full_commit_and_readback", "VULKAN_1_1_AVAILABLE"),
    ("mem", "defragmentation::native_alias_group_commit_and_readback", "VULKAN_1_1_AVAILABLE"),
    ("mem", "defragmentation::native_descriptor_rebind_after_commit", "VULKAN_1_1_AVAILABLE"),
    *[("mem", f"image::{name}", "VULKAN_AVAILABLE" if name == "managed_image_transfer_readback" else "VULKAN_1_1_AVAILABLE") for name in
      ("managed_image_transfer_readback", "pooled_image_transfer_readback", "adaptive_image_transfer_readback",
       "custom_pool_image_transfer_readback", "linear_mixed_upper_image_transfer_readback")],
    *[("mem", f"synchronized::{name}", "VULKAN_AVAILABLE") for name in
      ("serialized_native_buffer_mapping_across_host_threads", "parallel_native_pools_mapping_across_host_threads",
       "independent_blocks_bind_and_map_concurrently_inside_one_pool", "default_memory_type_scope_preserves_native_buffer_workload")],
]
OPTIONAL = {
    "vulkan12": ("load", "entry::native_vulkan_1_2_host_query_reset_and_render_pass2_submission", "VULKAN_1_2_AVAILABLE", None),
    "timeline": ("load", "entry::native_timeline_and_metadata_import_round_trip", "VULKAN_TIMELINE_AVAILABLE", "timeline_semaphore"),
    "address": ("mem", "allocator::address_storage_buffer_round_trip", "VULKAN_MEMORY_ADDRESS_AVAILABLE", "buffer_device_address"),
    "opaque-fd": ("mem", "import::opaque_fd_buffer_export_import_binding_mapping_and_accounting", "VULKAN_AVAILABLE", None),
    "dma-buf": ("mem", "import::dma_buf_buffer_export_import_properties_and_payload", "VULKAN_AVAILABLE", None),
    "opaque-fd-image": ("mem", "import::opaque_fd_linear_image_export_import_pixels_and_accounting", "VULKAN_AVAILABLE", None),
}
DIAGNOSTICS = re.compile(r"Validation Error|VUID-|SYNC-HAZARD|UNASSIGNED-|Validation Warning", re.I)
SKIPPED = re.compile(r"\bSKIP\b|unsupported|unqualified", re.I)
LAYER_ACTIVE = 'Insert instance layer "VK_LAYER_KHRONOS_validation"'


def validation_environment(env, output, layer_manifest, mode):
    """Inspect this installed layer's settings vocabulary instead of guessing it."""
    payload = layer_manifest.read_bytes()
    record = json.loads(payload)
    if record["layer"]["name"] != "VK_LAYER_KHRONOS_validation":
        raise ValueError("wrong validation-layer manifest")
    keys = set()

    def collect(value):
        if isinstance(value, dict):
            if "key" in value: keys.add(value["key"])
            for child in value.values(): collect(child)
        elif isinstance(value, list):
            for child in value: collect(child)

    collect(record)
    settings = {"report_flags": "error,warn", "debug_action": "VK_DBG_LAYER_ACTION_LOG_MSG",
                "log_filename": "stdout", "validate_core": "true"}
    if mode == "sync": settings["validate_sync"] = "true"
    if not settings.keys() <= keys:
        raise ValueError(f"installed layer does not describe required settings: {sorted(settings.keys() - keys)}")
    (output / "layer-manifest.json").write_bytes(payload)
    (output / "layer.json").write_text(json.dumps({"sha256": hashlib.sha256(payload).hexdigest(),
        "manifest": str(layer_manifest), "implementation_version": record["layer"].get("implementation_version"),
        "api_version": record["layer"].get("api_version"), "settings": settings}, indent=2) + "\n")
    for key in list(env):
        if key.startswith("VK_LAYER_") or key in ("VK_INSTANCE_LAYERS", "VK_LOADER_LAYERS_DISABLE", "VK_LOADER_LAYERS_ENABLE"):
            del env[key]
    (output / "vk_layer_settings.txt").write_text("".join(f"khronos_validation.{key} = {value}\n" for key, value in settings.items()))
    env.update(VK_INSTANCE_LAYERS="VK_LAYER_KHRONOS_validation", VK_LAYER_SETTINGS_PATH=str(output), VK_LOADER_DEBUG="layer")
    return env


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=("plan", "run"), nargs="?", default="plan")
    parser.add_argument("--build", type=Path, default=ROOT / "target/sdk/build")
    parser.add_argument("--output", type=Path)
    parser.add_argument("--api", choices=("1.2", "1.3", "1.4"), default="1.4")
    parser.add_argument("--validation", choices=("core", "sync"), default="core")
    parser.add_argument("--layer-manifest", type=Path, help="installed VkLayer_khronos_validation.json (required for run)")
    parser.add_argument("--require", action="append", choices=tuple(OPTIONAL), default=[])
    args = parser.parse_args()
    if args.action == "plan":
        print(json.dumps({"required_core": [name for _, name, _ in CORE], "optional": list(OPTIONAL),
            "preconditions": "single physical device under selected ICD, validation layer installed, no software fallback",
            "not_covered": ["WSI/presentation", "video decode", "ray tracing", "full Vulkan 1.4 feature execution",
                            "non-Linux platform import execution"]}, indent=2))
        return
    if not args.output or args.output.exists() or not args.layer_manifest:
        parser.error("run requires a new --output directory and --layer-manifest")
    env = bench.pinned_toolchain(bench.environment())
    # Prevent ambient switches from silently changing which named profiles run.
    for key in list(env):
        if key.startswith("VULKAN_"): del env[key]
    output = args.output.resolve()
    output.mkdir(parents=True)
    env = validation_environment(env, output, args.layer_manifest.resolve(), args.validation)
    current = bench.manifest(env)
    built = json.loads((args.build / "build.json").read_text())
    if current["source_sha256"] != built["source_sha256"]:
        raise ValueError("sources changed since SDK probe build; rebuild before qualification")
    (output / "environment.json").write_text(json.dumps(current, indent=2) + "\n")
    binary = args.build / "rust/release" / ("vk-bench.exe" if os.name == "nt" else "vk-bench")
    stdout, stderr = bench.run([binary, "--case", "probe", "--api", args.api], output, "probe", env)
    if DIAGNOSTICS.search(stdout + stderr): raise RuntimeError("validation diagnostics during probe")
    if LAYER_ACTIVE not in stderr: raise RuntimeError("loader did not confirm validation layer insertion")
    identity = json.loads(stdout)["identity"]
    if identity["device_type"] == 4 or identity["device_count"] != 1:
        raise RuntimeError("select an ICD exposing exactly one physical GPU; existing native tests choose the first compatible device")
    if "VK_LAYER_KHRONOS_validation" not in identity["layers"]:
        raise RuntimeError("validation layer is not installed")
    reports = []
    failed = False

    def save():
        (output / "results.json").write_text(json.dumps({"schema": "vk.sdk.qualification.v1", "identity": identity,
            "validation_profile_requested": args.validation, "profiles": reports,
            "limits": "named workloads only; zero skips required for each qualified profile; no blanket production claim"}, indent=2) + "\n")

    for target, name, gate in CORE:
        # Keep processing after independent profile failures so the laptop run produces a useful inventory.
        try:
            test_env = dict(env, **{gate: "1"})
            stdout, stderr = bench.run(["cargo", "test", "-p", "vkrust", "--locked", "--all-features",
                "--test", target, name, "--", "--exact", "--nocapture", "--test-threads=1"], output, name.replace("::", "-"), test_env)
            if not re.search(r"test result: ok\. 1 passed; 0 failed; 0 ignored", stdout) or SKIPPED.search(stdout + stderr) or DIAGNOSTICS.search(stdout + stderr) or LAYER_ACTIVE not in stderr:
                raise RuntimeError("profile skipped, missing, or emitted validation diagnostics")
            reports.append({"profile": name, "status": "passed"})
        except (RuntimeError, ValueError) as error:
            failed = True
            reports.append({"profile": name, "status": "failed", "reason": str(error)})
        save()
    for label, (target, name, gate, feature) in OPTIONAL.items():
        required = label in args.require
        reason = None
        if feature and not identity["features"][feature]: reason = "queried feature is false"
        if label in ("opaque-fd", "dma-buf", "opaque-fd-image") and not sys.platform.startswith("linux"):
            reason = "Linux import workload"
        if reason:
            reports.append({"profile": label, "status": "unsupported", "reason": reason})
            failed |= required
            save()
            continue
        try:
            stdout, stderr = bench.run(["cargo", "test", "-p", "vkrust", "--locked", "--all-features",
                "--test", target, name, "--", "--exact", "--nocapture", "--test-threads=1"], output, label, dict(env, **{gate: "1"}))
            if DIAGNOSTICS.search(stdout + stderr): raise RuntimeError("validation diagnostics")
            skipped = SKIPPED.search(stdout + stderr)
            if skipped:
                reports.append({"profile": label, "status": "unsupported", "reason": "native workload reported a missing capability; see retained logs"})
                failed |= required
            elif not re.search(r"test result: ok\. 1 passed; 0 failed; 0 ignored", stdout) or LAYER_ACTIVE not in stderr:
                raise RuntimeError("profile did not execute exactly one test")
            else:
                reports.append({"profile": label, "status": "passed"})
        except (RuntimeError, ValueError) as error:
            reports.append({"profile": label, "status": "failed", "reason": str(error)})
            failed = True
        save()
    if failed: raise SystemExit(f"qualification incomplete/failed; retained evidence at {output}")
    print(f"Named required hardware profiles passed; inspect unsupported/open scopes in {output}")


if __name__ == "__main__":
    main()
