use leptos::html::Div;
use leptos::logging::error;
use leptos::prelude::*;
use leptos_dom::warn;
use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::window;

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
    content_node_ref: NodeRef<Div>, // New parameter to get the element for CSS reading
    enable_idle_state: bool,
    defer_ending_state: bool,
) -> TransitionStatusState {
    // Always start in Undefined, allow effects to transition to Idle.
    let transition_status: RwSignal<TransitionStatus> = RwSignal::new(TransitionStatus::Closed);
    // `mounted` controls whether the dialog content (Portal) is in the DOM.
    // It should be true when the dialog is in any active transition state (Starting, Ending, Idle),
    // and false when it's Undefined (i.e., fully closed and removed from DOM).
    let mounted: RwSignal<bool> = RwSignal::new(false);

    // New signal to store the dynamically determined transition duration from CSS
    let transition_duration_ms: RwSignal<u64> = RwSignal::new(150); // Default fallback duration (e.g., for SSR or if style not found)

    // Effect to read the transition-duration from the content_node_ref element
    Effect::new(move |_| {
        if transition_status.get() == TransitionStatus::Opening
            || transition_status.get() == TransitionStatus::Closing
        {
            #[cfg(not(feature = "ssr"))]
            if let Some(element) = content_node_ref.get() {
                let element: web_sys::HtmlElement = element.unchecked_into();
                if let Some(window) = window() {
                    if let Ok(Some(style)) = window.get_computed_style(&element) {
                        if let Ok(duration_str) = style.get_property_value("transition-duration") {
                            if let Some(parsed_duration) = parse_css_duration(&duration_str) {
                                transition_duration_ms.set(parsed_duration);
                            } else {
                                leptos::logging::error!("Could not parse transition-duration CSS property: '{}'. Falling back to 150ms.", duration_str);
                                transition_duration_ms.set(150); // Fallback if parsing fails
                            }
                        } else {
                            leptos::logging::warn!(
                                "'transition-duration' CSS property not found. Falling back to 150ms."
                            );
                            transition_duration_ms.set(150); // Fallback if property not found
                        }
                    } else {
                        leptos::logging::warn!(
                            "Could not get computed style for content element. Falling back to 150ms."
                        );
                        transition_duration_ms.set(150); // Fallback if get_computed_style fails
                    }
                } else {
                    leptos::logging::warn!("Window object not available. Falling back to 150ms.");
                    transition_duration_ms.set(150); // Fallback if window is not available (shouldn't happen on client)
                }
            }
        }
    });

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

    // Effect 3: Set `transition_status` to `Starting` when dialog opens or re-opens,
    // and then schedule transition to `Idle` if enabled.
    Effect::new(move |_| {
        let current_open = open.get();
        let current_status = transition_status.get();

        // Condition for setting to Starting:
        // If open and not already in Starting or Idle (meaning it's closed or just mounted)
        if current_open
            && (current_status == TransitionStatus::Closed
                || current_status == TransitionStatus::Closing)
        {
            // Delay setting 'Starting' using a 0ms timeout to ensure the DOM renders
            // the initial state before applying the 'Starting' state for animation.
            #[cfg(not(feature = "ssr"))]
            {
                let transition_status_setter = transition_status; // Capture RwSignal
                let timeout_handle = set_timeout_with_handle(
                    move || {
                        transition_status_setter.set(TransitionStatus::Opening);
                    },
                    std::time::Duration::from_millis(0), // Defer to next event loop tick
                )
                .expect("Failed to set timeout for Starting transition");
                on_cleanup(move || {
                    timeout_handle.clear();
                });
            }
        }
    });

    Effect::new(move |_| {
        let current_open = open.get();
        let current_status = transition_status.get();
        let enable_idle_captured = enable_idle_state;

        // Only transition to Idle if the component is currently opening (Starting)
        // and the `open` signal is still true, and Idle state is enabled.
        if current_open && current_status == TransitionStatus::Opening && enable_idle_captured {
            #[cfg(not(feature = "ssr"))]
            {
                let transition_status_setter = transition_status;
                let timeout_handle = set_timeout_with_handle(
                    move || {
                        transition_status_setter.set(TransitionStatus::Open);
                    },
                    std::time::Duration::from_millis(transition_duration_ms.get()), // Use dynamically read duration
                )
                .expect("Failed to set timeout for Idle transition");
                on_cleanup(move || {
                    timeout_handle.clear();
                });
            }
        }
    });

    // Effect 4: Set `transition_status` to `Ending` immediately on close if not deferred.
    // This initiates the closing animation for non-deferred cases.
    Effect::new(move |_| {
        let current_open = open.get();
        let current_status = transition_status.get();
        let current_mounted = mounted.get(); // Track mounted to ensure it's still active

        if !current_open
            && current_mounted
            && current_status != TransitionStatus::Closing
            && !defer_ending_state
        {
            transition_status.set(TransitionStatus::Closing);
        }
    });

    #[cfg(not(feature = "ssr"))]
    let ending = Rc::new(Closure::new(move || {
        transition_status.set(TransitionStatus::Closing);
    }));

    // Effect 5: Deferred `Ending` transition using `AnimationFrame` for closing animations.
    // This effect determines *when* the 'Ending' status is set if deferred.
    Effect::new(move |_| {
        #[cfg(not(feature = "ssr"))]
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

    // Effect: From Ending to Undefined after animation completes.
    // This handles the "animation out" completion.
    Effect::new(move |_| {
        let current_open = open.get();
        let current_status = transition_status.get();

        if !current_open && current_status == TransitionStatus::Closing {
            let transition_status_setter = transition_status;
            let timeout_handle = set_timeout_with_handle(
                move || {
                    transition_status_setter.set(TransitionStatus::Closed);
                },
                std::time::Duration::from_millis(
                    transition_duration_ms
                        .get()
                        .checked_sub(10)
                        .unwrap_or_default(),
                ), // Use dynamically read duration
            )
            .expect("Failed to set timeout for Undefined transition");
            on_cleanup(move || {
                timeout_handle.clear();
            });
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
