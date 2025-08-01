use leptos::context::Provider;
use leptos::html;
use leptos::{leptos_dom::helpers::TimeoutHandle, prelude::*};
use leptos_use::{use_element_bounding, UseElementBoundingReturn};
use std::time::Duration;
use tailwind_fuse::tw_merge;
use web_sys::PointerEvent;

use crate::components::primitives::common::status::{
    use_transition_status, TransitionStatus, TransitionStatusState,
};
use crate::components::primitives::portal::Portal;

#[derive(Clone)]
struct TooltipProviderContext {
    is_open: RwSignal<bool>,
    on_trigger_leave: Signal<()>,
    on_trigger_enter: Signal<()>,
    on_open: Signal<()>,
    on_close: Signal<()>,
    trigger_ref: NodeRef<html::Div>,
}

#[component]
pub fn ToolTipProvider(
    children: Children,
    #[prop(default = Duration::new(0,0))] delay_duration: Duration,
) -> impl IntoView {
    let was_open_delayed_ref = RwSignal::new(false);
    let is_open = RwSignal::new(false);
    let open_timer_ref: RwSignal<Option<TimeoutHandle>> = RwSignal::new(None);
    let trigger_ref = NodeRef::<html::Div>::new();

    let handle_open = move || match open_timer_ref.get_untracked() {
        None => {
            was_open_delayed_ref.update_untracked(|value| *value = false);
            is_open.update(|value| *value = true);
        }
        Some(timer) => {
            timer.clear();
            was_open_delayed_ref.update_untracked(|value| *value = false);
            is_open.update(|value| *value = true);
        }
    };

    let handle_close = move || match open_timer_ref.get_untracked() {
        None => {
            is_open.update(|value| *value = false);
        }
        Some(timer) => {
            timer.clear();
            is_open.update(|value| *value = false);
        }
    };

    let handle_delayed_open = move || match open_timer_ref.get_untracked() {
        None => {
            open_timer_ref.update_untracked(|value| {
                *value = set_timeout_with_handle(
                    move || {
                        was_open_delayed_ref.update_untracked(|value| *value = true);
                        is_open.update(|value| *value = true);
                    },
                    delay_duration,
                )
                .ok()
            });
        }
        Some(timer) => {
            timer.clear();
            open_timer_ref.update_untracked(|value| {
                *value = set_timeout_with_handle(
                    move || {
                        was_open_delayed_ref.update_untracked(|value| *value = true);
                        is_open.update(|value| *value = true);
                    },
                    delay_duration,
                )
                .ok()
            });
        }
    };

    let on_trigger_enter = Signal::derive(handle_delayed_open);
    let on_trigger_leave = Signal::derive(move || match open_timer_ref.get_untracked() {
        None => {
            handle_close();
        }
        Some(timer) => {
            handle_close();
            timer.clear();
        }
    });
    let on_open = Signal::derive(handle_open);
    let on_close = Signal::derive(handle_close);

    view! {
        <Provider value=TooltipProviderContext {
                    is_open,
                    on_trigger_leave,
                    on_trigger_enter,
                    on_open,
                    on_close,
                    trigger_ref,
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
) -> impl IntoView {
    let provider_context = use_context::<TooltipProviderContext>().expect("have this context");
    let is_hover = RwSignal::new(false);
    let trigger_ref = provider_context.trigger_ref;

    view! {
        <div
            node_ref={trigger_ref}
            class=class
            on:pointermove=move |evt: PointerEvent| {
                if evt.pointer_type() == "touch" {
                    return;
                }
                if !is_hover.get_untracked() {
                    provider_context.on_trigger_enter.get_untracked();
                    is_hover.update_untracked(|value| *value = true)
                }
            }
            on:pointerleave=move |_| {
                provider_context.on_trigger_leave.get_untracked();
                is_hover.update_untracked(|value| *value = false)
            }
            on:click=move |_evt| {
                if close_on_click {
                    provider_context.on_close.get_untracked();
                }
                if let Some(on_click) = on_click {
                    on_click.run(())
                }
            }
            on:wheel=move |_| {
                provider_context.on_close.get_untracked();
            }
            on:focus=move |_| {
                provider_context.on_open.get_untracked();
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
pub fn ToolTipPortal(
    children: ChildrenFn,
    #[prop(optional)] open_duration: u64,
    #[prop(optional, default = 100)] close_duration: u64,
) -> impl IntoView {
    let context = use_context::<TooltipProviderContext>().expect("is open context");

    let is_open = context.is_open;

    let children = StoredValue::new(children);

    let state = use_transition_status(
        is_open.read_only(),
        true,
        true,
        open_duration,
        close_duration,
    );
    view! {
        <Show when=move || state.mounted.get()>
            <Provider value=state>
                <Portal>
                    {children.get_value()()}
                </Portal>
            </Provider>
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

    let content_ref = NodeRef::<html::Div>::new();

    let position = RwSignal::new(("".to_string(), "".to_string()));

    let transition_status =
        use_context::<TransitionStatusState>().expect("should acces the transition context");

    let UseElementBoundingReturn {
        width: trigger_width,
        height: trigger_height,
        x: trigger_x,
        y: trigger_y,
        ..
    } = use_element_bounding(trigger_ref);

    let UseElementBoundingReturn {
        width: content_width,
        height: content_height,
        ..
    } = use_element_bounding(content_ref);

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
            ToolTipSide::Bottom => "after:content-[' '] after:absolute after:bottom-[100%] after:left-[50%] after:ml-[-3px] after:border-[3px] after:border-solid after:border-transparent after:border-b-inherit",
            ToolTipSide::Right => "after:content-[' '] after:absolute after:right-[100%] after:top-[50%] after:mt-[-3px] after:border-[3px] after:border-solid after:border-transparent after:border-r-inherit",
            ToolTipSide::Left => "after:content-[' '] after:absolute after:left-[100%] after:top-[50%] after:mt-[-3px] after:border-[3px] after:border-solid after:border-transparent after:border-l-inherit",
            ToolTipSide::Top => "after:content-[' '] after:absolute after:top-[100%] after:left-[50%] after:ml-[-3px] after:border-[3px] after:border-solid after:border-transparent after:border-t-inherit",
        }
    } else {
        ""
    };

    let children = StoredValue::new(children);

    view! {
        <div
            data-state=move || {
                match transition_status.transition_status.get() {
                    TransitionStatus::Starting => "open",
                    TransitionStatus::Ending => "closed",
                    TransitionStatus::Idle => "",
                    TransitionStatus::Undefined => "undefined",
                }
            }
            node_ref=content_ref
            style:position="absolute"
            style:left=move || format!("{}px", position().0)
            style:top=move || format!("{}px",  position().1)
            style=move || format!("--radix-tooltip-content-transform-origin: {}", match tooltip_side {
                ToolTipSide::Bottom => "bottom",
                ToolTipSide::Left => "left",
                ToolTipSide::Right => "right",
                ToolTipSide::Top => "top",
            })
            class=format!("absolute z-50 left-0 top-0 font-normal")
        >
            <div
                data-side=match tooltip_side {
                    ToolTipSide::Bottom => "bottom",
                    ToolTipSide::Left => "left",
                    ToolTipSide::Right => "right",
                    ToolTipSide::Top => "top",
                }
                data-state=move || {
                    match transition_status.transition_status.get() {
                        TransitionStatus::Starting => "open",
                        TransitionStatus::Ending => "closed",
                        TransitionStatus::Idle => "",
                        TransitionStatus::Undefined => "undefined",
                    }
                }
                class=move || tw_merge!(
                class.get(),
                arrow,
            )>{children.get_value()()}</div>
        </div>
    }
}
