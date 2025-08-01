use leptos::context::Provider;
use leptos::html::Div;
use leptos::prelude::*;
use leptos_dom::error;
use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

use super::common::status::{AnimationFrame, TransitionStatus};

#[derive(Clone)]
pub struct CollapsibleContext {
    open: RwSignal<bool>,
    state: TransitionStatusState,
    dimensions: RwSignal<Dimensions>,
    trigger_ref: NodeRef<Div>,
    content_ref: NodeRef<Div>,
}

fn use_collapsible_context() -> CollapsibleContext {
    use_context().expect("should acces to teh collapsible context")
}

#[derive(Debug, Clone, Copy, PartialEq)] // Derive PartialEq for comparison in Effect
pub struct Dimensions {
    width: Option<i32>,
    height: Option<i32>,
}

#[component]
pub fn CollapsibleRoot(
    #[prop(into, optional, default = RwSignal::new(false))] open: RwSignal<bool>,
    children: Children,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, default = 150)] open_duration: u64,
    #[prop(optional, default = 150)] close_duration: u64,
) -> impl IntoView {
    let state = use_transition_status(open.into(), true, true, open_duration, close_duration);

    let dimensions = RwSignal::new(Dimensions {
        width: None,
        height: None,
    });

    let trigger_ref = NodeRef::new();
    let content_ref = NodeRef::new();
    view! {
        <Provider value=CollapsibleContext {
            open,
            state,
            dimensions,
            trigger_ref,
            content_ref
        }>
            <div class=class>
                {
                    children()
                }
            </div>
        </Provider>
    }
}

#[component]
pub fn CollapsibleTrigger(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let CollapsibleContext {
        trigger_ref, open, ..
    } = use_collapsible_context();
    view! {
        <div class=class node_ref=trigger_ref data-panel-open=move || open.get() on:click=move |_| {
            open.update(|open| *open = !*open);
        } >
            {children()}
        </div>
    }
}

#[component]
pub fn CollapsiblePanel(
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let CollapsibleContext {
        content_ref,
        state,
        open,
        dimensions, // Keep dimensions for tracking actual scroll height
        ..
    } = use_collapsible_context();

    // New signal to control the explicit pixel height used in CSS for animation
    // Starts at 0px conceptually when component is not mounted or fully closed.
    let panel_height_for_css: RwSignal<Option<i32>> = RwSignal::new(Some(0));

    Effect::new(move |_| {
        let current_status = state.transition_status.get();
        let current_open = open.get();

        if let Some(content) = content_ref.get() {
            let measured_height = content.scroll_height();
            let measured_width = content.scroll_width();

            let new_dims = Dimensions {
                width: Some(measured_width),
                height: Some(measured_height),
            };

            // Always update 'dimensions' with the actual scroll height when content is mounted.
            // This ensures 'dimensions' always holds the latest 'from' height for closing.
            if dimensions.get_untracked() != new_dims {
                dimensions.set(new_dims);
            }

            match current_status {
                TransitionStatus::Starting => {
                    // When opening, set CSS height to measured height immediately.
                    // The CSS transition will animate from 0 (Undefined) to this value.
                    panel_height_for_css.set(Some(measured_height));
                }
                TransitionStatus::Idle => {
                    // When fully open, set CSS height to 'auto' for content reflow.
                    panel_height_for_css.set(None);
                }
                TransitionStatus::Ending => {
                    // CRITICAL FOR CLOSING ANIMATION:
                    // 1. Immediately set CSS height to the last measured height (from 'dimensions').
                    //    This establishes the "from" point for the animation from 'auto'.
                    // 2. In the next microtask/frame, set CSS height to 0 to trigger the animation.
                    #[cfg(not(feature = "ssr"))]
                    {
                        if let Some(prev_height) = dimensions.get_untracked().height {
                            panel_height_for_css.set(Some(prev_height)); // Step 1: Set to measured height
                            let panel_height_setter = panel_height_for_css;
                            let timeout_handle = set_timeout_with_handle(
                                move || {
                                    panel_height_setter.set(Some(0)); // Step 2: Animate to 0
                                },
                                std::time::Duration::from_millis(0), // Next event loop tick
                            )
                            .expect("Failed to set timeout for closing animation target");
                            on_cleanup(move || {
                                timeout_handle.clear();
                            });
                        } else {
                            // Fallback if height somehow wasn't measured (e.g., content_ref was not ready)
                            panel_height_for_css.set(Some(0));
                        }
                    }
                }
                TransitionStatus::Undefined => {
                    // When fully closed or not yet opened, CSS height should be 0.
                    panel_height_for_css.set(Some(0));
                }
            }
        } else {
            // content_ref not available (e.g., not mounted), ensure height is 0.
            panel_height_for_css.set(Some(0));
            // Keep dimensions as 0 when not mounted/visible.
            let new_dims = Dimensions {
                width: Some(0),
                height: Some(0),
            };
            if dimensions.get_untracked() != new_dims {
                dimensions.set(new_dims);
            }
        }
    });

    view! {
        <Show when=move || state.mounted.get()>
            <div class=class node_ref=content_ref
                data-open=move || open.get()
                data-state=move || {
                    // Provide more semantic states for CSS styling
                    match state.transition_status.get() {
                        TransitionStatus::Starting => "opening",
                        TransitionStatus::Ending => "closing",
                        TransitionStatus::Idle => "open",
                        TransitionStatus::Undefined => "closed", // Conceptual state when unmounted
                    }
                }
                style=move || {
                    // Use dimensions for width, as it doesn't have the 'auto' to 'Xpx' issue for transitions
                    let width = dimensions.get().width;
                    let width_val = width.map(|w| format!("{w}px")).unwrap_or("auto".into());

                    // Use panel_height_for_css for the height variable
                    let height_val = match panel_height_for_css.get() {
                        Some(h) => format!("{h}px"),
                        None => "auto".to_string(),
                    };

                    format!(
                        "--collapsible-panel-height: {height_val}; --collapsible-panel-width: {width_val};"
                    )
                }
            >
                {children()}
            </div>
        </Show>
    }
}

