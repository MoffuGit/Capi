use std::time::Duration;

use web_time::Instant;

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
        close,
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

    let removed = RwSignal::new(false);

    Effect::new(move |_| {
        mounted.set(true);
    });

    let timeout_start_time: RwSignal<Option<Instant>> = RwSignal::new(None);
    let timeout_remaining_duration: RwSignal<Option<Duration>> = RwSignal::new(None);

    Effect::new(move |prev_handler: Option<Option<TimeoutHandle>>| {
        if let Some(handle) = prev_handler.flatten() {
            handle.clear();
        }

        if hovering() {
            if let Some(start) = timeout_start_time.get_untracked() {
                let elapsed = Instant::now().duration_since(start);
                let full_duration = Duration::from_millis(toast.timeout);
                let remaining = full_duration.saturating_sub(elapsed);
                timeout_remaining_duration.set(Some(remaining));
            }
            timeout_start_time.set(None);
            None
        } else {
            let duration_to_use = timeout_remaining_duration
                .get_untracked()
                .unwrap_or_else(|| Duration::from_millis(toast.timeout));

            timeout_start_time.set(Some(Instant::now()));
            timeout_remaining_duration.set(None);

            let new_handler = set_timeout_with_handle(
                move || {
                    mounted.set(false);
                    removed.set(true);
                },
                duration_to_use,
            );

            new_handler.ok()
        }
    });

    let state = use_transition_status(mounted.into(), toast.node_ref);

    Effect::new(move |_| {
        if removed() && !state.mounted.get() {
            close.run(toast.id);
        }
    });

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
