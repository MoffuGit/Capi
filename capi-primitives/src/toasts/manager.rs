use leptos::prelude::*;
use reactive_stores::Store;
use uuid::Uuid;

use super::{Toast, ToastContext, ToastStore};

pub struct ToastManager {
    pub toasts: RwSignal<Vec<Toast>>,
    pub add: Callback<Toast>,
    pub close: Callback<Uuid>,
}

pub fn use_toast_store() -> Store<ToastStore> {
    let ToastContext { store, .. } = use_context().expect("should acces to the toast context");
    store
}
