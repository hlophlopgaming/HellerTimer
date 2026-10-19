#!/usr/bin/env python3
"""Non-interactive whole-tree measurements of HellerTimer vs Pomatez.

Unlike bench.py, this never pauses for manual configuration: HellerTimer drives
its own timer through its --start flag, and Pomatez is measured in its default
idle-visible state. The measurement schema and process-tree accounting match
bench.py, so report.py consumes both outputs unchanged.
"""
import argparse
import json
import platform
import shlex
import subprocess
import time
from pathlib import Path

import bench
import psutil


def run_one(name, scenario, command, warmup, seconds):
    proc = subprocess.Popen(command)
    try:
        time.sleep(warmup)
        root = psutil.Process(proc.pid)
        row = bench.measure(root, seconds)
    finally:
        try:
            bench.stop(psutil.Process(proc.pid))
        except psutil.Error:
            pass
        if proc.poll() is None:
            try:
                proc.kill()
            except OSError:
                pass
    row.update(application=name, scenario=scenario)
    return row


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--heller", default="target/release/hellertimer")
    parser.add_argument("--pomatez", default="pomatez")
    parser.add_argument("--seconds", type=float, default=30)
    parser.add_argument("--warmup", type=float, default=15)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()

    heller = Path(args.heller).resolve()
    pomatez = shlex.split(args.pomatez)

    scenarios = [
        ("HellerTimer/software", "idle-visible", [str(heller), "--renderer", "software", "--no-animation"]),
        ("HellerTimer/femtovg", "idle-visible", [str(heller), "--renderer", "femtovg", "--no-animation"]),
        ("HellerTimer/software", "running-static", [str(heller), "--renderer", "software", "--start", "--no-animation"]),
        ("HellerTimer/femtovg", "running-static", [str(heller), "--renderer", "femtovg", "--start", "--no-animation"]),
        ("HellerTimer/software", "mini", [str(heller), "--renderer", "software", "--mini", "--no-animation"]),
        ("Pomatez", "idle-visible", pomatez),
    ]

    result = {
        "version": 1,
        "host": platform.node(),
        "os": platform.platform(),
        "cpu": platform.processor(),
        "ram_bytes": psutil.virtual_memory().total,
        "timestamp": time.time(),
        "measurements": [],
    }
    for name, scenario, command in scenarios:
        print(f"{name} / {scenario}: warmup {args.warmup}s, measure {args.seconds}s")
        row = run_one(name, scenario, command, args.warmup, args.seconds)
        result["measurements"].append(row)
        print(json.dumps(row, indent=2))

    output = args.output or Path("bench/results") / f"{platform.node()}-auto-{time.strftime('%Y%m%d-%H%M%S')}.json"
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(result, indent=2) + "\n")
    print(output)


if __name__ == "__main__":
    main()
