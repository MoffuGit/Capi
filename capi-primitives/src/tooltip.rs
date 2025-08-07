use leptos::context::Provider;
use leptos::html;
use leptos::prelude::*;
use leptos_use::{UseElementBoundingReturn, use_element_bounding};
use tailwind_fuse::tw_merge;

use crate::common::floating::{
    HoverAreaProvider, UseHoverHandlers, use_hover_area_item_handlers, use_is_hovering_area,
};
use crate::common::status::{TransitionStatus, TransitionStatusState, use_transition_status};
use crate::portal::Portal;

#[derive(Clone)]
struct TooltipProviderContext {
    is_open: RwSignal<bool>,
    trigger_ref: NodeRef<html::Div>,
    content_ref: NodeRef<html::Div>,
    transition_state: TransitionStatusState,
}

#[component]
pub fn ToolTipProvider(
    children: Children,
    #[prop(default = 0)] delay_duration: u64,
) -> impl IntoView {
    let is_open = RwSignal::new(false);
    let trigger_ref = NodeRef::<html::Div>::new();

    let content_ref = NodeRef::<html::Div>::new();

    let transition_state = use_transition_status(is_open.into(), content_ref, true, true);

    view! {
        <Provider
            value=TooltipProviderContext {
                transition_state,
                is_open,
                trigger_ref,
                content_ref
            }
        >
            <HoverAreaProvider is_hovering=is_open timeout_duration_ms=delay_duration enabled=RwSignal::new(true)>
                {children()}
            </HoverAreaProvider>
        </Provider>
    }
}

#[component]
pub fn ToolTipTrigger(
    children: ChildrenFn,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, default = true)] close_on_click: bool,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let TooltipProviderContext {
        trigger_ref,
        transition_state,
        ..
    } = use_context::<TooltipProviderContext>().expect("have this context");

    let UseHoverHandlers {
        on_pointer_enter,
        on_pointer_leave,
        close,
        open,
    } = use_hover_area_item_handlers();

    let is_hovering = RwSignal::new(false);

    view! {
        <div
            data-state=move || transition_state.transition_status.get().to_string()
            node_ref=trigger_ref
            class=class
            on:pointerenter=move |evt| {
                if !is_hovering.get() && transition_state.transition_status.get() != TransitionStatus::Closing {
                    on_pointer_enter.run(evt);
                }
                is_hovering.set(true);
            }
            on:pointerleave=move |evt| {
                on_pointer_leave.run(evt);
                is_hovering.set(false);
            }
            on:click=move |_evt| {
                if close_on_click {
                    close.run(());
                }
                if let Some(on_click) = on_click {
                    on_click.run(())
                }
            }
            on:wheel=move |_| {
                close.run(());
            }
            on:focus=move |_| {
                open.run(())
            }
        >
            {children()}
        </div>
    }
}

#[derive(Clone, Copy)]
pub enum ToolTipSide {
    Bottom,
    Left,
    Right,
    Top,
}

