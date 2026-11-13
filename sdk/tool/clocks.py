#!/usr/bin/env python3
"""Explicit, reversible Linux CPU/i915 clock controls; snapshot never writes sysfs."""
import argparse
import json
import os
from pathlib import Path
import platform
import stat

CPU = Path("/sys/devices/system/cpu")
DRM = Path("/sys/class/drm")


def controls(card):
    if not isinstance(card, str) or not card.startswith("card") or not card[4:].isdigit():
        raise ValueError("invalid DRM card name")
    pairs = [(p / "scaling_min_freq", p / "scaling_max_freq", p / "base_frequency")
             for p in sorted((CPU / "cpufreq").glob("policy*"))]
    gpu = DRM / card
    if (gpu / "gt_min_freq_mhz").exists():
        pairs.append((gpu / "gt_min_freq_mhz", gpu / "gt_max_freq_mhz", gpu / "gt_RP1_freq_mhz"))
    return pairs


def extra_paths(card):
    candidates = [CPU / "intel_pstate/no_turbo", CPU / "cpufreq/boost", DRM / card / "gt_boost_freq_mhz"]
    return [p for p in candidates if p.exists()]


def snapshot(card):
    paths = {p for pair in controls(card) for p in pair if p.exists()} | set(extra_paths(card))
    paths |= set((CPU / "cpufreq").glob("policy*/scaling_governor"))
    paths |= set(Path("/sys/class/thermal").glob("thermal_zone*/temp"))
    paths |= set((DRM / card).glob("gt_*freq_mhz"))
    return {"schema": "vk.sdk.clocks.v1", "host": platform.node(), "card": card,
            "boot_id": Path("/proc/sys/kernel/random/boot_id").read_text().strip(),
            "values": {str(p): p.read_text().strip() for p in sorted(paths)}}


def set_pair(low, high, target_low, target_high):
    # Raise the ceiling before moving the floor; then lower the ceiling if needed.
    old_high = int(high.read_text())
    high.write_text(str(max(old_high, target_high, target_low)))
    low.write_text(str(target_low))
    high.write_text(str(target_high))
    if (int(low.read_text()), int(high.read_text())) != (target_low, target_high):
        raise RuntimeError(f"clock control did not retain requested limits: {low.parent}")


def restore(state):
    now = snapshot(state["card"])
    if any(now[key] != state[key] for key in ("host", "boot_id", "schema")):
        raise ValueError("clock state belongs to another host/boot")
    pairs = controls(state["card"])
    allowed = {str(p) for low, high, _ in pairs for p in (low, high)} | {str(p) for p in extra_paths(state["card"])}
    if set(state["restore"]) != allowed:
        raise ValueError("restore paths differ from discovered controls")
    # Never trust a state file to select arbitrary root-writable paths.
    errors = []
    for low, high, _ in pairs:
        try:
            set_pair(low, high, int(state["restore"][str(low)]), int(state["restore"][str(high)]))
        except (OSError, ValueError, RuntimeError) as error:
            errors.append(str(error))
    for path in extra_paths(state["card"]):
        try:
            path.write_text(str(int(state["restore"][str(path)])))
        except (OSError, ValueError) as error:
            errors.append(str(error))
    if errors:
        raise RuntimeError("partial clock restore; retain state and restore remaining controls: " + "; ".join(errors))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=("snapshot", "apply", "restore"))
    parser.add_argument("--state", type=Path, help="new saved state for apply, existing state for restore")
    parser.add_argument("--cpu-khz", type=int, help="explicit fixed CPU clock at or below reported base")
    parser.add_argument("--gpu-mhz", type=int, help="explicit fixed i915 GPU clock")
    parser.add_argument("--card", default="card0")
    args = parser.parse_args()
    if not args.card.startswith("card") or not args.card[4:].isdigit():
        parser.error("card must be card followed by digits")
    if args.action == "snapshot":
        print(json.dumps(snapshot(args.card), indent=2))
        return
    if os.geteuid() != 0 or args.state is None:
        parser.error("apply/restore require root and --state; never invoked by benchmarks automatically")
    if args.action == "restore":
        descriptor = os.open(args.state, os.O_RDONLY | os.O_NOFOLLOW)
        with os.fdopen(descriptor, "r") as stream:
            metadata = os.fstat(stream.fileno())
            if metadata.st_uid != 0 or metadata.st_mode & 0o077 or not stat.S_ISREG(metadata.st_mode):
                raise ValueError("restore state must be a root-owned private regular file")
            restore(json.load(stream))
        return
    if args.cpu_khz is None or args.cpu_khz <= 0 or (args.gpu_mhz is not None and args.gpu_mhz <= 0):
        parser.error("apply requires positive --cpu-khz and optional positive --gpu-mhz")
    pairs = controls(args.card)
    cpu_pairs = [pair for pair in pairs if str(pair[0]).startswith(str(CPU))]
    if not cpu_pairs:
        parser.error("no supported CPU frequency controls found")
    gpu_pairs = [pair for pair in pairs if pair not in cpu_pairs]
    if args.gpu_mhz is not None and not gpu_pairs:
        parser.error("no supported i915 GPU controls; xe/other drivers need a separately reviewed adapter")
    for _, _, base in cpu_pairs:
        if not base.exists() or args.cpu_khz > int(base.read_text()):
            parser.error("CPU base frequency absent or requested clock exceeds base; inspect snapshot")
    state = snapshot(args.card)
    state["restore"] = {str(p): p.read_text().strip() for low, high, _ in pairs for p in (low, high)}
    state["restore"].update({str(p): p.read_text().strip() for p in extra_paths(args.card)})
    # Exclusive creation, no symlink following or overwriting existing recovery evidence.
    descriptor = os.open(args.state, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
    with os.fdopen(descriptor, "w") as stream:
        stream.write(json.dumps(state, indent=2) + "\n")
    try:
        for low, high, _ in cpu_pairs:
            set_pair(low, high, args.cpu_khz, args.cpu_khz)
        for path in extra_paths(args.card):
            if path.name == "no_turbo": path.write_text("1")
            elif path.name == "boost": path.write_text("0")
        if args.gpu_mhz is not None:
            for low, high, _ in gpu_pairs:
                set_pair(low, high, args.gpu_mhz, args.gpu_mhz)
            boost = DRM / args.card / "gt_boost_freq_mhz"
            if boost.exists(): boost.write_text(str(args.gpu_mhz))
    except BaseException:
        restore(state)
        raise
    print(json.dumps(snapshot(args.card), indent=2))


if __name__ == "__main__":
    main()
