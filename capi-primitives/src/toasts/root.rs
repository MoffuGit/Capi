use leptos::context::Provider;
use leptos::html::Div;
use leptos::prelude::*;

use crate::toasts::ToastContext;

use super::Toast;

pub struct ToastRootContext {
    toast: Toast,
    node_ref: NodeRef<Div>,
}

#[component]
pub fn ToastRoot(
    children: Children,
    #[prop(into, optional)] class: Signal<String>,
    toast: Toast,
    #[prop(into, optional)] node_ref: NodeRef<Div>,
) -> impl IntoView {
    let ToastContext {
        hovering, toasts, ..
    } = use_context().expect("should acces to the toast context");
    let offset_y = Memo::new(move |_| {
        toasts
            .get()
            .iter()
            .take_while(|t| t.id != toast.id)
            .fold(0, |acc, _| acc + 24)
    });
    view! {
        <Provider value=ToastRootContext {
            toast,
            node_ref
        }>
            <div
                class=class
                data-expanded=move || hovering.get().to_string()
                style=move || {
                    format!("--toast-offset-y: {}px", offset_y())
                }
            >
                {children()}
            </div>
        </Provider>
    }
}
