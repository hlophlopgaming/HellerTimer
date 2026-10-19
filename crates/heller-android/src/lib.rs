#[cfg(target_os = "android")]
mod android {
    use heller_ui::{App, Caps, Platform};
    use jni::{
        objects::{JObject, JString, JValue},
        JavaVM,
    };
    use slint::{android::AndroidApp, ComponentHandle};
    struct Android {
        app: AndroidApp,
        deadline: std::cell::Cell<u64>,
    }
    impl Android {
        fn call<T>(
            &self,
            f: impl FnOnce(&mut jni::JNIEnv, JObject) -> jni::errors::Result<T>,
        ) -> Option<T> {
            // AndroidApp owns the VM/activity and outlives every borrowed JNI reference here.
            let result = (|| {
                let vm = unsafe { JavaVM::from_raw(self.app.vm_as_ptr().cast()) }?;
                let mut env = vm.attach_current_thread()?;
                let activity = unsafe { JObject::from_raw(self.app.activity_as_ptr().cast()) };
                f(&mut env, activity)
            })();
            match result {
                Ok(v) => Some(v),
                Err(e) => {
                    eprintln!("Android bridge: {e}");
                    None
                }
            }
        }
    }
    impl Platform for Android {
        fn load(&self) -> Option<String> {
            self.call(|env, a| {
                let result = env
                    .call_method(a, "loadData", "()Ljava/lang/String;", &[])?
                    .l()?;
                let text = env.get_string(&JString::from(result))?.into();
                Ok(text)
            })
        }
        fn save(&self, data: &str) {
            self.call(|env, a| {
                let s = env.new_string(data)?;
                env.call_method(
                    a,
                    "saveData",
                    "(Ljava/lang/String;)V",
                    &[JValue::Object(&s)],
                )?;
                Ok(())
            });
        }
        fn capabilities(&self) -> Caps {
            Caps {
                notifications: true,
                ..Caps::default()
            }
        }
        fn utc_offset_seconds(&self) -> i32 {
            self.call(|env, a| env.call_method(a, "utcOffset", "()I", &[])?.i())
                .unwrap_or(0)
        }
        fn open_url(&self, url: &str) {
            self.call(|env, a| {
                let text = env.new_string(url)?;
                env.call_method(
                    a,
                    "openUrl",
                    "(Ljava/lang/String;)V",
                    &[JValue::Object(&text)],
                )?;
                Ok(())
            });
        }
        fn effect(&self, _e: heller_core::Effect, settings: &heller_core::Settings, message: &str) {
            if settings.notification == 0 {
                return;
            }
            self.call(|env, a| {
                let text = env.new_string(message)?;
                env.call_method(
                    a,
                    "notifyTimer",
                    "(Ljava/lang/String;)V",
                    &[JValue::Object(&text)],
                )?;
                Ok(())
            });
        }
        fn state_changed(&self, core: &heller_core::Core) {
            let deadline = match core.state {
                heller_core::RunState::Running { deadline } => deadline
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis()
                    as u64,
                _ => 0,
            };
            if self.deadline.replace(deadline) == deadline {
                return;
            }
            self.call(|env, a| {
                let text = env.new_string(if core.phase == heller_core::Phase::Focus {
                    "Focus"
                } else {
                    "Break"
                })?;
                env.call_method(
                    a,
                    "schedule",
                    "(JLjava/lang/String;)V",
                    &[JValue::Long(deadline as i64), JValue::Object(&text)],
                )?;
                Ok(())
            });
        }
    }
    #[no_mangle]
    fn android_main(app: AndroidApp) {
        let platform = std::rc::Rc::new(Android {
            app: app.clone(),
            deadline: std::cell::Cell::new(0),
        });
        let slot = std::rc::Rc::new(std::cell::RefCell::new(std::rc::Weak::<App>::new()));
        let listener = slot.clone();
        if let Err(e) = slint::android::init_with_event_listener(app, move |event| {
            use slint::android::android_activity::{MainEvent, PollEvent};
            let visible = match event {
                PollEvent::Main(MainEvent::Resume { .. }) => Some(true),
                PollEvent::Main(MainEvent::Pause) => Some(false),
                _ => None,
            };
            if let (Some(visible), Some(app)) = (visible, listener.borrow().upgrade()) {
                app.core.borrow_mut().visible = visible;
                app.dispatch(heller_core::Input::Tick);
                if !visible {
                    app.flush();
                }
            }
        }) {
            eprintln!("Android backend: {e}");
            return;
        }
        match App::new(platform) {
            Ok(app) => {
                *slot.borrow_mut() = std::rc::Rc::downgrade(&app);
                if let Err(e) = app.window.run() {
                    eprintln!("Android event loop: {e}");
                }
                app.flush();
            }
            Err(e) => eprintln!("Android UI: {e}"),
        }
    }
}
