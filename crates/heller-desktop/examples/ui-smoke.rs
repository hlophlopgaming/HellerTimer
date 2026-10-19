//! Display-backed regression check. No real user settings, tasks, tray or audio are touched.
//! HELLER_SCREENSHOT_DIR receives raw RGBA + dimensions; no image crate is needed.
use heller_core::{ColorTheme, Data, RunState};
use heller_ui::{App, Caps, Platform};
use slint::{ComponentHandle, Model};
use std::{cell::RefCell, path::PathBuf, rc::Rc, time::Duration};

#[derive(Default)]
struct MemoryPlatform {
    saved: RefCell<Option<String>>,
}
impl Platform for MemoryPlatform {
    fn capabilities(&self) -> Caps {
        Caps {
            window_modes: true,
            ..Caps::default()
        }
    }
    fn load(&self) -> Option<String> {
        self.saved.borrow().clone()
    }
    fn save(&self, json: &str) {
        *self.saved.borrow_mut() = Some(json.into());
    }
}
fn capture(app: &App, name: &str) -> Result<(), String> {
    let pixels = app
        .window
        .window()
        .take_snapshot()
        .map_err(|e| e.to_string())?;
    let dir = std::env::var_os("HELLER_SCREENSHOT_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::temp_dir().join("hellertimer-ui-smoke"));
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    std::fs::write(dir.join(format!("{name}.rgba")), pixels.as_bytes())
        .map_err(|e| e.to_string())?;
    std::fs::write(
        dir.join(format!("{name}.json")),
        format!(
            "{{\"width\":{},\"height\":{}}}",
            pixels.width(),
            pixels.height()
        ),
    )
    .map_err(|e| e.to_string())?;
    println!("{name}: {}×{}", pixels.width(), pixels.height());
    Ok(())
}
fn click(app: &App, x: f32, y: f32) {
    use slint::platform::{PointerEventButton, WindowEvent};
    let position = slint::LogicalPosition::new(x, y);
    for event in [
        WindowEvent::PointerMoved { position },
        WindowEvent::PointerPressed {
            position,
            button: PointerEventButton::Left,
        },
        WindowEvent::PointerReleased {
            position,
            button: PointerEventButton::Left,
        },
    ] {
        app.window.window().dispatch_event(event);
    }
}
fn drag_releases_pointer(app: &App) {
    use slint::platform::{PointerEventButton, WindowEvent, WindowEventDispatchResult};
    let window = app.window.window();
    let point = |x, y| slint::LogicalPosition::new(x, y);
    window.dispatch_event(WindowEvent::PointerPressed {
        position: point(40.0, 28.0),
        button: PointerEventButton::Left,
    });
    window.dispatch_event(WindowEvent::PointerMoved {
        position: point(65.0, 28.0),
    });
    // The compositor owns the native drag and can consume its button-release event.
    // Slint must already have relinquished its own grab after handing off the move.
    let result = window
        .dispatch_event_with_result(WindowEvent::PointerMoved {
            position: point(400.0, 200.0),
        })
        .unwrap();
    window.dispatch_event(WindowEvent::PointerReleased {
        position: point(400.0, 200.0),
        button: PointerEventButton::Left,
    });
    assert_eq!(
        result,
        WindowEventDispatchResult::Rejected,
        "drag handle retained the pointer grab after native handoff"
    );
    click(app, 173.0, 31.0);
    assert!(app.window.get_running());
    click(app, 173.0, 31.0);
    assert!(!app.window.get_running());
    window.dispatch_event(WindowEvent::PointerPressed {
        position: point(40.0, 28.0),
        button: PointerEventButton::Left,
    });
    window.dispatch_event(WindowEvent::PointerMoved {
        position: point(65.0, 28.0),
    });
    window.dispatch_event(WindowEvent::PointerReleased {
        position: point(65.0, 28.0),
        button: PointerEventButton::Left,
    });
    assert_eq!(
        window
            .dispatch_event_with_result(WindowEvent::PointerMoved {
                position: point(400.0, 200.0)
            })
            .unwrap(),
        WindowEventDispatchResult::Rejected,
        "pointer stayed captured after button release"
    );
}
fn geometry(app: &App, mini: bool) -> Result<(), String> {
    let size = app
        .window
        .window()
        .size()
        .to_logical(app.window.window().scale_factor());
    let correct = if mini {
        (240.0..=280.0).contains(&size.width) && (60.0..=90.0).contains(&size.height)
    } else {
        (size.width - 480.0).abs() <= 1.0 && (size.height - 640.0).abs() <= 1.0
    };
    if !correct {
        return Err(format!(
            "mini={mini}: wrong window size {:.0}×{:.0}",
            size.width, size.height
        ));
    }
    Ok(())
}
fn step(
    app: Rc<App>,
    platform: Rc<MemoryPlatform>,
    outcome: Rc<RefCell<Result<(), String>>>,
    n: usize,
) {
    slint::Timer::single_shot(Duration::from_millis(200), move || {
        let result = (|| -> Result<(), String> {
            match n {
                0 => {
                    capture(&app, "normal")?;
                    geometry(&app, false)?;
                    click(&app, 435.0, 43.0);
                    assert!(app.window.get_mini());
                }
                1 => {
                    capture(&app, "mini-button")?;
                    geometry(&app, true)?;
                    drag_releases_pointer(&app);
                    click(&app, 227.0, 31.0);
                    assert!(!app.window.get_mini());
                }
                2 => {
                    capture(&app, "restored")?;
                    geometry(&app, false)?;
                    app.window.invoke_external_command(3);
                }
                3 => {
                    geometry(&app, true)?;
                    app.window.invoke_external_command(3);
                }
                4 => {
                    geometry(&app, false)?;
                    app.toggle_mini();
                }
                5 => {
                    geometry(&app, true)?;
                    app.toggle_mini();
                }
                6 => {
                    geometry(&app, false)?;
                    click(&app, 240.0, 566.0);
                    assert!(matches!(app.core.borrow().state, RunState::Running { .. }));
                    click(&app, 240.0, 566.0);
                    assert!(matches!(app.core.borrow().state, RunState::Paused { .. }));
                    app.window.invoke_command(1);
                    assert_eq!(app.core.borrow().state, RunState::Idle);
                    app.window.invoke_setting("strict".into());
                    app.window.invoke_command(0);
                    app.window.invoke_command(2);
                    assert_eq!(app.core.borrow().phase, heller_core::Phase::Focus);
                    assert!(app.window.get_locked());
                    app.core.borrow_mut().settings.strict_mode = false;
                    app.window.invoke_command(1);
                    app.window.invoke_list_action(2, "Smoke test".into());
                    app.window
                        .invoke_add_task("Check the compact window".into());
                    app.window
                        .invoke_task_action(0, 6, "Notes survive theme changes".into());
                    app.window.invoke_task_action(0, 0, "".into());
                    assert_eq!(app.window.get_tasks().row_count(), 1);
                    assert!(app.window.get_tasks().row_data(0).unwrap().done);
                    click(&app, 75.0, 94.0);
                    assert_eq!(app.window.get_page(), 0);
                }
                7 => {
                    capture(&app, "tasks")?;
                    click(&app, 180.0, 94.0);
                    assert_eq!(app.window.get_page(), 1);
                    app.window.invoke_special(0, "bad".into(), "5".into());
                    assert_eq!(app.window.get_config_error(), 1);
                    app.window.invoke_special(0, "12:30".into(), "5".into());
                    assert_eq!(app.window.get_config_error(), 0);
                }
                8 => {
                    capture(&app, "config")?;
                    click(&app, 400.0, 94.0);
                    assert_eq!(app.window.get_page(), 3);
                    click(&app, 65.0, 176.0);
                    assert_eq!(app.window.get_color_theme(), 0);
                }
                9 => {
                    capture(&app, "settings-light")?;
                    app.window.set_page(2);
                }
                10..=14 => {
                    let index = n - 10;
                    capture(
                        &app,
                        ["light", "dark", "paper", "forest", "midnight"][index],
                    )?;
                    if index < 4 {
                        app.window.invoke_select_theme(index as i32 + 1);
                    } else {
                        app.window.invoke_setting("language".into());
                    }
                }
                15 => {
                    capture(&app, "russian")?;
                    assert_eq!(
                        app.window.get_message_focus().as_str(),
                        "Время сосредоточиться"
                    );
                    app.flush();
                    let data = Data::decode(platform.saved.borrow().as_deref().unwrap())
                        .map_err(|e| e.to_string())?;
                    assert_eq!(data.settings.theme, ColorTheme::Midnight);
                    assert_eq!(data.settings.language, "ru");
                    assert_eq!(
                        data.lists.last().unwrap().tasks[0].description,
                        "Notes survive theme changes"
                    );
                    let reloaded = App::new(platform.clone()).map_err(|e| e.to_string())?;
                    assert_eq!(reloaded.window.get_color_theme(), 4);
                    assert_eq!(reloaded.core.borrow().settings.language, "ru");
                    app.set_visible(false);
                    assert!(!app.core.borrow().visible);
                    app.set_visible(true);
                }
                16 => {
                    geometry(&app, false)?;
                    app.set_visible(false);
                    app.toggle_mini();
                    app.set_visible(true);
                }
                17 => {
                    geometry(&app, true)?;
                    app.set_visible(false);
                    app.toggle_mini();
                    app.set_visible(true);
                }
                18 => {
                    geometry(&app, false)?;
                    println!("PASS: pointer clicks, every mini entry point, restore, timer controls, strict mode, tasks, config, five themes, Russian, reload and hidden window transitions");
                }
                100 => {
                    capture(&app, "startup-mini")?;
                    geometry(&app, true)?;
                    app.toggle_mini();
                }
                101 => {
                    capture(&app, "startup-restored")?;
                    geometry(&app, false)?;
                    println!("PASS: --mini startup and restore");
                }
                _ => unreachable!(),
            }
            Ok(())
        })();
        if result.is_err() || n == 18 || n == 101 {
            *outcome.borrow_mut() = result;
            let _ = slint::quit_event_loop();
        } else {
            step(app, platform, outcome, n + 1);
        }
    });
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    slint::BackendSelector::new()
        .backend_name("winit".into())
        .renderer_name("software".into())
        .select()?;
    let platform = Rc::new(MemoryPlatform::default());
    if std::env::args().any(|a| a == "--frameless") {
        let mut data = Data::default();
        data.settings.native_titlebar = false;
        *platform.saved.borrow_mut() = Some(serde_json::to_string(&data)?);
    }
    let app = App::new(platform.clone())?;
    let startup = std::env::args().any(|a| a == "--start-mini");
    if startup {
        app.toggle_mini();
    }
    app.window.show()?;
    let outcome = Rc::new(RefCell::new(Err("UI scenario did not finish".to_string())));
    step(
        app,
        platform,
        outcome.clone(),
        if startup { 100 } else { 0 },
    );
    slint::run_event_loop_until_quit()?;
    let checked = std::mem::replace(&mut *outcome.borrow_mut(), Ok(()));
    checked.map_err(Into::into)
}
