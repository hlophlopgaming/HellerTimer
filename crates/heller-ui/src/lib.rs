use heller_core::{Core, Data, Effect, Input, Phase, RunState, SpecialBreak, Task, TaskList};
use slint::{ComponentHandle, ModelRc, VecModel};
use std::{
    cell::{Cell, RefCell},
    fmt::Write,
    rc::Rc,
    time::{Duration, SystemTime},
};
slint::include_modules!();

#[derive(Clone, Copy, Default)]
pub struct Caps {
    pub tray: bool,
    pub updates: bool,
    pub notifications: bool,
    pub audio: bool,
    pub voice: bool,
    pub login: bool,
    pub pin: bool,
}
pub trait Platform {
    fn load(&self) -> Option<String> {
        None
    }
    fn save(&self, _data: &str) {}
    fn capabilities(&self) -> Caps {
        Caps::default()
    }
    fn effect(&self, _effect: Effect, _settings: &heller_core::Settings, _message: &str) {}
    fn state_changed(&self, _core: &Core) {}
    fn open_url(&self, _url: &str) {}
    fn login(&self, _enabled: bool) {}
    fn utc_offset_seconds(&self) -> i32 {
        0
    }
    fn request_notifications(&self) {}
}
pub struct App {
    pub window: MainWindow,
    pub core: RefCell<Core>,
    data: RefCell<Data>,
    platform: Rc<dyn Platform>,
    timer: slint::Timer,
    save_timer: slint::Timer,
    buffer: RefCell<String>,
    pub visible: Cell<bool>,
    list: Cell<usize>,
    language: RefCell<String>,
}
macro_rules! set_changed {
    ($ui:expr, $get:ident, $set:ident, $value:expr) => {{
        let value = $value;
        if $ui.$get() != value {
            $ui.$set(value);
        }
    }};
}
impl App {
    pub fn new(platform: Rc<dyn Platform>) -> Result<Rc<Self>, slint::PlatformError> {
        let data = platform
            .load()
            .and_then(|s| match Data::decode(&s) {
                Ok(d) => Some(d),
                Err(e) => {
                    log::error!("Invalid saved data: {e}");
                    None
                }
            })
            .unwrap_or_default();
        let mut core = Core::new(data.config.clone(), data.settings.clone());
        core.utc_offset_seconds = platform.utc_offset_seconds();
        let app = Rc::new(Self {
            window: MainWindow::new()?,
            core: RefCell::new(core),
            data: RefCell::new(data),
            platform,
            timer: slint::Timer::default(),
            save_timer: slint::Timer::default(),
            buffer: RefCell::new(String::with_capacity(16)),
            visible: Cell::new(true),
            list: Cell::new(0),
            language: RefCell::new(String::new()),
        });
        let caps = app.platform.capabilities();
        app.window.set_cap_updates(caps.updates);
        app.window.set_cap_tray(caps.tray);
        app.window.set_cap_notify(caps.notifications);
        app.window.set_cap_audio(caps.audio);
        app.window.set_cap_voice(caps.voice);
        app.window.set_cap_login(caps.login);
        app.window.set_cap_pin(caps.pin);
        let weak = Rc::downgrade(&app);
        app.window.on_external_command(move |n| {
            if let Some(a) = weak.upgrade() {
                match n {
                    4 => {
                        if a.core.borrow().settings.minimize_to_tray
                            && a.platform.capabilities().tray
                        {
                            a.set_visible(false);
                        } else {
                            a.core.borrow_mut().visible = false;
                            a.dispatch(Input::Tick);
                        }
                    }
                    5 => {
                        a.core.borrow_mut().visible = true;
                        a.dispatch(Input::Tick);
                    }
                    6 => {
                        a.core.borrow_mut().visible = false;
                        a.dispatch(Input::Tick);
                    }
                    1 => a.dispatch(Input::Toggle),
                    2 => a.dispatch(Input::Skip),
                    3 => {
                        a.window.set_mini(!a.window.get_mini());
                        a.set_visible(true);
                    }
                    _ => a.set_visible(true),
                }
            }
        });
        let weak = Rc::downgrade(&app);
        app.window.on_command(move |n| {
            if let Some(a) = weak.upgrade() {
                a.dispatch(match n {
                    1 => Input::Reset,
                    2 => Input::Skip,
                    _ => Input::Toggle,
                });
            }
        });
        let weak = Rc::downgrade(&app);
        app.window.on_setting(move |key| {
            if let Some(a) = weak.upgrade() {
                a.setting(key.as_str());
            }
        });
        let weak = Rc::downgrade(&app);
        app.window.on_configure(move |field, delta| {
            if let Some(a) = weak.upgrade() {
                let mut core = a.core.borrow_mut();
                if core.settings.strict_mode && core.state != RunState::Idle {
                    return;
                }
                let mut config = core.config.clone();
                let adjust = |v: u16| (i32::from(v) + delta).clamp(1, 120) as u16;
                match field {
                    0 => config.focus_minutes = adjust(config.focus_minutes),
                    1 => config.short_break_minutes = adjust(config.short_break_minutes),
                    2 => config.long_break_minutes = adjust(config.long_break_minutes),
                    3 => {
                        config.sessions_per_round =
                            (i32::from(config.sessions_per_round) + delta).clamp(1, 12) as u8
                    }
                    _ => config = Default::default(),
                }
                core.apply_config(config);
                drop(core);
                a.specials();
                a.persist();
                a.refresh();
                a.arm();
            }
        });
        let weak = Rc::downgrade(&app);
        app.window.on_special(move |i, time, minutes| {
            if let Some(a) = weak.upgrade() {
                let mut core = a.core.borrow_mut();
                if core.settings.strict_mode && core.state != RunState::Idle {
                    return;
                }
                if let Some(slot) = core.config.special_breaks.get_mut(i as usize) {
                    if time.is_empty() {
                        *slot = None;
                    } else {
                        let parsed = time
                            .split_once(':')
                            .and_then(|(h, m)| {
                                Some((h.parse::<u16>().ok()?, m.parse::<u16>().ok()?))
                            })
                            .filter(|(h, m)| *h < 24 && *m < 60);
                        if let (Some((h, m)), Ok(d)) = (parsed, minutes.parse::<u16>()) {
                            if (1..=120).contains(&d) {
                                *slot = Some(SpecialBreak {
                                    minute_of_day: h * 60 + m,
                                    duration_minutes: d,
                                });
                            } else {
                                a.window.set_config_error(1);
                                return;
                            }
                        } else {
                            a.window.set_config_error(1);
                            return;
                        }
                    }
                }
                drop(core);
                a.window.set_config_error(0);
                a.specials();
                a.persist();
                a.arm();
            }
        });
        let weak = Rc::downgrade(&app);
        app.window.on_add_task(move |text| {
            if let Some(a) = weak.upgrade() {
                if !text.trim().is_empty() {
                    a.data.borrow_mut().lists[a.list.get()].tasks.push(Task {
                        title: text.trim().into(),
                        ..Task::default()
                    });
                    a.tasks();
                    a.persist();
                }
            }
        });
        let weak = Rc::downgrade(&app);
        app.window.on_list_action(move |action, text| {
            if let Some(a) = weak.upgrade() {
                let mut d = a.data.borrow_mut();
                match action {
                    2 if !text.trim().is_empty() => {
                        d.lists.push(TaskList {
                            title: text.trim().into(),
                            tasks: vec![],
                        });
                        a.list.set(d.lists.len() - 1);
                    }
                    3 if !text.trim().is_empty() => {
                        d.lists[a.list.get()].title = text.trim().into()
                    }
                    4 if d.lists.len() > 1 && d.lists[a.list.get()].tasks.is_empty() => {
                        d.lists.remove(a.list.get());
                        a.list.set(0);
                    }
                    -1 => a
                        .list
                        .set((a.list.get() + d.lists.len() - 1) % d.lists.len()),
                    1 => a.list.set((a.list.get() + 1) % d.lists.len()),
                    _ => {}
                }
                drop(d);
                a.tasks();
                a.persist();
            }
        });
        let weak = Rc::downgrade(&app);
        app.window.on_task_action(move |i, action, text| {
            if let Some(a) = weak.upgrade() {
                let mut d = a.data.borrow_mut();
                let list = a.list.get();
                let i = i as usize;
                if i >= d.lists[list].tasks.len() {
                    return;
                }
                match action {
                    0 => {
                        let t = &mut d.lists[list].tasks[i];
                        t.done = !t.done;
                    }
                    1 if !text.trim().is_empty() => {
                        d.lists[list].tasks[i].title = text.trim().into()
                    }
                    2 => {
                        d.lists[list].tasks.remove(i);
                    }
                    3 if i > 0 => d.move_task(list, i, list, i - 1),
                    4 => d.move_task(list, i, list, i + 1),
                    5 => {
                        let next = (list + 1) % d.lists.len();
                        let pos = d.lists[next].tasks.len();
                        d.move_task(list, i, next, pos);
                    }
                    6 => d.lists[list].tasks[i].description = text.into(),
                    _ => {}
                }
                drop(d);
                a.tasks();
                a.persist();
            }
        });
        let weak = Rc::downgrade(&app);
        app.window.on_open_link(move || {
            if let Some(a) = weak.upgrade() {
                a.platform
                    .open_url("https://github.com/hlophlopgaming/HellerTimer");
            }
        });
        let weak = Rc::downgrade(&app);
        app.window.on_close_window(move || {
            if let Some(a) = weak.upgrade() {
                if a.core.borrow().settings.close_to_tray && a.platform.capabilities().tray {
                    a.set_visible(false);
                } else {
                    a.flush();
                    let _ = slint::quit_event_loop();
                }
            }
        });
        let weak = Rc::downgrade(&app);
        app.window.on_hide_window(move || {
            if let Some(a) = weak.upgrade() {
                a.set_visible(false);
            }
        });
        let weak = Rc::downgrade(&app);
        app.window.window().on_close_requested(move || {
            if let Some(a) = weak.upgrade() {
                if a.core.borrow().settings.close_to_tray && a.platform.capabilities().tray {
                    a.set_visible(false);
                    return slint::CloseRequestResponse::KeepWindowShown;
                }
                a.flush();
            }
            let _ = slint::quit_event_loop();
            slint::CloseRequestResponse::HideWindow
        });
        app.tasks();
        app.specials();
        app.refresh();
        app.arm();
        Ok(app)
    }
    pub fn dispatch(self: &Rc<Self>, input: Input) {
        let effects = self.core.borrow_mut().handle(input, SystemTime::now());
        for e in effects {
            let message = match e {
                Effect::Warning(_) => self.window.get_message_warning(),
                Effect::Transition(Phase::Focus) => self.window.get_message_focus(),
                _ => self.window.get_message_break(),
            };
            self.platform
                .effect(e, &self.core.borrow().settings, message.as_str());
        }
        self.refresh();
        self.arm();
    }
    pub fn set_visible(self: &Rc<Self>, visible: bool) {
        self.visible.set(visible);
        self.core.borrow_mut().visible = visible;
        let result = if visible {
            self.window.show()
        } else {
            self.window.hide()
        };
        if let Err(e) = result {
            log::error!("Window visibility: {e}");
        }
        self.dispatch(Input::Tick);
    }
    fn setting(self: &Rc<Self>, key: &str) {
        if key == "mini" {
            if self.window.get_mini() {
                let pos = self.window.window().position();
                self.core.borrow_mut().settings.mini_window_pos = Some((pos.x, pos.y));
            } else if let Some((x, y)) = self.core.borrow().settings.mini_window_pos {
                self.window
                    .window()
                    .set_position(slint::PhysicalPosition::new(x, y));
            }
            self.window.set_mini(!self.window.get_mini());
            self.persist();
            return;
        }
        let mut core = self.core.borrow_mut();
        let s = &mut core.settings;
        match key {
            "updates" => s.check_updates = !s.check_updates,
            "titlebar" => s.native_titlebar = !s.native_titlebar,
            "minimize" => s.minimize_to_tray = !s.minimize_to_tray,
            "dark" => s.dark = !s.dark,
            "strict" => {
                if !s.strict_mode || core.state == RunState::Idle {
                    core.settings.strict_mode = !core.settings.strict_mode;
                }
            }
            "animation" => s.progress_animation = !s.progress_animation,
            "auto" => s.auto_start_work = !s.auto_start_work,
            "sound" => s.sound_enabled = !s.sound_enabled,
            "voice" => s.voice_assistance = !s.voice_assistance,
            "pin" => s.always_on_top = !s.always_on_top,
            "fullscreen" => s.fullscreen_breaks = !s.fullscreen_breaks,
            "tray" => s.close_to_tray = !s.close_to_tray,
            "login" => {
                s.open_at_login = !s.open_at_login;
                self.platform.login(s.open_at_login);
            }
            "notify" => {
                s.notification = (s.notification + 1) % 3;
                self.platform.request_notifications();
            }
            "language" => {
                s.language = if s.language == "en" { "ru" } else { "en" }.into();
            }
            _ => {}
        }
        drop(core);
        self.persist();
        self.refresh();
        self.arm();
    }
    fn specials(&self) {
        let c = self.core.borrow();
        self.window.set_special_breaks(ModelRc::new(VecModel::from(
            c.config
                .special_breaks
                .iter()
                .map(|slot| match slot {
                    Some(s) => SpecialRow {
                        time: format!("{:02}:{:02}", s.minute_of_day / 60, s.minute_of_day % 60)
                            .into(),
                        duration: s.duration_minutes.to_string().into(),
                    },
                    None => SpecialRow::default(),
                })
                .collect::<Vec<_>>(),
        )));
    }
    fn tasks(&self) {
        let d = self.data.borrow();
        let list = &d.lists[self.list.get()];
        self.window.set_list_title(list.title.as_str().into());
        self.window.set_tasks(ModelRc::new(VecModel::from(
            list.tasks
                .iter()
                .map(|t| TaskRow {
                    title: t.title.as_str().into(),
                    description: t.description.as_str().into(),
                    done: t.done,
                })
                .collect::<Vec<_>>(),
        )));
    }
    fn persist(self: &Rc<Self>) {
        {
            let core = self.core.borrow();
            let mut d = self.data.borrow_mut();
            d.config = core.config.clone();
            d.settings = core.settings.clone();
        }
        let weak = Rc::downgrade(self);
        self.save_timer.start(
            slint::TimerMode::SingleShot,
            Duration::from_millis(500),
            move || {
                if let Some(a) = weak.upgrade() {
                    a.flush();
                }
            },
        );
    }
    pub fn flush(&self) {
        match serde_json::to_string(&*self.data.borrow()) {
            Ok(s) => self.platform.save(&s),
            Err(e) => log::error!("Save serialization: {e}"),
        }
    }
    fn refresh(&self) {
        let c = self.core.borrow();
        let w = &self.window;
        let now = SystemTime::now();
        let settings = &c.settings;
        set_changed!(w, get_updates, set_updates, settings.check_updates);
        let mut b = self.buffer.borrow_mut();
        b.clear();
        let seconds = c.seconds(now);
        let _ = write!(&mut *b, "{:02}:{:02}", seconds / 60, seconds % 60);
        if w.get_time_text().as_str() != b.as_str() {
            w.set_time_text(b.as_str().into());
        }
        set_changed!(w, get_progress, set_progress, c.percent(now) as f32 / 100.0);
        set_changed!(
            w,
            get_running,
            set_running,
            matches!(c.state, RunState::Running { .. })
        );
        set_changed!(
            w,
            get_locked,
            set_locked,
            settings.strict_mode && c.state != RunState::Idle
        );
        set_changed!(
            w,
            get_phase,
            set_phase,
            match c.phase {
                Phase::Focus => 0,
                Phase::ShortBreak => 1,
                Phase::LongBreak => 2,
                Phase::Special(_) => 3,
            }
        );
        set_changed!(w, get_round, set_round, i32::from(c.round));
        set_changed!(
            w,
            get_rounds,
            set_rounds,
            i32::from(c.config.sessions_per_round)
        );
        set_changed!(
            w,
            get_focus_minutes,
            set_focus_minutes,
            i32::from(c.config.focus_minutes)
        );
        set_changed!(
            w,
            get_short_minutes,
            set_short_minutes,
            i32::from(c.config.short_break_minutes)
        );
        set_changed!(
            w,
            get_long_minutes,
            set_long_minutes,
            i32::from(c.config.long_break_minutes)
        );
        set_changed!(
            w,
            get_native_titlebar,
            set_native_titlebar,
            settings.native_titlebar
        );
        set_changed!(
            w,
            get_minimize_tray,
            set_minimize_tray,
            settings.minimize_to_tray
        );
        set_changed!(w, get_animation, set_animation, settings.progress_animation);
        set_changed!(w, get_dark, set_dark, settings.dark);
        if w.global::<Theme>().get_dark() != settings.dark {
            w.global::<Theme>().set_dark(settings.dark);
        }
        set_changed!(w, get_strict, set_strict, settings.strict_mode);
        set_changed!(w, get_auto_start, set_auto_start, settings.auto_start_work);
        set_changed!(w, get_sound, set_sound, settings.sound_enabled);
        set_changed!(w, get_voice, set_voice, settings.voice_assistance);
        set_changed!(w, get_pinned, set_pinned, settings.always_on_top);
        set_changed!(
            w,
            get_fullscreen_breaks,
            set_fullscreen_breaks,
            settings.fullscreen_breaks
        );
        set_changed!(w, get_close_tray, set_close_tray, settings.close_to_tray);
        set_changed!(w, get_login, set_login, settings.open_at_login);
        set_changed!(
            w,
            get_notification,
            set_notification,
            i32::from(settings.notification)
        );
        let overlay = settings.fullscreen_breaks
            && c.phase != Phase::Focus
            && matches!(c.state, RunState::Running { .. });
        if w.get_break_overlay() != overlay {
            w.set_break_overlay(overlay);
            w.window().set_fullscreen(overlay);
        }
        if *self.language.borrow() != settings.language {
            if let Err(e) = slint::select_bundled_translation(&settings.language) {
                log::warn!("Translation: {e}");
            }
            self.language.replace(settings.language.clone());
        }
        self.platform.state_changed(&c);
    }
    fn arm(self: &Rc<Self>) {
        self.timer.stop();
        let now = SystemTime::now();
        if let Some(next) = self.core.borrow().next_wakeup(now) {
            let weak = Rc::downgrade(self);
            self.timer.start(
                slint::TimerMode::SingleShot,
                next.duration_since(now)
                    .unwrap_or(Duration::from_millis(1))
                    .max(Duration::from_millis(1)),
                move || {
                    if let Some(a) = weak.upgrade() {
                        a.dispatch(Input::Tick);
                    }
                },
            );
        }
    }
}
