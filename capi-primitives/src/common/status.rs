use leptos::html::Div;
use leptos::logging::{error, warn};
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::{Event, window};

#[derive(Clone, Copy)]
pub struct AnimationFrame;

impl AnimationFrame {
    pub fn create() -> Self {
        Self {}
    }

    pub fn request(f: Rc<Closure<dyn Fn()>>) -> impl Fn() + 'static {
        let handle = window()
            .expect("should acces the window")
            .request_animation_frame((*f).as_ref().unchecked_ref())
            .unwrap();

        // Return a cleanup function for manual cancellation
        move || {
            if let Some(window) = web_sys::window() {
                window.cancel_animation_frame(handle).unwrap_or_default();
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum TransitionStatus {
    Opening,
    Closing,
    Open,
    Closed,
}

fn parse_css_duration(duration_str: &str) -> Option<u64> {
    if duration_str.ends_with("ms") {
        duration_str[..duration_str.len() - 2].parse::<u64>().ok()
    } else if duration_str.ends_with("s") {
        let seconds = duration_str[..duration_str.len() - 1].parse::<f64>().ok()?;
        Some((seconds * 1000.0).round() as u64)
    } else {
        None
    }
}

pub fn use_transition_status(
    open: Signal<bool>,
    content_node_ref: NodeRef<Div>,
    enable_idle_state: bool,
    defer_ending_state: bool,
) -> TransitionStatusState {
    let transition_status: RwSignal<TransitionStatus> = RwSignal::new(TransitionStatus::Closed);
    let mounted: RwSignal<bool> = RwSignal::new(false);

    let transition_duration_ms: RwSignal<u64> = RwSignal::new(150);

    #[cfg(feature = "hydrate")]
    let read_style_closure = Rc::new(Closure::new(move || {
        if let Some(element) = content_node_ref
            .get_untracked()
            .map(|element| element.unchecked_into::<web_sys::HtmlElement>())
        {
            if let Some(style) = window()
                .and_then(|window| window.get_computed_style(&element).ok())
                .flatten()
            {
                let mut max_duration = 0;

                if let Ok(duration_str) = style.get_property_value("animation-duration")
                    && let Some(parsed_duration) = parse_css_duration(&duration_str)
                {
                    max_duration = max_duration.max(parsed_duration);
                }

                if let Ok(duration_str) = style.get_property_value("transition-duration")
                    && let Some(parsed_duration) = parse_css_duration(&duration_str)
                {
                    max_duration = max_duration.max(parsed_duration);
                }

                if max_duration == 0 {
                    transition_duration_ms.set(150);
                } else {
                    transition_duration_ms.set(max_duration);
                }
            } else {
                transition_duration_ms.set(150);
            }
        } else {
            transition_duration_ms.set(150);
        }
    }));

    Effect::watch(
        move || transition_status.get(),
        move |current_status, _, _| {
            if current_status == &TransitionStatus::Opening
                || current_status == &TransitionStatus::Closing
            {
                #[cfg(feature = "hydrate")]
                {
                    // Schedule the closure to run on the next animation frame
                    let cancel_frame = AnimationFrame::request(read_style_closure.clone());
                    on_cleanup(move || {
                        cancel_frame();
                    });
                }
            }
        },
        false,
    );

    Effect::new(move |_| {
        let current_open = open.get();
        let current_status = transition_status.get();

        if current_open || current_status == TransitionStatus::Closing {
            if !mounted.get_untracked() {
                mounted.set(true);
            }
        } else if !current_open
            && current_status == TransitionStatus::Closed
            && mounted.get_untracked()
        {
            mounted.set(false);
        }
    });

    #[cfg(feature = "hydrate")]
    let closure_for_animation_frame = Rc::new(Closure::new(move || {
        transition_status.set(TransitionStatus::Opening);
    }));
    Effect::new(move |_| {
        let current_open = open.get();
        let current_status = transition_status.get();

        if current_open
            && (current_status == TransitionStatus::Closed
                || current_status == TransitionStatus::Closing)
        {
            #[cfg(feature = "hydrate")]
            {
                let cancel_frame = AnimationFrame::request(closure_for_animation_frame.clone());
                on_cleanup(move || {
                    cancel_frame();
                });
            }
        }
    });

    Effect::new(move |_| {
        let current_open = open.get();
        let current_status = transition_status.get();
        let enable_idle_captured = enable_idle_state;

        if current_open && current_status == TransitionStatus::Opening && enable_idle_captured {
            #[cfg(feature = "hydrate")]
            {
                let transition_status_setter = transition_status;
                let timeout_handle = set_timeout_with_handle(
                    move || {
                        transition_status_setter.set(TransitionStatus::Open);
                    },
                    std::time::Duration::from_millis(transition_duration_ms.get()),
                )
                .expect("Failed to set timeout for Idle transition");
                on_cleanup(move || {
                    timeout_handle.clear();
                });
            }
        }
    });

    Effect::new(move |_| {
        let current_open = open.get();
        let current_status = transition_status.get();
        let current_mounted = mounted.get();

        if !current_open
            && current_mounted
            && current_status != TransitionStatus::Closing
            && !defer_ending_state
        {
            transition_status.set(TransitionStatus::Closing);
        }
    });

    #[cfg(feature = "hydrate")]
    let ending = Rc::new(Closure::new(move || {
        transition_status.set(TransitionStatus::Closing);
    }));

    Effect::new(move |_| {
        #[cfg(feature = "hydrate")]
        {
            let open_val = open.get();
            let mounted_val = mounted.get();
            let status_val = transition_status.get();

            if !open_val
                && mounted_val
                && status_val != TransitionStatus::Closing
                && defer_ending_state
            {
                let cancel_frame = AnimationFrame::request(ending.clone());
                on_cleanup(move || {
                    cancel_frame();
                });
            }
        }
    });

    Effect::new(move |_| {
        let current_open = open.get();
        let current_status = transition_status.get();
        let node_ref_clone = content_node_ref;

        if !current_open && current_status == TransitionStatus::Closing {
            #[cfg(feature = "hydrate")]
            {
                let duration = transition_duration_ms.get_untracked();

                let transition_status_setter = transition_status;

                let set_status_to_closed = move || {
                    transition_status_setter.set(TransitionStatus::Closed);
                };

                if duration == 0 {
                    let timeout_handle = set_timeout_with_handle(
                        set_status_to_closed,
                        std::time::Duration::from_millis(0),
                    )
                    .expect("Failed to set 0ms timeout for immediate Closed transition");
                    on_cleanup(move || {
                        timeout_handle.clear();
                    });
                } else if let Some(element) = node_ref_clone
                    .get()
                    .map(|el| el.unchecked_into::<web_sys::HtmlElement>())
                {
                    let element_for_closure = element.clone();

                    let js_closures =
                        Rc::new(RefCell::new(Vec::<web_sys::js_sys::Function>::new()));

                    let setup_event_listener = move |event_name_str: &str,
                                                     element: &web_sys::HtmlElement,
                                                     closures_ref: Rc<
                        RefCell<Vec<web_sys::js_sys::Function>>,
                    >| {
                        let event_name_owned = event_name_str.to_string();
                        let element_for_callback = element_for_closure.clone();
                        let set_status_to_closed_for_callback = set_status_to_closed;

                        let closure_for_event =
                            Closure::wrap(Box::new(move |event: web_sys::Event| {
                                if let Some(target) = event.target()
                                    && let Ok(html_element) =
                                        target.dyn_into::<web_sys::HtmlElement>()
                                {
                                    if element_for_callback.is_same_node(Some(&html_element)) {
                                        set_status_to_closed_for_callback();
                                    } else {
                                        warn!(
                                            "{} event fired on a different element than expected.",
                                            event_name_owned
                                        );
                                    }
                                }
                            }) as Box<dyn Fn(Event)>)
                            .into_js_value();

                        let options = web_sys::AddEventListenerOptions::new();
                        options.set_once(true);

                        if let Err(e) = element
                            .add_event_listener_with_callback_and_add_event_listener_options(
                                event_name_str,
                                closure_for_event.as_ref().unchecked_ref(),
                                &options,
                            )
                        {
                            error!("Failed to add {} listener: {:?}", event_name_str, e);
                            None
                        } else {
                            closures_ref
                                .borrow_mut()
                                .push(closure_for_event.unchecked_into());
                            Some(())
                        }
                    };

                    setup_event_listener("animationend", &element, js_closures.clone());
                    setup_event_listener("transitionend", &element, js_closures.clone());

                    let timeout_handle = set_timeout_with_handle(
                        move || {
                            set_status_to_closed();
                            warn!("Fallback timeout hit for {}ms.", duration);
                        },
                        std::time::Duration::from_millis(duration),
                    )
                    .expect("Failed to set fallback timeout");

                    on_cleanup({
                        let closures_to_drop = SendWrapper::new(js_closures.clone());
                        move || {
                            closures_to_drop.take().borrow_mut().clear();
                            timeout_handle.clear();
                        }
                    });
                } else {
                    warn!(
                        "Element for animationend/transitionend listener not found for closing transition, falling back to timeout."
                    );
                    let transition_status_setter_fallback = transition_status;
                    let timeout_handle = set_timeout_with_handle(
                        move || {
                            transition_status_setter_fallback.set(TransitionStatus::Closed);
                            warn!(
                                "Fallback timeout hit (no element): TransitionStatus set to Closed."
                            );
                        },
                        std::time::Duration::from_millis(duration),
                    )
                    .expect("Failed to set timeout for Undefined transition (fallback)");
                    on_cleanup(move || {
                        timeout_handle.clear();
                    });
                }
            }
        }
    });

    let (mounted, set_mounted) = mounted.split();

    TransitionStatusState {
        mounted,
        set_mounted,
        transition_status,
    }
}

#[derive(Clone, Copy)]
pub struct TransitionStatusState {
    pub mounted: ReadSignal<bool>,
    pub set_mounted: WriteSignal<bool>,
    pub transition_status: RwSignal<TransitionStatus>,
}
