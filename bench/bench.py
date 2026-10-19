#!/usr/bin/env python3
"""Measure whole process trees; never substitute estimates for measurements."""
import argparse
import json
import os
import platform
import shlex
import statistics
import subprocess
import time
from pathlib import Path


def tree(root):
    import psutil
    try:
        return [root] + root.children(recursive=True)
    except psutil.Error:
        return []


def sample(root):
    import psutil
    total = dict(rss=0, private=0, cpu_seconds=0, threads=0, processes=0, wakeups=0)
    private_kind = "pss" if platform.system() == "Linux" else "uss"
    for proc in tree(root):
        try:
            memory = proc.memory_full_info()
            total["rss"] += memory.rss
            total["private"] += getattr(memory, private_kind)
            cpu = proc.cpu_times()
            total["cpu_seconds"] += cpu.user + cpu.system
            total["threads"] += proc.num_threads()
            total["processes"] += 1
            if platform.system() == "Linux":
                for status in Path(f"/proc/{proc.pid}/task").glob("*/status"):
                    try:
                        for line in status.read_text().splitlines():
                            if line.startswith("voluntary_ctxt_switches:"):
                                total["wakeups"] += int(line.split()[1])
                    except (OSError, ValueError):
                        pass
        except (psutil.Error, AttributeError) as e:
            raise RuntimeError(f"Cannot measure PID {proc.pid}: {e}") from e
    if total["processes"] == 0:
        raise RuntimeError("Measured application exited")
    return total


def measure(root, seconds):
    before = sample(root)
    started = time.monotonic()
    rows = []
    while time.monotonic() - started < seconds:
        time.sleep(min(1, max(0, seconds - (time.monotonic() - started))))
        rows.append(sample(root))
    elapsed = time.monotonic() - started
    after = rows[-1]
    return {
        "memory_kind": "PSS" if platform.system() == "Linux" else "USS",
        "private_bytes": statistics.mean(r["private"] for r in rows),
        "rss_bytes": statistics.mean(r["rss"] for r in rows),
        "cpu_percent": max(0, after["cpu_seconds"] - before["cpu_seconds"]) / elapsed * 100,
        "wakeups_per_second": max(0, after["wakeups"] - before["wakeups"]) / elapsed if platform.system() == "Linux" else None,
        "processes": max(r["processes"] for r in rows),
        "threads": max(r["threads"] for r in rows),
        "duration_seconds": elapsed,
    }


def stop(root):
    import psutil
    procs = tree(root)
    for proc in reversed(procs):
        try:
            proc.terminate()
        except psutil.Error:
            pass
    _, alive = psutil.wait_procs(procs, timeout=5)
    for proc in alive:
        try:
            proc.kill()
        except psutil.Error:
            pass


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--heller", help="Path to HellerTimer (built with --features femtovg)")
    parser.add_argument("--pomatez", help="Pomatez launch command")
    parser.add_argument("--pomatez-attach", help="Process name for a running snap/flatpak Pomatez")
    parser.add_argument("--seconds", type=float, default=120)
    parser.add_argument("--warmup", type=float, default=30)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    if not (args.heller or args.pomatez or args.pomatez_attach):
        parser.error("Supply --heller, --pomatez or --pomatez-attach")
    if args.seconds <= 0 or args.warmup < 0:
        parser.error("Measurement duration must be positive; warmup must be nonnegative")
    import psutil
    scenarios = ["idle-visible", "running-animated", "running-static", "running-tray", "mini"]
    result = {"version": 1, "host": platform.node(), "os": platform.platform(), "cpu": platform.processor(), "ram_bytes": psutil.virtual_memory().total, "timestamp": time.time(), "measurements": []}
    launches = []
    if args.heller:
        for renderer in ("software", "femtovg"):
            launches.append((f"HellerTimer/{renderer}", [args.heller, "--renderer", renderer], None))
    if args.pomatez:
        launches.append(("Pomatez", shlex.split(args.pomatez), None))
    if args.pomatez_attach:
        matches = [p for p in psutil.process_iter(["name"]) if args.pomatez_attach.lower() in (p.info["name"] or "").lower()]
        if not matches:
            parser.error("No matching Pomatez process")
        launches.append(("Pomatez/attached", None, min(matches, key=lambda p: p.create_time())))
    for name, command, attached in launches:
        for scenario in scenarios:
            if name.startswith("Pomatez") and scenario == "mini":
                continue
            proc = None
            root = attached
            try:
                if command:
                    flags = []
                    if name.startswith("HellerTimer"):
                        if scenario != "idle-visible": flags.append("--start")
                        flags.append("--animation" if scenario == "running-animated" else "--no-animation")
                        if scenario == "running-tray": flags.append("--hidden")
                        if scenario == "mini": flags.append("--mini")
                    proc = subprocess.Popen(command + flags)
                    root = psutil.Process(proc.pid)
                input(f"{name}: configure {scenario}; match durations and disable sound/updates in both apps. Press Enter when ready. ")
                time.sleep(args.warmup)
                row = measure(root, args.seconds)
                row.update(application=name, scenario=scenario)
                result["measurements"].append(row)
                print(json.dumps(row, indent=2))
            finally:
                if proc and root:
                    stop(root)
    output = args.output or Path("bench/results") / f"{platform.node()}-{time.strftime('%Y%m%d-%H%M%S')}.json"
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(result, indent=2) + "\n")
    print(output)


if __name__ == "__main__":
    main()
