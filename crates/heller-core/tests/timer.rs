use heller_core::*;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
struct FakeClock(SystemTime);
impl FakeClock {
    fn new() -> Self {
        Self(UNIX_EPOCH + Duration::from_secs(86400))
    }
    fn advance(&mut self, secs: u64) {
        self.0 += Duration::from_secs(secs);
    }
}
fn timer() -> Core {
    Core::new(Config::default(), Settings::default())
}
#[test]
fn full_round() {
    let mut t = timer();
    let mut c = FakeClock::new();
    for round in 1..=4 {
        assert_eq!(t.round, round);
        t.handle(Input::Start, c.0);
        c.advance(1500);
        t.handle(Input::Tick, c.0);
        assert_eq!(
            t.phase,
            if round == 4 {
                Phase::LongBreak
            } else {
                Phase::ShortBreak
            }
        );
        c.advance(if round == 4 { 900 } else { 300 });
        t.handle(Input::Tick, c.0);
        assert_eq!(t.phase, Phase::Focus);
        assert_eq!(t.state, RunState::Idle);
    }
    assert_eq!(t.round, 1);
}
#[test]
fn strict_blocks_all_escape_commands() {
    let mut t = timer();
    let c = FakeClock::new();
    t.settings.strict_mode = true;
    t.handle(Input::Start, c.0);
    for input in [
        Input::Pause,
        Input::Toggle,
        Input::Skip,
        Input::Reset,
        Input::ResetAll,
    ] {
        t.handle(input, c.0);
        assert!(matches!(t.state, RunState::Running { .. }));
        assert_eq!(t.phase, Phase::Focus);
    }
}
#[test]
fn sleep_preserves_deadlines_and_only_last_notification() {
    let mut t = timer();
    let mut c = FakeClock::new();
    t.settings.auto_start_work = true;
    t.handle(Input::Start, c.0);
    c.advance(3700);
    let effects = t.handle(Input::Tick, c.0);
    assert_eq!(t.phase, Phase::Focus);
    assert_eq!(t.seconds(c.0), 1400);
    assert_eq!(effects, vec![Effect::Transition(Phase::Focus)]);
}
#[test]
fn pause_resume_preserves_remaining() {
    let mut t = timer();
    let mut c = FakeClock::new();
    t.handle(Input::Start, c.0);
    c.advance(37);
    t.handle(Input::Pause, c.0);
    c.advance(500);
    assert_eq!(t.seconds(c.0), 1463);
    t.handle(Input::Resume, c.0);
    c.advance(3);
    assert_eq!(t.seconds(c.0), 1460);
}
#[test]
fn extra_warning_once_and_no_hidden_second_tick() {
    let mut t = timer();
    let mut c = FakeClock::new();
    t.settings.notification = 2;
    t.visible = false;
    t.handle(Input::Start, c.0);
    assert_eq!(t.next_wakeup(c.0), Some(c.0 + Duration::from_secs(1440)));
    c.advance(1440);
    assert_eq!(
        t.handle(Input::Tick, c.0),
        vec![Effect::Warning(Phase::Focus)]
    );
    assert!(t.handle(Input::Tick, c.0).is_empty());
    c.advance(60);
    t.handle(Input::Tick, c.0);
    c.advance(270);
    assert_eq!(
        t.handle(Input::Tick, c.0),
        vec![Effect::Warning(Phase::ShortBreak)]
    );
}
#[test]
fn special_interrupts_without_changing_round() {
    let mut t = timer();
    let mut c = FakeClock::new();
    t.config.special_breaks[0] = Some(SpecialBreak {
        minute_of_day: 1,
        duration_minutes: 2,
    });
    t.handle(Input::Start, c.0);
    c.advance(60);
    t.handle(Input::Tick, c.0);
    assert_eq!(t.phase, Phase::Special(0));
    c.advance(120);
    t.handle(Input::Tick, c.0);
    assert_eq!(t.phase, Phase::Focus);
    assert_eq!(t.round, 1);
    t.handle(Input::Tick, c.0);
    assert_eq!(t.phase, Phase::Focus);
}
#[test]
fn data_validation_and_task_move() {
    assert!(Data::decode("{\"version\":2}").is_err());
    assert!(Data::decode("bad").is_err());
    let d = Data::decode("{\"version\":1,\"config\":{\"focus_minutes\":0}}").unwrap();
    assert_eq!(d.config.focus_minutes, 1);
    let mut d = Data::default();
    d.lists[0].tasks.push(Task {
        title: "A".into(),
        ..Task::default()
    });
    d.lists.push(TaskList {
        title: "B".into(),
        tasks: vec![],
    });
    d.move_task(0, 0, 1, 99);
    assert_eq!(d.lists[1].tasks[0].title, "A");
    d.move_task(8, 0, 0, 0);
}

#[test]
fn scheduled_break_survives_sleep_past_start_minute() {
    let mut t = timer();
    let mut c = FakeClock::new();
    t.config.special_breaks[0] = Some(SpecialBreak {
        minute_of_day: 1,
        duration_minutes: 5,
    });
    t.handle(Input::Start, c.0);
    c.advance(190);
    t.handle(Input::Tick, c.0);
    assert_eq!(t.phase, Phase::Special(0));
    assert_eq!(t.seconds(c.0), 170);
}
#[test]
fn years_of_sleep_are_bounded_and_correct() {
    let mut t = timer();
    let mut c = FakeClock::new();
    t.settings.auto_start_work = true;
    t.handle(Input::Start, c.0);
    c.advance(7800 * 100000 + 10);
    t.handle(Input::Tick, c.0);
    assert_eq!(t.phase, Phase::Focus);
    assert_eq!(t.round, 1);
    assert_eq!(t.seconds(c.0), 1490);
}
