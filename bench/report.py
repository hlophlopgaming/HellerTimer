#!/usr/bin/env python3
import argparse
import html
import json
import shutil
from pathlib import Path

BG = "#111d25"
FG = "#e3e4e5"
MUTED = "#a7abae"
BLUE = "#0098f7"
RED = "#e04a5f"


def report(data):
    rows = data["measurements"]
    lines = ["| Application | Scenario | Private MiB | CPU % |", "|---|---|---:|---:|"]
    max_mib = max((r["private_bytes"] / 1048576 for r in rows), default=1)
    bar_w = 560.0
    label_w = 330.0
    row_h = 44
    top = 40
    bars = []
    for i, r in enumerate(rows):
        mib = r["private_bytes"] / 1048576
        cpu = r["cpu_percent"]
        lines.append(f'| {r["application"]} | {r["scenario"]} | {mib:.1f} | {cpu:.2f} |')
        label = html.escape(f'{r["application"]} · {r["scenario"]}')
        width = max(2.0, mib / max_mib * bar_w)
        color = RED if "Pomatez" in r["application"] else BLUE
        y = top + i * row_h
        bars.append(
            f'<text x="8" y="{y + 28}" fill="{MUTED}" font-size="14">{label}</text>'
            f'<rect x="{label_w}" y="{y + 8}" width="{width:.1f}" height="22" rx="4" fill="{color}"/>'
            f'<text x="{label_w + width + 8}" y="{y + 25}" fill="{FG}" font-size="14">{mib:.1f} MiB · {cpu:.2f}% CPU</text>'
        )
    height = top + max(row_h, len(rows) * row_h) + 8
    svg = (
        f'<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="{height}">'
        f'<rect width="100%" height="100%" fill="{BG}"/>'
        f'<text x="8" y="22" fill="{FG}" font-size="15" font-weight="600">Private memory by application (lower is better)</text>'
        f'{"".join(bars)}</svg>'
    )
    return "\n".join(lines) + "\n", svg


def main():
    p = argparse.ArgumentParser()
    p.add_argument("result", type=Path)
    p.add_argument("--publish", action="store_true")
    args = p.parse_args()
    data = json.loads(args.result.read_text())
    markdown, svg = report(data)
    args.result.with_suffix(".md").write_text(markdown)
    args.result.with_suffix(".svg").write_text(svg)
    if args.publish:
        root = Path(__file__).resolve().parents[1]
        shutil.copyfile(args.result, root / "site/bench/latest.json")
        shutil.copyfile(args.result.with_suffix(".svg"), root / "site/bench/benchmark.svg")
        readme = root / "README.md"
        text = readme.read_text()
        start, tail = text.split("<!-- bench:start -->", 1)
        _, end = tail.split("<!-- bench:end -->", 1)
        readme.write_text(start + "<!-- bench:start -->\n" + markdown + "<!-- bench:end -->" + end)


if __name__ == "__main__":
    main()
