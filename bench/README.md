# Reproducible local measurements

Install Python 3 and `psutil` (`python -m pip install psutil`). Build HellerTimer with both renderers:

```
cargo build --release -p heller-desktop --features femtovg
python bench/bench.py --heller ./target/release/hellertimer --pomatez /usr/bin/pomatez
python bench/bench.py --pomatez-attach pomatez
python bench/report.py bench/results/HOST-DATE.json --publish
```

Close existing HellerTimer instances before launching. Align focus/break durations, theme, notifications, sound, voice, update checks and animation. The script pauses before each scenario so that Pomatez can be configured manually. Defaults: 30 seconds warmup, 120 seconds measurement. Linux needs a tray host (GNOME: AppIndicator extension). Do not interact during a sample.

Private memory is PSS on Linux and USS on Windows/macOS; RSS is also recorded. All memory, CPU, thread and process counts include the entire process tree. CPU is one-core percent, so values may exceed 100. Linux voluntary context switches count all threads; these are a proxy for wakeups, not hardware wakeup counters. Process churn can make cumulative CPU deltas undercount; rerun samples if subprocesses restart. Attached processes are never stopped.

`report.py --publish` explicitly opts into copying measured results to the website and README. No performance claims are published before real measurements. Cold-start window detection, installed size and package-size measurements are not automated yet; measure those separately. `--bench-first-frame` uses the renderer callback on desktop, rather than time to process creation.

Quick self-check: `python -m unittest discover -s bench`.
