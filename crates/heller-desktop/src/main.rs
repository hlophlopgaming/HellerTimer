use heller_ui::{App, Caps, Platform};
use slint::ComponentHandle;
use std::{path::PathBuf, rc::Rc};
slint::include_modules!();
struct Desktop {
    dir: PathBuf,
    tray: std::cell::RefCell<Option<Tray>>,
    progress: std::cell::Cell<(u8, u8)>,
    caps: Caps,
}
impl Desktop {
    fn new() -> Self {
        let portable = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .filter(|p| p.join("portable").exists());
        let dir = portable
            .map(|p| p.join("data"))
            .or_else(|| {
                directories::ProjectDirs::from("io.github", "hlophlopgaming", "HellerTimer")
                    .map(|d| d.config_dir().to_path_buf())
            })
            .unwrap_or_else(|| PathBuf::from(".hellertimer"));
        let mut caps = Caps {
            tray: true,
            updates: true,
            notifications: cfg!(target_os = "macos"),
            audio: cfg!(any(target_os = "macos", target_os = "windows")),
            voice: cfg!(any(target_os = "macos", target_os = "windows")),
            login: true,
            pin: std::env::var_os("WAYLAND_DISPLAY").is_none(),
            window_modes: true,
        };
        #[cfg(target_os = "linux")]
        {
            caps.notifications = available("notify-send");
            caps.audio = available("paplay");
            caps.voice = caps.audio;
            caps.tray = std::process::Command::new("busctl")
                .args([
                    "--user",
                    "--quiet",
                    "status",
                    "org.kde.StatusNotifierWatcher",
                ])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status()
                .is_ok_and(|s| s.success());
        }
        Self {
            dir,
            caps,
            tray: Default::default(),
            progress: std::cell::Cell::new((255, 255)),
        }
    }
}
impl Platform for Desktop {
    fn load(&self) -> Option<String> {
        let path = self.dir.join("data.json");
        match std::fs::read_to_string(&path) {
            Ok(s) => {
                if heller_core::Data::decode(&s).is_err() {
                    if let Err(e) = std::fs::rename(&path, self.dir.join("data.json.bak")) {
                        log::error!("Backup: {e}");
                    }
                    None
                } else {
                    Some(s)
                }
            }
            Err(e) => {
                if e.kind() != std::io::ErrorKind::NotFound {
                    log::error!("Read settings: {e}");
                }
                None
            }
        }
    }
    fn save(&self, data: &str) {
        let result = (|| -> std::io::Result<()> {
            std::fs::create_dir_all(&self.dir)?;
            let temp = self.dir.join("data.json.tmp");
            let mut file = std::fs::File::create(&temp)?;
            use std::io::Write;
            file.write_all(data.as_bytes())?;
            file.sync_all()?;
            std::fs::rename(temp, self.dir.join("data.json"))
        })();
        if let Err(e) = result {
            log::error!("Save settings: {e}");
        }
    }
    fn capabilities(&self) -> Caps {
        self.caps
    }
    fn utc_offset_seconds(&self) -> i32 {
        // Resolve local civil time once at startup without an always-running clock thread.
        #[cfg(unix)]
        {
            if let Ok(out) = std::process::Command::new("date").arg("+%z").output() {
                let s = String::from_utf8_lossy(&out.stdout);
                let s = s.trim();
                if s.len() == 5 {
                    if let (Ok(h), Ok(m)) = (s[1..3].parse::<i32>(), s[3..5].parse::<i32>()) {
                        return (h * 3600 + m * 60) * if s.starts_with('-') { -1 } else { 1 };
                    }
                }
            }
        }
        0
    }
    fn login(&self, enabled: bool) {
        let result = (|| -> Result<(), Box<dyn std::error::Error>> {
            let exe = std::env::current_exe()?;
            let launcher = auto_launch::AutoLaunchBuilder::new()
                .set_app_name("HellerTimer")
                .set_app_path(exe.to_str().ok_or("Invalid executable path")?)
                .set_args(&["--hidden"])
                .build()?;
            if enabled {
                launcher.enable()?;
            } else {
                launcher.disable()?;
            }
            Ok(())
        })();
        if let Err(e) = result {
            log::error!("Autostart: {e}");
        }
    }
    fn effect(&self, e: heller_core::Effect, s: &heller_core::Settings, message: &str) {
        let text = message.to_owned();
        let notify = s.notification > 0;
        let audio = s.sound_enabled;
        let voice = s.voice_assistance;
        let dir = self.dir.clone();
        std::thread::spawn(move || {
            #[cfg(target_os = "linux")]
            if notify {
                run(
                    "notify-send",
                    &["--app-name=HellerTimer", "HellerTimer", &text],
                );
            }
            #[cfg(target_os = "macos")]
            if notify {
                run(
                    "osascript",
                    &[
                        "-e",
                        &format!("display notification \"{text}\" with title \"HellerTimer\""),
                    ],
                );
            }
            if audio {
                play_wav(
                    &dir,
                    "bell.wav",
                    include_bytes!("../../../assets/sounds/bell.wav"),
                );
            }
            // parity: the original Pomatez voice recordings are English in both UI languages.
            if voice {
                use heller_core::{Effect, Phase};
                let (name, bytes): (&str, &[u8]) = match e {
                    Effect::Warning(Phase::Focus) => (
                        "sixty-seconds-left.wav",
                        include_bytes!("../../../assets/sounds/sixty-seconds-left.wav"),
                    ),
                    Effect::Warning(_) => (
                        "thirty-seconds-left.wav",
                        include_bytes!("../../../assets/sounds/thirty-seconds-left.wav"),
                    ),
                    Effect::Transition(Phase::Focus) => (
                        "break-finished.wav",
                        include_bytes!("../../../assets/sounds/break-finished.wav"),
                    ),
                    Effect::Transition(Phase::Special(_)) => (
                        "special-break-started.wav",
                        include_bytes!("../../../assets/sounds/special-break-started.wav"),
                    ),
                    _ => (
                        "focus-finished.wav",
                        include_bytes!("../../../assets/sounds/focus-finished.wav"),
                    ),
                };
                play_wav(&dir, name, bytes);
            }
            let _ = (notify, voice, text);
        });
    }
    fn state_changed(&self, core: &heller_core::Core) {
        let phase = match core.phase {
            heller_core::Phase::Focus => 0,
            heller_core::Phase::ShortBreak => 1,
            _ => 2,
        };
        let percent = core.percent(std::time::SystemTime::now());
        if self.progress.replace((phase, percent)) == (phase, percent) {
            return;
        }
        if let Some(tray) = self.tray.borrow().as_ref() {
            let mut pixels = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::new(32, 32);
            for (i, p) in pixels.make_mut_slice().iter_mut().enumerate() {
                let x = (i % 32) as f32 - 15.5;
                let y = (i / 32) as f32 - 15.5;
                let radius = (x * x + y * y).sqrt();
                let angle = (x.atan2(-y) + std::f32::consts::TAU) % std::f32::consts::TAU;
                let active = angle / std::f32::consts::TAU <= f32::from(percent) / 100.0;
                let color = if active {
                    match phase {
                        0 => (0, 152, 247),
                        1 => (7, 181, 131),
                        _ => (212, 141, 10),
                    }
                } else {
                    (80, 90, 100)
                };
                *p = slint::Rgba8Pixel::new(
                    color.0,
                    color.1,
                    color.2,
                    if (11.0..15.0).contains(&radius) {
                        255
                    } else {
                        0
                    },
                );
            }
            tray.set_progress_icon(slint::Image::from_rgba8(pixels));
        }
    }
    fn open_url(&self, url: &str) {
        #[cfg(target_os = "linux")]
        let result = std::process::Command::new("xdg-open").arg(url).spawn();
        #[cfg(target_os = "macos")]
        let result = std::process::Command::new("open").arg(url).spawn();
        #[cfg(target_os = "windows")]
        let result = std::process::Command::new("rundll32")
            .args(["url.dll,FileProtocolHandler", url])
            .spawn();
        if let Err(e) = result {
            log::error!("Open URL: {e}");
        }
    }
}
fn play_wav(dir: &std::path::Path, name: &str, bytes: &[u8]) {
    // Native playback is synchronous on a short-lived effect thread; no audio device stays open.
    #[cfg(target_os = "windows")]
    {
        #[link(name = "winmm")]
        extern "system" {
            fn PlaySoundW(sound: *const u16, module: *mut std::ffi::c_void, flags: u32) -> i32;
        }
        let _ = (dir, name);
        if unsafe { PlaySoundW(bytes.as_ptr().cast(), std::ptr::null_mut(), 0x0004 | 0x0002) } == 0
        {
            log::warn!("Windows sound playback failed");
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let path = dir.join(name);
        if !path.exists() {
            if let Err(e) = std::fs::create_dir_all(dir).and_then(|_| std::fs::write(&path, bytes))
            {
                log::error!("Audio cache: {e}");
                return;
            }
        }
        #[cfg(target_os = "linux")]
        run("paplay", &[path.to_string_lossy().as_ref()]);
        #[cfg(target_os = "macos")]
        run("afplay", &[path.to_string_lossy().as_ref()]);
    }
}
#[cfg(target_os = "linux")]
fn available(command: &str) -> bool {
    std::env::var_os("PATH")
        .is_some_and(|paths| std::env::split_paths(&paths).any(|p| p.join(command).is_file()))
}
fn newer_version(tag: &str, current: &str) -> bool {
    fn version(s: &str) -> Option<(u64, u64, u64)> {
        let mut parts = s.trim_start_matches('v').split('.');
        let a = parts.next()?.parse().ok()?;
        let b = parts.next()?.parse().ok()?;
        let c = parts.next()?.parse().ok()?;
        if parts.next().is_some() {
            return None;
        }
        Some((a, b, c))
    }
    match (version(tag), version(current)) {
        (Some(a), Some(b)) => a > b,
        _ => false,
    }
}

fn run(command: &str, args: &[&str]) {
    match std::process::Command::new(command).args(args).status() {
        Ok(s) if s.success() => {}
        Ok(s) => log::warn!("{command}: {s}"),
        Err(e) => log::warn!("{command}: {e}"),
    }
}
struct Logger;
impl log::Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }
    fn log(&self, r: &log::Record) {
        eprintln!("{}: {}", r.level(), r.args());
    }
    fn flush(&self) {}
}
static LOGGER: Logger = Logger;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = log::set_logger(&LOGGER);
    log::set_max_level(log::LevelFilter::Warn);
    let started = std::time::Instant::now();
    let args: Vec<_> = std::env::args().collect();
    if args.iter().any(|s| s == "--help") {
        println!("HellerTimer\n--start  --mini  --renderer software|femtovg  --no-animation\n--bench-first-frame");
        return Ok(());
    }
    let renderer = args
        .windows(2)
        .find(|a| a[0] == "--renderer")
        .map_or("software", |a| a[1].as_str());
    if !matches!(renderer, "software" | "femtovg") {
        return Err("Unknown renderer".into());
    }
    slint::BackendSelector::new()
        .backend_name("winit".into())
        .renderer_name(renderer.into())
        .select()?;
    let instance = single_instance::SingleInstance::new("io.github.hlophlopgaming.HellerTimer")?;
    if !instance.is_single() {
        let _ = std::net::TcpStream::connect((std::net::Ipv4Addr::LOCALHOST, 43829));
        return Ok(());
    }
    let platform = Rc::new(Desktop::new());
    let app = App::new(platform.clone())?;
    match std::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 43829)) {
        Ok(listener) => {
            let weak = app.window.as_weak();
            std::thread::spawn(move || {
                for stream in listener.incoming() {
                    if stream.is_err() {
                        break;
                    }
                    let _ = weak.upgrade_in_event_loop(|w| w.invoke_external_command(0));
                }
            });
        }
        Err(e) => log::warn!("Second-instance activation: {e}"),
    }
    let hotkeys = if std::env::var_os("WAYLAND_DISPLAY").is_none() {
        match global_hotkey::GlobalHotKeyManager::new() {
            Ok(manager) => {
                use global_hotkey::hotkey::{Code, HotKey, Modifiers};
                let bindings = [
                    HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyP),
                    HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyN),
                    HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyM),
                ];
                for key in &bindings {
                    if let Err(e) = manager.register(*key) {
                        log::warn!("Global shortcut: {e}");
                    }
                }
                let weak = app.window.as_weak();
                global_hotkey::GlobalHotKeyEvent::set_event_handler(Some(
                    move |event: global_hotkey::GlobalHotKeyEvent| {
                        if event.state == global_hotkey::HotKeyState::Pressed {
                            if let Some(i) = bindings.iter().position(|k| k.id() == event.id) {
                                let _ = weak.upgrade_in_event_loop(move |w| {
                                    w.invoke_external_command(i as i32 + 1)
                                });
                            }
                        }
                    },
                ));
                Some(manager)
            }
            Err(e) => {
                log::warn!("Global shortcuts: {e}");
                None
            }
        }
    } else {
        None
    };
    let _hotkeys = hotkeys;
    let tray = Tray::new()?;
    let weak = Rc::downgrade(&app);
    tray.on_action(move |n| {
        if let Some(a) = weak.upgrade() {
            match n {
                0 => a.set_visible(!a.visible.get()),
                1 => {
                    a.toggle_mini();
                    a.set_visible(true);
                }
                2 => a.dispatch(heller_core::Input::Toggle),
                3 => a.dispatch(heller_core::Input::Skip),
                4 => {
                    a.window.set_page(3);
                    a.set_visible(true);
                }
                _ => {
                    a.flush();
                    let _ = slint::quit_event_loop();
                }
            }
        }
    });
    *platform.tray.borrow_mut() = Some(tray);
    app.core.borrow_mut().tray_progress = platform.capabilities().tray;
    use slint::winit_030::WinitWindowAccessor;
    let weak = app.window.as_weak();
    app.window
        .window()
        .on_winit_window_event(move |window, event| {
            use slint::winit_030::{winit::event::WindowEvent, EventResult};
            let command = match event {
                WindowEvent::Occluded(true) => Some(6),
                WindowEvent::Occluded(false) | WindowEvent::Focused(true) => Some(5),
                WindowEvent::Resized(_) => {
                    if window
                        .with_winit_window(|w| w.is_minimized() == Some(true))
                        .unwrap_or(false)
                    {
                        Some(4)
                    } else {
                        None
                    }
                }
                _ => None,
            };
            if let Some(n) = command {
                let _ = weak.upgrade_in_event_loop(move |w| w.invoke_external_command(n));
            }
            EventResult::Propagate
        });
    if app.core.borrow().settings.check_updates {
        let weak = app.window.as_weak();
        slint::Timer::single_shot(std::time::Duration::from_secs(30), move || {
            std::thread::spawn(move || {
                let result = std::process::Command::new("curl")
                    .args([
                        "--fail",
                        "--silent",
                        "--max-time",
                        "5",
                        "-H",
                        "User-Agent: HellerTimer",
                        "https://api.github.com/repos/hlophlopgaming/HellerTimer/releases/latest",
                    ])
                    .output();
                if let Ok(output) = result {
                    if output.status.success() {
                        if let Ok(data) =
                            serde_json::from_slice::<serde_json::Value>(&output.stdout)
                        {
                            if let Some(tag) = data["tag_name"].as_str() {
                                if newer_version(tag, env!("CARGO_PKG_VERSION")) {
                                    let _ = weak
                                        .upgrade_in_event_loop(|w| w.set_update_available(true));
                                }
                            }
                        }
                    }
                }
            });
        });
    }
    if args.iter().any(|s| s == "--animation") {
        app.core.borrow_mut().settings.progress_animation = true;
    }
    if args.iter().any(|s| s == "--no-animation") {
        app.core.borrow_mut().settings.progress_animation = false;
    }
    if args.iter().any(|s| s == "--bench-first-frame") {
        let once = std::cell::Cell::new(false);
        if let Err(e) = app.window.window().set_rendering_notifier(move |state, _| {
            if matches!(state, slint::RenderingState::AfterRendering) && !once.replace(true) {
                println!(
                    "first_frame_ms={:.3}",
                    started.elapsed().as_secs_f64() * 1000.0
                );
            }
        }) {
            log::warn!("First-frame instrumentation unavailable: {e}");
        }
    }
    if args.iter().any(|s| s == "--mini") {
        app.toggle_mini();
    }
    if args.iter().any(|s| s == "--start") {
        app.dispatch(heller_core::Input::Start);
    }
    if args.iter().any(|s| s == "--hidden") && platform.capabilities().tray {
        app.set_visible(false);
    } else {
        app.set_visible(true);
    }
    slint::run_event_loop_until_quit()?;
    app.flush();
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn versions() {
        assert!(super::newer_version("v0.2.0", "0.1.0"));
        assert!(!super::newer_version("v0.1.0", "0.1.0"));
        assert!(!super::newer_version("v0.2.0-beta", "0.1.0"));
    }
}
