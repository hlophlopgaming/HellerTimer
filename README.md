# HellerTimer

[![Made with Slint](https://img.shields.io/badge/Made_with-Slint-007bc7)](https://slint.dev)
[![Checks](https://github.com/hlophlopgaming/HellerTimer/actions/workflows/check.yml/badge.svg)](https://github.com/hlophlopgaming/HellerTimer/actions/workflows/check.yml)

A native Pomodoro timer written in **Rust + Slint**, inspired by [Pomatez](https://github.com/zidoro/pomatez). Named for Joseph Heller, author of *Catch-22*.

**Early development release.** The timer and desktop UI work; full Pomatez parity and the performance budgets in [PLAN.md](PLAN.md) are not yet verified. The web version is a demo; desktop is the main application.

[Download releases](https://github.com/hlophlopgaming/HellerTimer/releases) · [Website & web demo](https://hlophlopgaming.github.io/HellerTimer/) · [Build checks](https://github.com/hlophlopgaming/HellerTimer/actions)

<p align="center"><img src="site/screenshot.png" width="360" alt="HellerTimer running on Linux"></p>
<p align="center"><img src="site/mini.png" width="260" alt="Compact mini window with timer, pause and expand controls"></p>

## Features

- Focus, short and long breaks; configurable rounds; four scheduled special breaks.
- Strict mode, automatic focus starts, optional fullscreen breaks and extra advance notifications.
- Absolute SystemTime deadlines: missed transitions are replayed after sleep, with only the final transition announced.
- Task lists with titles, descriptions, completion, reordering and movement between lists. Press Enter to save text edits; arrows reorder, and → moves to the next list. Only empty lists can be deleted.
- English and Russian interfaces, bundled Noto Sans with Cyrillic, five themes (Light, Dark, Paper, Forest, Midnight) and original Pomatez sounds/voice recordings.
- An original hourglass/loop icon inspired by Joseph Heller’s *Catch-22*.
- Desktop tray with progress, compact mini window, always-on-top, native/custom title bar, close/minimize to tray, single instance and autostart.
- Optional update check 30 seconds after startup, with a five-second request timeout. No network request is needed to start or use the timer.
- Atomic, versioned JSON storage and a 500 ms save debounce. A damaged desktop file is backed up as `data.json.bak`.
- Software rendering by default; optional FemtoVG for comparisons. One single-shot timer schedules core wakeups; no timer worker thread or async runtime in application code.

## Platform status

| Feature | Linux | Windows | macOS | Web | Android |
|---|---|---|---|---|---|
| Timer, tasks, configuration, translations | locally checked | CI build | CI build | WASM checked | SDK/device check pending |
| Tray and progress | StatusNotifier host required | implemented | implemented | — | countdown notification |
| Mini window / pin | compositor dependent | implemented | implemented | in-canvas layout | in-app layout |
| Global shortcuts | X11 | implemented | implemented | — | — |
| Sound / original English voice | `paplay` | native WinMM | `afplay` | bell after interaction | pending |
| System notifications | `notify-send` | pending; hidden | `osascript` | browser permission | Java bridge |
| Autostart | implemented | implemented | implemented | — | — |

Unsupported options are hidden in Settings. On Linux, audio/notifications are offered only when the relevant utilities are installed; tray controls require an active StatusNotifier watcher. GNOME normally needs the AppIndicator extension. Global shortcuts are not registered under Wayland. Always-on-top is hidden there because compositor support varies.

Desktop shortcuts: **Ctrl+Alt+P** start/pause, **Ctrl+Alt+N** skip, **Ctrl+Alt+M** mini window. Conflicting/unsupported shortcuts log a warning and leave the app usable.

## Install / run

Download a package from [Releases](https://github.com/hlophlopgaming/HellerTimer/releases) when available. “Made with Slint” attribution and complete notices accompany the packages.

```
cargo run -p heller-desktop
cargo run -p heller-desktop -- --start
cargo run -p heller-desktop -- --mini
cargo build --release -p heller-desktop --features femtovg
```

Requires stable Rust 1.92+ and platform libraries for Slint/winit. Ubuntu:

```
sudo apt install libxkbcommon-dev libwayland-dev libfontconfig1-dev libx11-xcb-dev
```

Optional Linux integration utilities: `pulseaudio-utils` (`paplay`, also works with PipeWire's PulseAudio compatibility server), `libnotify-bin`, `curl`, and `busctl` from systemd. There is no GTK dependency.

CLI: `--start`, `--hidden`, `--mini`, `--renderer software|femtovg`, `--animation`, `--no-animation`, `--bench-first-frame`, `--help`. FemtoVG requires the compile feature above. `--hidden` falls back to showing a window when a tray host is unavailable. The first-frame instrumentation reports a value only when the active renderer supports Slint's rendering notifier.

A `portable` file beside the executable selects a neighboring `data/` directory; otherwise platform config directories are used. Windows portable archives include this marker. There is no import from Pomatez.

Unsigned macOS packages may require **Open Anyway** in System Settings → Privacy & Security. No Apple signing credentials are bundled. APKs are debug signed until a release keystore is configured.

## Web and Android

```
rustup target add wasm32-unknown-unknown
cargo check -p heller-web --target wasm32-unknown-unknown
cargo install trunk --locked
cd crates/heller-web
trunk serve
```

Browser notifications require permission and a secure origin (HTTPS or localhost); sound needs a user interaction. Hidden tabs may delay notifications due to browser throttling, while remaining time is recomputed from deadlines on return. Theme selection is saved locally. Version-1 dark/light settings migrate to version 2 without changing tasks or timer configuration. Browser storage can be unavailable in private/locked-down sessions; storage errors are logged. The PWA manifest enables standalone installation; offline caching is not implemented.

Android: Java 17, Android SDK platform 35, NDK and Gradle 8.11.1 are required. Install `cargo-ndk`, add the three Rust Android targets, then follow `.github/workflows/release.yml` to generate JNI libraries and run `gradle assembleDebug` in `crates/heller-android/android`. Exact alarm denial falls back to an inexact alarm; notification permission is requested on Android 13+. The Java receiver can announce the next deadline without the UI process. **Continuing multiple phases after process death is not implemented:** reopen the app to continue. Android builds and IME need a real-device test.

## Verification

```
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace -- -D warnings
cargo test -p heller-core
cargo check -p heller-web --target wasm32-unknown-unknown
cargo check -p heller-android --target aarch64-linux-android
python bench/bench.py --help
python -m unittest discover -s bench
```

To exercise actual Slint controls and window sizes in a graphical session:

```
cargo run -p heller-desktop --example ui-smoke
cargo run -p heller-desktop --example ui-smoke -- --start-mini
cargo run -p heller-desktop --example ui-smoke -- --frameless
```

These scenarios send pointer events, exercise every mini-window entry point and restoration (including hidden windows), timer controls, tasks, configuration, all themes, Russian and persistence. They use an in-memory platform and never modify your user data. Raw RGBA screenshots and dimensions go to `$HELLER_SCREENSHOT_DIR` or a temporary `hellertimer-ui-smoke` directory. CI runs the same scenarios under Xvfb.

The last Android command requires an SDK/JDK even for `cargo check`. Long packaging and release jobs run in CI. Push a `v*` tag to build release packages; the workflow publishes a prerelease only when all platform jobs succeed. GitHub Pages deploys from `main` using Actions. No PR is required.

## Performance

See [benchmark instructions](bench/README.md). Targets (not measured claims): ≤30 MiB visible PSS, ≤20 MiB tray PSS, near-zero hidden CPU, ≤250 ms startup. Both renderers must be compared against the full Pomatez process tree on the same machine.

<!-- bench:start -->
No comparable benchmark has been published yet.
<!-- bench:end -->

The built-in Slint tray uses ksni on Linux; its internal dependencies are outside the pure timer core. Native audio helpers run only during effects and release the audio device when done. The mini mode reuses the main window. Visible progress interpolation is optional and disabled by default.

## Remaining parity work

This initial implementation does **not** yet satisfy the full definition of done in PLAN.md. Remaining work includes exact visual/spacing parity, task drag-and-drop and arbitrary move menus, system-theme following, a separate compact layout, Windows system notifications, browser/Android voice, Android sound and process-death phase continuation, cold-start/installed-size benchmark automation, macOS universal/signing/notarization, and device verification. Special-break timezone offsets are resolved at startup; restart after a DST/timezone change. Timer sessions are not persisted across process restarts.

## License and attribution

Project source: [MIT](LICENSE). Slint is used under the [Slint Royalty-free License 2.0](assets/licenses/Slint-Royalty-free-2.0.md), with its AboutSlint widget in Settings and attribution alongside downloads. [Third-party notices](THIRD_PARTY.md) cover Pomatez's MIT assets, Noto Sans (OFL), and direct dependencies. Thank you to Pomatez and Slint contributors.
