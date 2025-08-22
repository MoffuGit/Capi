use std::time::Duration;

use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use web_time::Instant;

use leptos::context::Provider;
use leptos::prelude::*;

use crate::common::Point;
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
            .filter(|toast| !toast.removed.get())
            .take_while(|t| t.id != toast.id)
            .fold(0.0, |acc, _| acc + 42.0)
    });
    let index = Memo::new(move |_| {
        toasts
            .get()
            .iter()
            .rev()
            .filter(|toast| !toast.removed.get())
            .position(|t| t.id == toast.id)
    });

    let limited = Memo::new(move |_| index().is_some_and(|index| index + 1 > limit as usize));

    let mounted = RwSignal::new(false);

    let removed = toast.removed;

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

            //NOTE: if you swipe out the component whit will run either way
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

    let front = Memo::new(move |_| index().is_some_and(|index| index == 0));

    let pointer_start = RwSignal::new(None::<Point>);
    let (swiping, set_swiping) = signal(false);
    let (swipe_amount, set_swipe_amount) = signal(0.0);
    let drag_start_time = RwSignal::new(None::<Instant>);
    let offset_before_removed = RwSignal::new(0.0);

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
                data-swiping=move || swiping.get().to_string()
                data-removed=move || removed.get().to_string()
                style=move || {
                    format!("--toast-offset-y: {}px; --toast-index: {}; --toast-swipe-movement-y: {}px", if removed() { offset_before_removed() } else { offset_y() }, index().unwrap_or_default(), swipe_amount())
                }
                on:dragend=move |_| {
                    pointer_start.set(None);
                    set_swiping(false);
                }
                on:pointerdown=move |evt| {
                    if evt.button() == 2 {
                        return
                    }
                    drag_start_time.set(Some(Instant::now()));
                    offset_before_removed.set(offset_y.get_untracked());
                    evt.target().map(|target| target.unchecked_into::<HtmlElement>().set_pointer_capture(evt.pointer_id()));
                    set_swiping(true);
                    pointer_start.set(
                        Some(Point {
                            x: evt.client_x().into(),
                            y: evt.client_y().into()
                        })
                    );
                }
                on:pointerup=move |_| {
                    pointer_start.set(None);
                    let swipe_amount: f64 = swipe_amount.get_untracked();
                    let time_taken = if let Some(start) = drag_start_time.get() {
                        Instant::now().saturating_duration_since(start)
                                } else {
                        Duration::new(0, 0)
                    };
                    let velocity = swipe_amount.abs() / (time_taken.as_secs_f64() * 1000.0);
                    if swipe_amount.abs() >= 45.0 || velocity > 0.11 {
                        offset_before_removed.set(offset_y.get_untracked());
                        mounted.set(false);
                        removed.set(true);
                        return;
                    }
                    set_swipe_amount(0.0);
                    set_swiping(false);
                }
                on:pointermove=move |evt| {
                    if let Some(pointer) = pointer_start.get() {
                        let get_dampening = move |n: f64| {
                            let factor = n.abs() / 20.0;
                            1.0 / (1.5 + factor)
                        };
                        let y_delta = evt.client_y() as f64 - pointer.y;
                        let x_delta = evt.client_x() as f64 - pointer.x;
                        if y_delta > 0.0 {
                            set_swipe_amount(y_delta)
                        } else {
                            let damped_delta = y_delta * get_dampening(y_delta);
                            set_swipe_amount(if damped_delta.abs() < y_delta.abs() { damped_delta } else { y_delta })
                        }
                    }
                }
            >
                {children()}
            </div>
        </Provider>
    }
}
