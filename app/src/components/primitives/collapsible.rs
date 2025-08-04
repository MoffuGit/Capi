use leptos::context::Provider;
use leptos::html::Div;
use leptos::prelude::*;

use crate::components::primitives::common::status::use_transition_status;

use super::common::status::{TransitionStatus, TransitionStatusState};

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
    // open_duration and close_duration are removed
) -> impl IntoView {
    let dimensions = RwSignal::new(Dimensions {
        width: None,
        height: None,
    });

    let trigger_ref = NodeRef::new();
    let content_ref = NodeRef::new();

    // Pass content_ref to use_transition_status to read the CSS duration
    let state = use_transition_status(open.into(), content_ref, true, true);

    view! {
        <Provider value=CollapsibleContext {
            open,
            state,
            dimensions,
            trigger_ref,
            content_ref
        }>
            {
                children()
            }
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
                TransitionStatus::Opening => {
                    // When opening, set CSS height to measured height immediately.
                    // The CSS transition will animate from 0 (Undefined) to this value.
                    panel_height_for_css.set(Some(measured_height));
                }
                TransitionStatus::Open => {
                    // When fully open, set CSS height to 'auto' for content reflow.
                    panel_height_for_css.set(None);
                }
                TransitionStatus::Closing => {
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
                TransitionStatus::Closed => {
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
                        TransitionStatus::Opening => "opening",
                        TransitionStatus::Closing => "closing",
                        TransitionStatus::Open => "open",
                        TransitionStatus::Closed => "closed", // Conceptual state when unmounted
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
