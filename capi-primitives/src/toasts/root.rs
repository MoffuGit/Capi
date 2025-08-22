use std::time::Duration;

use leptos::context::Provider;
use leptos::prelude::*;

use crate::common::status::use_transition_status;
use crate::toasts::ToastContext;

use super::Toast;

pub struct ToastRootContext {
    toast: Toast,
}

#[component]
pub fn ToastRoot(
    children: Children,
    #[prop(into, optional)] class: Signal<String>,
    toast: Toast,
) -> impl IntoView {
    let ToastContext {
        hovering,
        toasts,
        limit,
        ..
    } = use_context().expect("should acces to the toast context");
    let offset_y = Memo::new(move |_| {
        toasts
            .get()
            .iter()
            .rev()
            .take_while(|t| t.id != toast.id)
            .fold(0, |acc, _| acc + 42)
    });
    let index = Memo::new(move |_| {
        toasts
            .get()
            .iter()
            .rev()
            .position(|t| t.id == toast.id)
            .unwrap()
    });

    let limited = Memo::new(move |_| index() + 1 > limit as usize);

    let mounted = RwSignal::new(false);

    Effect::new(move |_| {
        mounted.set(true);
    });

    Effect::new(move |_| {
        set_timeout(
            move || {
                mounted.set(false);
            },
            Duration::from_millis(toast.timeout),
        );
    });

    let state = use_transition_status(mounted.into(), toast.node_ref);

    let front = Memo::new(move |_| index() == 0);

    view! {
        <Provider value=ToastRootContext {
            toast,
        }>
            <div
                node_ref=toast.node_ref
                class=class
                data-expanded=move || hovering.get().to_string()
                data-limited=move || limited.get().to_string()
                data-state=move || state.transition_status.get().to_string()
                data-front=move || front.get().to_string()
                style=move || {
                    format!("--toast-offset-y: {}px; --toast-index: {}", offset_y(), index())
                }
            >
                {children()}
            </div>
        </Provider>
    }
}
