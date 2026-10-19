#!/usr/bin/env python3
import argparse
import html
import json
import shutil
from pathlib import Path


def report(data):
    rows = data["measurements"]
    lines = ["| Application | Scenario | Private MiB | CPU % |", "|---|---|---:|---:|"]
    bars = []
    for i, r in enumerate(rows):
        mib = r["private_bytes"] / 1048576
        lines.append(f'| {r["application"]} | {r["scenario"]} | {mib:.1f} | {r["cpu_percent"]:.2f} |')
        label = html.escape(f'{r["application"]} · {r["scenario"]}')
        bars.append(f'<text x="8" y="{i*40+18}" fill="#a7abae">{label}</text><rect x="350" y="{i*40+4}" width="{min(mib,600)}" height="20" fill="#0098f7"/><text x="{360+min(mib,600)}" y="{i*40+18}" fill="#e3e4e5">{mib:.1f} MiB</text>')
    svg = f'<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="{max(40,len(rows)*40)}"><rect width="100%" height="100%" fill="#111d25"/>{"".join(bars)}</svg>'
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
        readme = root / "README.md"
        text = readme.read_text()
        start, tail = text.split("<!-- bench:start -->", 1)
        _, end = tail.split("<!-- bench:end -->", 1)
        readme.write_text(start + "<!-- bench:start -->\n" + markdown + "<!-- bench:end -->" + end)


if __name__ == "__main__":
    main()
