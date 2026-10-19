# HellerTimer

[![Made with Slint](https://img.shields.io/badge/Made_with-Slint-007bc7)](https://slint.dev)
[![Checks](https://github.com/hlophlopgaming/HellerTimer/actions/workflows/check.yml/badge.svg)](https://github.com/hlophlopgaming/HellerTimer/actions/workflows/check.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[English](README.md) · [Русский](README.ru.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [Português](README.pt.md) · [Italiano](README.it.md) · [中文](README.zh.md) · [日本語](README.ja.md)

一个用 **Rust + Slint** 编写的原生番茄钟，灵感来自 [Pomatez](https://github.com/zidoro/pomatez)。以《第二十二条军规》（*Catch-22*）的作者约瑟夫·海勒命名。

计时器和桌面界面均可正常工作；网页版是演示，桌面版是主要应用。

[下载发行版](https://github.com/hlophlopgaming/HellerTimer/releases) · [网站与网页演示](https://hlophlopgaming.github.io/HellerTimer/) · [构建](https://github.com/hlophlopgaming/HellerTimer/actions)

<p align="center"><img src="site/screenshot.png" width="360" alt="运行于 Linux 的 HellerTimer"></p>
<p align="center"><img src="site/mini.png" width="260" alt="带计时器、暂停和展开按钮的紧凑迷你窗口"></p>

## 功能

- 专注、短休息和长休息；可配置轮次；四个计划特殊休息。
- 严格模式、自动开始专注、可选的全屏休息和额外提前通知。
- 基于 `SystemTime` 的绝对截止时间：睡眠后错过的阶段会重放，只播报最后一次切换。
- 任务列表，支持标题、描述、完成状态、排序和跨列表移动。回车保存编辑；方向键排序，→ 移到下一列表。只能删除空列表。
- 英语和俄语界面（另有七种语言），内置带西里尔字母的 Noto Sans，五种主题（Light、Dark、Paper、Forest、Midnight）以及 Pomatez 的原声/录音。
- 灵感来自《第二十二条军规》的原创沙漏图标。
- 带进度的桌面托盘、紧凑迷你窗口、置顶、原生/自定义标题栏、关闭/最小化到托盘、单实例和开机自启。
- 启动 30 秒后可选检查更新，请求超时 5 秒。启动和使用计时器无需网络。
- 原子化、带版本号的 JSON 存储，500 毫秒防抖保存。损坏的桌面文件会备份为 `data.json.bak`。
- 默认软件渲染；可选 FemtoVG 用于对比。单个 single-shot 计时器调度核心唤醒；无计时器工作线程，无异步运行时。

## 平台状态

| 功能 | Linux | Windows | macOS | Web | Android |
|---|---|---|---|---|---|
| 计时器、任务、配置、翻译 | 本地已验证 | CI 构建 | CI 构建 | WASM 已验证 | SDK/设备待验证 |
| 托盘与进度 | 需要 StatusNotifier 宿主 | 已实现 | 已实现 | — | 倒计时通知 |
| 迷你窗口 / 固定 | 取决于合成器 | 已实现 | 已实现 | 画布内 | 布局内 |
| 全局快捷键 | X11 | 已实现 | 已实现 | — | — |
| 声音 / 英语语音 | `paplay` | 原生 WinMM | `afplay` | 交互后铃声 | 待定 |
| 系统通知 | `notify-send` | 待定；已隐藏 | `osascript` | 浏览器授权 | Java 桥接 |
| 开机自启 | 已实现 | 已实现 | 已实现 | — | — |

不支持的选项会在设置中隐藏。在 Linux 上，仅当安装了相应工具时才提供声音和通知；托盘需要活动的 StatusNotifier 监视器。GNOME 通常需要 AppIndicator 扩展。Wayland 下不注册全局快捷键。「置顶」在那里会被隐藏。

桌面快捷键：**Ctrl+Alt+P** 开始/暂停，**Ctrl+Alt+N** 跳过，**Ctrl+Alt+M** 迷你窗口。冲突或不支持的快捷键会记录日志，应用仍可正常使用。

## 安装 / 运行

从 [Releases](https://github.com/hlophlopgaming/HellerTimer/releases) 下载软件包。软件包随附「Made with Slint」署名和完整声明。

```
cargo run -p heller-desktop
cargo run -p heller-desktop -- --start
cargo run -p heller-desktop -- --mini
cargo build --release -p heller-desktop --features femtovg
```

需要 Rust 稳定版 1.92+ 以及 Slint/winit 的平台库。Ubuntu：

```
sudo apt install libxkbcommon-dev libxkbcommon-x11-dev libwayland-dev libfontconfig1-dev libx11-xcb-dev
```

可选 Linux 工具：`pulseaudio-utils`（`paplay`，也兼容 PipeWire 的 PulseAudio 兼容服务）、`libnotify-bin`、`curl` 和 systemd 的 `busctl`。无 GTK 依赖。

命令行：`--start`、`--hidden`、`--mini`、`--renderer software|femtovg`、`--animation`、`--no-animation`、`--bench-first-frame`、`--help`。FemtoVG 需要上面的编译特性。`--hidden` 在无托盘宿主时会回退为显示窗口。首帧插桩仅在渲染器支持 Slint 渲染通知器时报告数值。

可执行文件旁的 `portable` 文件会选择相邻的 `data/` 目录；否则使用平台配置目录。Windows 便携包包含此标记。没有从 Pomatez 导入。

未签名的 macOS 包可能需要在「系统设置 → 隐私与安全性」中选择 **Open Anyway**。未包含 Apple 签名凭据。APK 在配置 release keystore 之前使用 debug 签名。

## 网页与 Android

```
rustup target add wasm32-unknown-unknown
cargo check -p heller-web --target wasm32-unknown-unknown
cargo install trunk --locked
cd crates/heller-web
trunk serve
```

浏览器通知需要授权和安全源（HTTPS 或 localhost）；声音需要用户交互。隐藏标签页可能因浏览器节流而延迟通知；返回时会根据截止时间重新计算剩余时间。主题选择保存在本地。版本 1 的深色/浅色设置迁移到版本 2，不改变任务或计时器配置。在无痕/受限会话中浏览器存储可能不可用；错误会记录日志。PWA 清单支持独立安装；未实现离线缓存。

Android：需要 Java 17、Android SDK platform 35、NDK 和 Gradle 8.11.1。安装 `cargo-ndk`，添加三个 Rust Android 目标，然后按 `.github/workflows/release.yml` 生成 JNI 库并在 `crates/heller-android/android` 中运行 `gradle assembleDebug`。精确闹钟被拒会回退为不精确闹钟；Android 13+ 会请求通知权限。Java 接收器无需 UI 进程即可播报下一截止时间。**进程死亡后继续多个阶段尚未实现：**请重新打开应用。Android 构建和输入法需要真机测试。

## 验证

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

在图形会话中测试真实的 Slint 控件和窗口尺寸：

```
cargo run -p heller-desktop --example ui-smoke
cargo run -p heller-desktop --example ui-smoke -- --start-mini
cargo run -p heller-desktop --example ui-smoke -- --frameless
```

这些场景会发送指针事件，测试迷你窗口的每个入口和恢复（包括隐藏窗口）、原生拖拽交接和指针释放、计时器控件、任务、配置、所有主题、俄语和持久化。它们使用内存平台，绝不修改你的用户数据。RGBA 截图和尺寸输出到 `$HELLER_SCREENSHOT_DIR` 或临时 `hellertimer-ui-smoke` 目录。CI 在 Xvfb 下运行相同场景。

最后一条 Android 命令即使只是 `cargo check` 也需要 SDK/JDK。冗长的打包和发布任务在 CI 中运行。推送 `v*` 标签会构建发布包；工作流立即发布一个标记为 Latest 的正式 release，随后附加软件包。GitHub Pages 通过 Actions 从 `main` 部署。无需 PR。

## 性能

见[基准测试说明](bench/README.md)。目标：≤30 MiB 可见 PSS、≤20 MiB 托盘 PSS、近乎为零的后台 CPU、≤250 ms 启动。两种渲染器都要与同一台机器上的完整 Pomatez 进程树对比。

<!-- bench:start -->
在桌面 Linux 上对照 Pomatez 1.11.0 测得（完整进程树，PSS）。见[基准测试说明](bench/README.md)。图表：[SVG](site/bench/benchmark.svg)。

| Application | Scenario | Private MiB | CPU % |
|---|---|---:|---:|
| HellerTimer/software | idle-visible | 16.9 | 0.00 |
| HellerTimer/femtovg | idle-visible | 51.9 | 0.00 |
| HellerTimer/software | running-static | 17.3 | 0.07 |
| HellerTimer/femtovg | running-static | 49.6 | 0.10 |
| HellerTimer/software | mini | 15.9 | 0.00 |
| Pomatez | idle-visible | 301.5 | 1.07 |
<!-- bench:end -->

内置的 Slint 托盘在 Linux 上使用 ksni；其内部依赖不在纯计时器核心之内。原生音频助手仅在音效期间运行，完成后释放设备。迷你模式复用主窗口。可见进度插值可选，默认关闭。

## 剩余的兼容性工作

本实现尚未完全满足 PLAN.md 中的完整完成定义：精确的视觉/间距对齐、任务拖放和任意移动菜单、跟随系统主题、单独的紧凑布局、Windows 系统通知、浏览器/Android 语音、Android 声音和进程死亡后的阶段延续、冷启动/安装体积基准自动化、macOS 通用/签名/公证以及设备验证。特殊休息的时区偏移在启动时解析；更改时区/DST 后请重启。计时器会话不会在进程重启后保留。

## 许可证与署名

项目源码：[MIT](LICENSE)。Slint 依据 [Slint Royalty-free License 2.0](assets/licenses/Slint-Royalty-free-2.0.md) 使用，设置中有 AboutSlint 控件，下载旁有署名。[第三方声明](THIRD_PARTY.md)涵盖 Pomatez 的 MIT 素材、Noto Sans（OFL）和直接依赖。感谢 Pomatez 与 Slint 的贡献者。
