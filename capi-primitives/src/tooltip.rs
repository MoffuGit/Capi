use leptos::context::Provider;
use leptos::html;
use leptos::prelude::*;
use tailwind_fuse::tw_merge;

pub use crate::common::Align as ToolTipAlign;
pub use crate::common::Side as ToolTipSide;
use crate::common::floating::FloatingContext;
use crate::common::floating::FloatingPosition;
use crate::common::floating::use_floating;
use crate::common::floating::use_position;
use crate::common::hover::use_hover;
use crate::common::status::{TransitionStatusState, use_transition_status};
use crate::portal::Portal;

#[derive(Clone)]
struct TooltipProviderContext {
    open: RwSignal<bool>,
    trigger_ref: NodeRef<html::Div>,
    content_ref: NodeRef<html::Div>,
    transition_state: TransitionStatusState,
    floating: FloatingContext,
    hoverable: Signal<bool>,
}

#[component]
pub fn ToolTipProvider(
    children: Children,
    #[prop(optional, into)] hoverable: Signal<bool>,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let trigger_ref = NodeRef::<html::Div>::new();

    let content_ref = NodeRef::<html::Div>::new();

    let transition_state = use_transition_status(open.into(), content_ref, true, true);

    let floating = use_floating(trigger_ref, content_ref, open);

    view! {
        <Provider
            value=TooltipProviderContext {
                hoverable,
                transition_state,
                open,
                trigger_ref,
                content_ref,
                floating
            }
        >
            {children()}
        </Provider>
    }
}

#[component]
pub fn ToolTipTrigger(
    children: ChildrenFn,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, default = true)] close_on_click: bool,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
    #[prop(default = 0)] delay_duration: u64,
    #[prop(optional, default = Signal::derive(move || true))] enabled: Signal<bool>,
) -> impl IntoView {
    let TooltipProviderContext {
        trigger_ref,
        transition_state,
        floating,
        hoverable,
        ..
    } = use_context::<TooltipProviderContext>().expect("have this context");

    use_hover(&floating, delay_duration, 0, enabled, hoverable);

    view! {
        <div
            data-state=move || transition_state.transition_status.get().to_string()
            node_ref=trigger_ref
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ToolTipPortal(children: ChildrenFn) -> impl IntoView {
    let context = use_context::<TooltipProviderContext>().expect("is open context");

    let children = StoredValue::new(children);

    let state = context.transition_state;
    view! {
        <Show when=move || state.mounted.get()>
            <Portal>
                {children.get_value()()}
            </Portal>
        </Show>
    }
}

#[component]
pub fn ToolTipContent(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] side: Signal<ToolTipSide>,
    #[prop(optional, into)] side_of_set: Signal<f64>,
    #[prop(optional, into)] align: Signal<ToolTipAlign>,
    #[prop(optional, into)] align_of_set: Signal<f64>,
    #[prop(optional, into)] arrow: bool,
    children: ChildrenFn,
) -> impl IntoView {
    let context = use_context::<TooltipProviderContext>().expect("is open context");

    let content_ref = context.content_ref;

    let transition_status = context.transition_state;

    let mount_ref = NodeRef::new();

    let children = StoredValue::new(children);

    let FloatingPosition {
        x,
        y,
        transform_origin,
        ..
    } = use_position(
        &context.floating,
        side,
        side_of_set,
        align,
        align_of_set,
        None,
    );

    view! {
        <div
            data-state=move || transition_status.transition_status.get().to_string()
            node_ref=mount_ref
            style:position="absolute"
            style:left=move || format!("{}px", x())
            style:top=move || format!("{}px",  y())
            style=move || format!("--radix-tooltip-content-transform-origin: {}", transform_origin())
            class=format!("absolute z-50 left-0 top-0 font-normal")
        >
            <div
                node_ref=content_ref
                data-side=side.get().to_string()
                data-state=move || transition_status.transition_status.get().to_string()
                class=move || tw_merge!(
                class.get(),
            )>{children.get_value()()}</div>
        </div>
    }
}
