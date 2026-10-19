# Third-party notices

HellerTimer source code is MIT licensed. Dependencies retain their own licenses.

* **Slint 1.18.1** — https://slint.dev — used under the Slint Royalty-free License 2.0, not the alternative GPL license. Full terms: [Slint license](assets/licenses/Slint-Royalty-free-2.0.md). The AboutSlint widget is available in Settings; attribution is included beside downloads and in README. Built-in tray and accessibility support bring their own transitive dependencies.
* **Pomatez** — Copyright (c) 2024 Roldan Montilla Jr — https://github.com/zidoro/pomatez — MIT. `assets/sounds/bell.wav` from `app/renderer/src/assets/audios/notification-bell.wav`; `assets/sounds/{focus-finished,break-finished,special-break-started,sixty-seconds-left,thirty-seconds-left}.wav` from the matching files under `app/renderer/src/assets/audios/`; palette values in `theme.slint` from `app/renderer/src/styles/themes.ts`. Full notice: [Pomatez MIT](assets/licenses/Pomatez-MIT.txt).
* **Noto Sans Regular** — Copyright Google LLC, SIL Open Font License 1.1. `assets/fonts/NotoSans-Regular.ttf` from https://github.com/notofonts/noto-fonts/tree/main/hinted/ttf/NotoSans. Used by Pomatez; includes Cyrillic. Full notice: [OFL](assets/licenses/Noto-OFL.txt).
* **serde / serde_json** — MIT OR Apache-2.0; versioned configuration and task serialization.
* **log** — MIT OR Apache-2.0; recoverable error reporting.
* **directories** — MIT OR Apache-2.0; platform data paths.
* **auto-launch** — MIT; native autostart registration on desktop.
* **global-hotkey** — MIT OR Apache-2.0; desktop shortcuts without polling.
* **single-instance** — MIT; native process-instance lock.
* **wasm-bindgen / js-sys / web-sys** — MIT OR Apache-2.0; browser entry point, local storage, visibility and notifications.
* **console_error_panic_hook** — MIT OR Apache-2.0; renders WASM panics as readable console errors in the web demo.
* **jni** — MIT OR Apache-2.0; Android Java storage and alarm bridge.

Use `cargo tree --locked` for exact transitive versions in Cargo.lock. No additional third-party assets are embedded in the site.

The HellerTimer hourglass icon (`assets/icon.png`, copied to the site and Android resources) and its monochrome tray variant (`assets/tray-icon.png`) are original generated project branding, not Pomatez assets. Their concept, generation method and exact prompts are recorded in [assets/branding/README.md](assets/branding/README.md).
