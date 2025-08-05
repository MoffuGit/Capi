use leptos::context::Provider;
use leptos::html::Div;
use leptos::prelude::*;

use crate::common::status::use_transition_status;

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
        dimensions,
        ..
    } = use_collapsible_context();

    let panel_height_for_css: RwSignal<Option<i32>> = RwSignal::new(None);

    Effect::new(move |_| {
        let current_status = state.transition_status.get();

        if let Some(content) = content_ref.get() {
            let measured_height = content.scroll_height();
            let measured_width = content.scroll_width();

            let new_dims = Dimensions {
                width: Some(measured_width),
                height: Some(measured_height),
            };

            if dimensions.get_untracked() != new_dims {
                dimensions.set(new_dims);
            }

            match current_status {
                TransitionStatus::Opening => {
                    panel_height_for_css.set(Some(measured_height));
                }
                TransitionStatus::Open => {
                    panel_height_for_css.set(None);
                }
                TransitionStatus::Closing => {
                    #[cfg(not(feature = "ssr"))]
                    {
                        if let Some(prev_height) = dimensions.get_untracked().height {
                            panel_height_for_css.set(Some(prev_height)); // Step 1: Set to measured height
                            let panel_height_setter = panel_height_for_css;
                            let timeout_handle = set_timeout_with_handle(
                                move || {
                                    panel_height_setter.set(Some(0));
                                },
                                std::time::Duration::from_millis(0),
                            )
                            .expect("Failed to set timeout for closing animation target");
                            on_cleanup(move || {
                                timeout_handle.clear();
                            });
                        } else {
                            panel_height_for_css.set(Some(0));
                        }
                    }
                }
                TransitionStatus::Closed => {
                    panel_height_for_css.set(Some(0));
                }
            }
        }
    });

    view! {
        <Show when=move || state.mounted.get()>
            <div class=class node_ref=content_ref
                data-open=move || open.get()
                data-state=move || {
                    match state.transition_status.get() {
                        TransitionStatus::Opening => "opening",
                        TransitionStatus::Closing => "closing",
                        TransitionStatus::Open => "open",
                        TransitionStatus::Closed => "closed",
                    }
                }
                style=move || {
                    let width = dimensions.get().width;
                    let width_val = width.map(|w| format!("{w}px")).unwrap_or("auto".into());

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
