use leptos::prelude::*;
use uuid::Uuid;

use super::{Toast, ToastContext};

pub struct ToastManager {
    pub toasts: RwSignal<Vec<Toast>>,
    pub add: Callback<Toast>,
    pub close: Callback<Uuid>,
}

pub fn use_toast_manager() -> ToastManager {
    let ToastContext {
        toasts, add, close, ..
    } = use_context().expect("should acces to the toast context");
    ToastManager { toasts, add, close }
}
