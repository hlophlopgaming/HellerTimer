#[cfg(target_arch = "wasm32")]
mod web {
    use heller_ui::{App, Caps, Platform};
    use wasm_bindgen::prelude::*;
    struct Web {
        audio: Option<web_sys::HtmlAudioElement>,
    }
    impl Platform for Web {
        fn load(&self) -> Option<String> {
            web_sys::window()?
                .local_storage()
                .ok()??
                .get_item("hellertimer-v1")
                .ok()?
        }
        fn save(&self, data: &str) {
            if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten())
            {
                if let Err(e) = storage.set_item("hellertimer-v1", data) {
                    web_sys::console::error_1(&e);
                }
            }
        }
        fn capabilities(&self) -> Caps {
            Caps {
                notifications: true,
                audio: self.audio.is_some(),
                ..Caps::default()
            }
        }
        fn utc_offset_seconds(&self) -> i32 {
            (-js_sys::Date::new_0().get_timezone_offset() * 60.0) as i32
        }
        fn effect(&self, _e: heller_core::Effect, s: &heller_core::Settings, message: &str) {
            if s.sound_enabled {
                if let Some(audio) = &self.audio {
                    audio.set_muted(false);
                    audio.set_current_time(0.0);
                    let _ = audio.play();
                }
            }
            if s.notification > 0 {
                let _ = web_sys::Notification::new(message);
            }
        }
        fn request_notifications(&self) {
            let _ = web_sys::Notification::request_permission();
        }
        fn open_url(&self, url: &str) {
            if let Some(w) = web_sys::window() {
                let _ = w.open_with_url_and_target(url, "_blank");
            }
        }
    }
    #[wasm_bindgen(start)]
    pub fn start() -> Result<(), JsValue> {
        let audio = web_sys::HtmlAudioElement::new_with_src("bell.wav").ok();
        if let (Some(doc), Some(audio)) =
            (web_sys::window().and_then(|w| w.document()), audio.clone())
        {
            let unlocked = std::cell::Cell::new(false);
            let callback = Closure::<dyn FnMut()>::new(move || {
                if !unlocked.replace(true) {
                    audio.set_muted(true);
                    let _ = audio.play();
                }
            });
            doc.add_event_listener_with_callback("pointerdown", callback.as_ref().unchecked_ref())?;
            callback.forget();
        }
        let app = App::new(std::rc::Rc::new(Web { audio }))
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let weak = std::rc::Rc::downgrade(&app);
            let callback = Closure::<dyn FnMut()>::new(move || {
                if let Some(a) = weak.upgrade() {
                    let visible = web_sys::window()
                        .and_then(|w| w.document())
                        .is_some_and(|d| !d.hidden());
                    a.core.borrow_mut().visible = visible;
                    a.dispatch(heller_core::Input::Tick);
                }
            });
            doc.add_event_listener_with_callback(
                "visibilitychange",
                callback.as_ref().unchecked_ref(),
            )?;
            callback.forget();
        }
        app.window
            .show()
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        APP.with(|a| *a.borrow_mut() = Some(app));
        slint::run_event_loop().map_err(|e| JsValue::from_str(&e.to_string()))
    }
    thread_local! {static APP:std::cell::RefCell<Option<std::rc::Rc<App>>>=const{std::cell::RefCell::new(None)};}
    use slint::ComponentHandle;
}
