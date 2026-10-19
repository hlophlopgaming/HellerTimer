//! Pure deadline-based timer and versioned user data. No platform or IO dependencies.
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub focus_minutes: u16,
    pub short_break_minutes: u16,
    pub long_break_minutes: u16,
    pub sessions_per_round: u8,
    pub special_breaks: [Option<SpecialBreak>; 4],
}
impl Default for Config {
    fn default() -> Self {
        Self {
            focus_minutes: 25,
            short_break_minutes: 5,
            long_break_minutes: 15,
            sessions_per_round: 4,
            special_breaks: [None; 4],
        }
    }
}
impl Config {
    pub fn validate(&mut self) {
        self.focus_minutes = self.focus_minutes.clamp(1, 120);
        self.short_break_minutes = self.short_break_minutes.clamp(1, 60);
        self.long_break_minutes = self.long_break_minutes.clamp(1, 120);
        self.sessions_per_round = self.sessions_per_round.clamp(1, 12);
        for slot in &mut self.special_breaks {
            if let Some(s) = slot {
                if s.minute_of_day >= 1440 {
                    *slot = None;
                } else {
                    s.duration_minutes = s.duration_minutes.clamp(1, 120);
                }
            }
        }
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SpecialBreak {
    pub minute_of_day: u16,
    pub duration_minutes: u16,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub strict_mode: bool,
    pub auto_start_work: bool,
    pub progress_animation: bool,
    pub sound_enabled: bool,
    pub voice_assistance: bool,
    pub notification: u8,
    pub dark: bool,
    pub language: String,
    pub always_on_top: bool,
    pub fullscreen_breaks: bool,
    pub close_to_tray: bool,
    pub open_at_login: bool,
    pub check_updates: bool,
    pub native_titlebar: bool,
    pub minimize_to_tray: bool,
    pub mini_window_pos: Option<(i32, i32)>,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            strict_mode: false,
            auto_start_work: false,
            progress_animation: false,
            sound_enabled: true,
            voice_assistance: false,
            notification: 1,
            dark: true,
            language: "en".into(),
            always_on_top: false,
            fullscreen_breaks: false,
            close_to_tray: false,
            open_at_login: false,
            check_updates: true,
            native_titlebar: true,
            minimize_to_tray: false,
            mini_window_pos: None,
        }
    }
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub description: String,
    pub done: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskList {
    pub title: String,
    pub tasks: Vec<Task>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Data {
    pub version: u32,
    pub config: Config,
    pub settings: Settings,
    pub lists: Vec<TaskList>,
}
impl Default for Data {
    fn default() -> Self {
        Self {
            version: 1,
            config: Config::default(),
            settings: Settings::default(),
            lists: vec![TaskList {
                title: "Inbox".into(),
                tasks: vec![],
            }],
        }
    }
}
impl Data {
    pub fn decode(json: &str) -> Result<Self, String> {
        let mut data: Self = serde_json::from_str(json).map_err(|e| e.to_string())?;
        if data.version != 1 {
            return Err("Unsupported data version".into());
        }
        data.config.validate();
        data.settings.notification = data.settings.notification.min(2);
        if data.lists.is_empty() {
            data.lists = Self::default().lists;
        }
        Ok(data)
    }
    pub fn move_task(&mut self, from: usize, index: usize, to: usize, position: usize) {
        if from >= self.lists.len()
            || to >= self.lists.len()
            || index >= self.lists[from].tasks.len()
        {
            return;
        }
        let task = self.lists[from].tasks.remove(index);
        let position = position.min(self.lists[to].tasks.len());
        self.lists[to].tasks.insert(position, task);
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Focus,
    ShortBreak,
    LongBreak,
    Special(usize),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunState {
    Idle,
    Running { deadline: SystemTime },
    Paused { remaining: Duration },
}
#[derive(Clone, Copy, Debug)]
pub enum Input {
    Start,
    Pause,
    Resume,
    Toggle,
    Skip,
    Reset,
    ResetAll,
    Tick,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Effect {
    Transition(Phase),
    Warning(Phase),
}
#[derive(Debug)]
pub struct Core {
    pub config: Config,
    pub settings: Settings,
    pub phase: Phase,
    pub state: RunState,
    pub round: u8,
    pub visible: bool,
    pub tray_progress: bool,
    /// Local UTC offset supplied by the platform; core never reads OS time zones.
    pub utc_offset_seconds: i32,
    special_days: [Option<u64>; 4],
    warned: bool,
    last_checked: Option<SystemTime>,
}
impl Core {
    pub fn new(mut config: Config, settings: Settings) -> Self {
        config.validate();
        Self {
            config,
            settings,
            phase: Phase::Focus,
            state: RunState::Idle,
            round: 1,
            visible: true,
            tray_progress: false,
            utc_offset_seconds: 0,
            special_days: [None; 4],
            warned: false,
            last_checked: None,
        }
    }
    pub fn duration(&self) -> Duration {
        Duration::from_secs(
            u64::from(match self.phase {
                Phase::Focus => self.config.focus_minutes,
                Phase::ShortBreak => self.config.short_break_minutes,
                Phase::LongBreak => self.config.long_break_minutes,
                Phase::Special(i) => self
                    .config
                    .special_breaks
                    .get(i)
                    .and_then(|s| *s)
                    .map_or(5, |s| s.duration_minutes),
            }) * 60,
        )
    }
    pub fn remaining(&self, now: SystemTime) -> Duration {
        match self.state {
            RunState::Idle => self.duration(),
            RunState::Paused { remaining } => remaining,
            RunState::Running { deadline } => deadline.duration_since(now).unwrap_or_default(),
        }
    }
    pub fn seconds(&self, now: SystemTime) -> u64 {
        let d = self.remaining(now);
        d.as_secs() + u64::from(d.subsec_nanos() > 0)
    }
    pub fn percent(&self, now: SystemTime) -> u8 {
        (100.0 * (1.0 - self.remaining(now).as_secs_f64() / self.duration().as_secs_f64()))
            .clamp(0.0, 100.0) as u8
    }
    fn advance(&mut self, at: SystemTime) {
        self.phase = match self.phase {
            Phase::Focus if self.round >= self.config.sessions_per_round => {
                self.round = 1;
                Phase::LongBreak
            }
            Phase::Focus => {
                self.round += 1;
                Phase::ShortBreak
            }
            _ => Phase::Focus,
        };
        self.warned = false;
        self.state = if self.phase != Phase::Focus || self.settings.auto_start_work {
            RunState::Running {
                deadline: at + self.duration(),
            }
        } else {
            RunState::Idle
        };
    }
    fn special_next(&self, now: SystemTime) -> Option<(SystemTime, usize, u64)> {
        let seconds = now.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() as i64
            + i64::from(self.utc_offset_seconds);
        let day = seconds.div_euclid(86400).max(0) as u64;
        self.config
            .special_breaks
            .iter()
            .enumerate()
            .filter_map(|(i, s)| {
                let s = s.as_ref()?;
                let mut target_day = day;
                let mut target = day * 86400 + u64::from(s.minute_of_day) * 60;
                if target < seconds.max(0) as u64 || self.special_days[i] == Some(day) {
                    target += 86400;
                    target_day += 1;
                }
                let utc = (target as i64 - i64::from(self.utc_offset_seconds)).max(0) as u64;
                Some((UNIX_EPOCH + Duration::from_secs(utc), i, target_day))
            })
            .min_by_key(|s| s.0)
    }
    pub fn next_wakeup(&self, now: SystemTime) -> Option<SystemTime> {
        let mut next = self.special_next(now).map(|s| s.0);
        if let RunState::Running { deadline } = self.state {
            let mut wake = deadline;
            if self.visible {
                wake = wake.min(now + Duration::from_secs(1));
            } else if self.tray_progress {
                let step = self.duration().as_secs_f64() / 100.0;
                wake = wake.min(now + Duration::from_secs_f64(step.max(0.01)));
            }
            if self.settings.notification == 2 && !self.warned {
                let lead = if self.phase == Phase::Focus { 60 } else { 30 };
                let warning = deadline
                    .checked_sub(Duration::from_secs(lead))
                    .unwrap_or(now);
                wake = wake.min(warning.max(now));
            }
            next = Some(next.map_or(wake, |n| n.min(wake)));
        }
        next
    }
    pub fn handle(&mut self, input: Input, now: SystemTime) -> Vec<Effect> {
        let mut effects = Vec::new();
        // Replay scheduled breaks and phase deadlines in chronological order after sleep.
        let mut cursor = self.last_checked.unwrap_or(now);
        if cursor > now {
            cursor = now;
        }
        // An older daily interruption is superseded by the most recent day's schedule.
        if now.duration_since(cursor).unwrap_or_default() > Duration::from_secs(86400)
            && self.config.special_breaks.iter().any(Option::is_some)
        {
            cursor = now - Duration::from_secs(86400);
        }
        while let Some((at, index, day)) = self.special_next(cursor) {
            if at > now {
                break;
            }
            self.catch_up(at, &mut effects);
            self.special_days[index] = Some(day);
            self.phase = Phase::Special(index);
            self.warned = false;
            self.state = RunState::Running {
                deadline: at + self.duration(),
            };
            effects.clear();
            effects.push(Effect::Transition(self.phase));
            cursor = at;
        }
        self.catch_up(now, &mut effects);
        self.last_checked = Some(now);
        let locked = self.settings.strict_mode && self.state != RunState::Idle;
        match input {
            Input::Start | Input::Resume => {
                if !matches!(self.state, RunState::Running { .. }) {
                    self.state = RunState::Running {
                        deadline: now + self.remaining(now),
                    };
                }
            }
            Input::Toggle => {
                if let RunState::Running { .. } = self.state {
                    if !locked {
                        self.state = RunState::Paused {
                            remaining: self.remaining(now),
                        };
                    }
                } else {
                    self.state = RunState::Running {
                        deadline: now + self.remaining(now),
                    };
                }
            }
            Input::Pause if !locked && matches!(self.state, RunState::Running { .. }) => {
                self.state = RunState::Paused {
                    remaining: self.remaining(now),
                }
            }
            Input::Skip if !locked => {
                self.advance(now);
                effects.push(Effect::Transition(self.phase));
            }
            Input::Reset if !locked => {
                self.state = RunState::Idle;
                self.warned = false;
            }
            Input::ResetAll if !locked => {
                self.phase = Phase::Focus;
                self.round = 1;
                self.state = RunState::Idle;
                self.warned = false;
            }
            _ => {}
        }
        if matches!(self.state, RunState::Running { .. })
            && self.settings.notification == 2
            && !self.warned
            && self.seconds(now) <= if self.phase == Phase::Focus { 60 } else { 30 }
        {
            self.warned = true;
            effects.push(Effect::Warning(self.phase));
        }
        effects
    }
    fn catch_up(&mut self, now: SystemTime, effects: &mut Vec<Effect>) {
        // Skip complete identical cycles arithmetically, even after years of suspension.
        if self.settings.auto_start_work && !matches!(self.phase, Phase::Special(_)) {
            if let RunState::Running { deadline } = self.state {
                let sessions = u64::from(self.config.sessions_per_round);
                let cycle = 60
                    * (sessions * u64::from(self.config.focus_minutes)
                        + (sessions - 1) * u64::from(self.config.short_break_minutes)
                        + u64::from(self.config.long_break_minutes));
                let cycles = now.duration_since(deadline).unwrap_or_default().as_secs() / cycle;
                if cycles > 0 {
                    self.state = RunState::Running {
                        deadline: deadline + Duration::from_secs(cycles * cycle),
                    };
                    effects.clear();
                    effects.push(Effect::Transition(self.phase));
                }
            }
        }
        while let RunState::Running { deadline } = self.state {
            if deadline > now {
                break;
            }
            self.advance(deadline);
            effects.clear();
            effects.push(Effect::Transition(self.phase));
        }
    }
    pub fn apply_config(&mut self, mut config: Config) {
        config.validate();
        self.config = config;
        self.round = self.round.min(self.config.sessions_per_round);
        // parity: configuration edits affect the next phase; the active deadline stays stable.
    }
}