pub fn get_tooltip_position(
    trigger_x: f64,
    trigger_y: f64,
    trigger_width: f64,
    trigger_height: f64,
    content_width: f64,
    content_height: f64,
    tooltip_side: ToolTipSide,
    tooltip_of_side: f64,
) -> (String, String) {
    match tooltip_side {
        ToolTipSide::Bottom => {
            let y = trigger_y + trigger_height + tooltip_of_side;
            let x = trigger_x + (trigger_width / 2.0) - (content_width / 2.0);
            (x.to_string(), y.to_string())
        }
        ToolTipSide::Left => {
            let y = trigger_y + (trigger_height / 2.0) - (content_height / 2.0);
            let x = trigger_x - content_width - tooltip_of_side;
            (x.to_string(), y.to_string())
        }
        ToolTipSide::Right => {
            let y = trigger_y + (trigger_height / 2.0) - (content_height / 2.0);
            let x = trigger_x + trigger_width + tooltip_of_side;
            (x.to_string(), y.to_string())
        }
        ToolTipSide::Top => {
            let y = trigger_y - content_height - tooltip_of_side;
            let x = trigger_x + (trigger_width / 2.0) - (content_width / 2.0);
            (x.to_string(), y.to_string())
        }
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
    #[prop(optional, default = ToolTipSide::Right)] tooltip_side: ToolTipSide,
    #[prop(optional, default = 2.0)] tooltip_of_side: f64,
    #[prop(optional, default = false)] arrow: bool,
    children: ChildrenFn,
) -> impl IntoView {
    let context = use_context::<TooltipProviderContext>().expect("is open context");
    let trigger_ref = context.trigger_ref;

    let content_ref = context.content_ref;

    let position = RwSignal::new(("".to_string(), "".to_string()));

    let transition_status = context.transition_state;

    let UseElementBoundingReturn {
        width: trigger_width,
        height: trigger_height,
        x: trigger_x,
        y: trigger_y,
        ..
    } = use_element_bounding(trigger_ref);

    let mount_ref = NodeRef::new();

    let UseElementBoundingReturn {
        width: content_width,
        height: content_height,
        ..
    } = use_element_bounding(mount_ref);

    Effect::new(move |_| {
        if transition_status.mounted.get() {
            position.set(get_tooltip_position(
                trigger_x.get(),
                trigger_y.get(),
                trigger_width.get(),
                trigger_height.get(),
                content_width.get(),
                content_height.get(),
                tooltip_side,
                tooltip_of_side,
            ));
        } else {
            position.set(("".to_string(), "".to_string()));
        }
    });

    let arrow = if arrow {
        match tooltip_side {
            ToolTipSide::Bottom => {
                "after:content-[' '] after:absolute after:bottom-[100%] after:left-[50%] after:ml-[-3px] after:border-[3px] after:border-solid after:border-transparent after:border-b-inherit"
            }
            ToolTipSide::Right => {
                "after:content-[' '] after:absolute after:right-[100%] after:top-[50%] after:mt-[-3px] after:border-[3px] after:border-solid after:border-transparent after:border-r-inherit"
            }
            ToolTipSide::Left => {
                "after:content-[' '] after:absolute after:left-[100%] after:top-[50%] after:mt-[-3px] after:border-[3px] after:border-solid after:border-transparent after:border-l-inherit"
            }
            ToolTipSide::Top => {
                "after:content-[' '] after:absolute after:top-[100%] after:left-[50%] after:ml-[-3px] after:border-[3px] after:border-solid after:border-transparent after:border-t-inherit"
            }
        }
    } else {
        ""
    };

    let children = StoredValue::new(children);

    let UseHoverHandlers {
        on_pointer_enter,
        on_pointer_leave,
        ..
    } = use_hover_area_item_handlers();

    view! {
        <div
            data-state=move || transition_status.transition_status.get().to_string()
            node_ref=mount_ref
            style:position="absolute"
            style:left=move || format!("{}px", position().0)
            style:top=move || format!("{}px",  position().1)
            style=move || format!("--radix-tooltip-content-transform-origin: {}", match tooltip_side {
                ToolTipSide::Bottom => "bottom",
                ToolTipSide::Left => "left",
                ToolTipSide::Right => "right",
                ToolTipSide::Top => "top",
            })
            on:pointerenter=move |evt| {
                if transition_status.transition_status.get() != TransitionStatus::Closing {
                    on_pointer_enter.run(evt);
                }
            }
            on:pointerleave=move |evt| {
                on_pointer_leave.run(evt);
            }
            class=format!("absolute z-50 left-0 top-0 font-normal")
        >
            <div
                node_ref=content_ref
                data-side=match tooltip_side {
                    ToolTipSide::Bottom => "bottom",
                    ToolTipSide::Left => "left",
                    ToolTipSide::Right => "right",
                    ToolTipSide::Top => "top",
                }
                data-state=move || transition_status.transition_status.get().to_string()
                class=move || tw_merge!(
                class.get(),
                arrow,
            )>{children.get_value()()}</div>
        </div>
    }
}
