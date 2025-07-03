#![allow(dead_code)]
pub mod size;
pub mod status;

use leptos::prelude::*;

#[derive(Clone, strum_macros::Display)]
pub enum Orientation {
    #[strum(to_string = "horizontal")]
    Horizontal,
    #[strum(to_string = "vertical")]
    Vertical,
}

#[derive(Clone, Copy, strum_macros::Display, PartialEq)]
pub enum Side {
    #[strum(to_string = "top")]
    Top,
    #[strum(to_string = "bottom")]
    Bottom,
    #[strum(to_string = "left")]
    Left,
    #[strum(to_string = "right")]
    Right,
}

pub fn is_mobile() -> Memo<bool> {
    let (window_size, set_window_size) = signal(f64::MAX);

    let is_mobile = Memo::new(move |_| window_size.get() <= 768.0);

    #[cfg(not(feature = "ssr"))]
    {
        use send_wrapper::SendWrapper;
        use wasm_bindgen::prelude::Closure;
        use wasm_bindgen::JsCast;
        let update = move || {
            set_window_size(
                window()
                    .inner_width()
                    .unwrap_or_default()
                    .as_f64()
                    .unwrap_or_default(),
            );
        };
        let closure = Closure::wrap(Box::new(update) as Box<dyn Fn()>).into_js_value();
        let cleanup_fn = {
            let closure_js = closure.clone();
            move || {
                let _ = window().remove_event_listener_with_callback(
                    "resize",
                    closure_js.as_ref().unchecked_ref(),
                );
            }
        };
        Effect::new(move |_| {
            update();

            let _ = window()
                .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
        });
        on_cleanup({
            let cleanup = SendWrapper::new(cleanup_fn);
            move || cleanup.take()()
        });
    }

    is_mobile
}
