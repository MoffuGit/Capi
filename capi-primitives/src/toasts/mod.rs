pub mod close;
pub mod description;
pub mod manager;
pub mod portal;
pub mod root;
pub mod title;
pub mod viewport;

pub use close::ToastClose;
pub use description::ToastDescription;
pub use manager::ToastManager;
pub use portal::ToastPortal;
use reactive_stores::{Patch, Store};
pub use root::ToastRoot;
pub use title::ToastTitle;
pub use viewport::ToastViewport;

use leptos::context::Provider;
use leptos::html::Div;
use leptos::prelude::*;

#[derive(Clone, Store, Patch, Debug)]
pub struct Toast {
    pub id: u128,
    pub title: String,
    pub _type: String,
    pub description: String,
    pub removed: bool,
    pub timeout: u64,
    pub height: f64,
}

#[derive(Debug, Clone, Store, Patch)]
pub struct ToastStore {
    #[store(key: u128 = |toast| toast.id)]
    toasts: Vec<Toast>,
}

#[derive(Debug, Clone, Copy)]
pub struct ToastContext {
    pub store: Store<ToastStore>,
    pub hovering: RwSignal<bool>,
    pub view_ref: NodeRef<Div>,
    pub limit: u64,
}

#[component]
pub fn ToastProvider(
    children: Children,
    #[prop(into, optional)] view_ref: NodeRef<Div>,
    #[prop(into, optional)] hovering: RwSignal<bool>,
    #[prop(into, optional, default = 3)] limit: u64,
) -> impl IntoView {
    let store = Store::new(ToastStore { toasts: vec![] });
    Effect::new(move |_| {
        if store.toasts().get().is_empty() {
            hovering.set(false);
        }
    });
    let context = ToastContext {
        store,
        hovering,
        view_ref,
        limit,
    };
    view! {
        <Provider value=context>
            {children()}
        </Provider>
    }
}
