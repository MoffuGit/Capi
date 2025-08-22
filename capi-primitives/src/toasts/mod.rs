pub mod manager;
pub mod portal;
pub mod root;
pub mod viewport;

pub use manager::ToastManager;
pub use portal::ToastPortal;
pub use root::ToastRoot;
pub use viewport::ToastViewport;

use leptos::context::Provider;
use leptos::html::Div;
use leptos::prelude::*;
use uuid::Uuid;

#[derive(Clone, Copy)]
pub struct Toast {
    pub id: Uuid,
    pub node_ref: NodeRef<Div>,
    pub title: MaybeProp<String>,
    pub _type: MaybeProp<String>,
    pub description: MaybeProp<String>,
    pub timeout: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct ToastContext {
    pub toasts: RwSignal<Vec<Toast>>,
    pub hovering: RwSignal<bool>,
    pub add: Callback<Toast>,
    pub close: Callback<Uuid>,
    pub view_ref: NodeRef<Div>,
    // pub timer_refs: RwSignal<HashMap<Uuid, TimeoutHandle>>,
}

#[component]
pub fn ToastProvider(
    children: Children,
    #[prop(into, optional)] view_ref: NodeRef<Div>,
    #[prop(into, optional)] toasts: RwSignal<Vec<Toast>>,
    #[prop(into, optional)] hovering: RwSignal<bool>,
) -> impl IntoView {
    Effect::new(move |_| {
        if toasts.get().is_empty() {
            hovering.set(false);
        }
    });
    let context = ToastContext {
        toasts,
        hovering,
        add: Callback::new(move |toast| {
            toasts.update(|toasts| {
                toasts.push(toast);
            });
        }),
        close: Callback::new(move |id| {
            toasts.update(|toasts| {
                toasts.retain(|toast| toast.id != id);
            });
        }),
        view_ref,
    };
    view! {
        <Provider value=context>
            {children()}
        </Provider>
    }
}