pub fn use_transition_status(
    open: Signal<bool>,
    enable_idle_state: bool,
    defer_ending_state: bool,
    open_duration: u64,
    close_duration: u64,
) -> TransitionStatusState {
    // Always start in Undefined, allow effects to transition to Idle.
    let transition_status: RwSignal<TransitionStatus> = RwSignal::new(TransitionStatus::Undefined);
    // `mounted` controls whether the dialog content (Portal) is in the DOM.
    // It should be true when the dialog is in any active transition state (Starting, Ending, Idle),
    // and false when it's Undefined (i.e., fully closed and removed from DOM).
    let mounted: RwSignal<bool> = RwSignal::new(false);

    Effect::new(move |_| {
        let current_open = open.get();
        let current_status = transition_status.get();

        if current_open || current_status == TransitionStatus::Ending {
            if !mounted.get_untracked() {
                mounted.set(true);
            }
        } else if !current_open
            && current_status == TransitionStatus::Undefined
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
            && (current_status == TransitionStatus::Undefined
                || current_status == TransitionStatus::Ending)
        {
            // Delay setting 'Starting' using a 0ms timeout to ensure the DOM renders
            // the initial state before applying the 'Starting' state for animation.
            #[cfg(not(feature = "ssr"))]
            {
                let transition_status_setter = transition_status; // Capture RwSignal
                let timeout_handle = set_timeout_with_handle(
                    move || {
                        transition_status_setter.set(TransitionStatus::Starting);
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
        if current_open && current_status == TransitionStatus::Starting && enable_idle_captured {
            #[cfg(not(feature = "ssr"))]
            {
                let transition_status_setter = transition_status;
                let timeout_handle = set_timeout_with_handle(
                    move || {
                        transition_status_setter.set(TransitionStatus::Idle);
                    },
                    std::time::Duration::from_millis(open_duration),
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
            && current_status != TransitionStatus::Ending
            && !defer_ending_state
        {
            transition_status.set(TransitionStatus::Ending);
        }
    });

    #[cfg(not(feature = "ssr"))]
    let ending = Rc::new(Closure::new(move || {
        transition_status.set(TransitionStatus::Ending);
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
                && status_val != TransitionStatus::Ending
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

        if !current_open && current_status == TransitionStatus::Ending {
            let transition_status_setter = transition_status;
            let timeout_handle = set_timeout_with_handle(
                move || {
                    transition_status_setter.set(TransitionStatus::Undefined);
                },
                std::time::Duration::from_millis(close_duration),
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
