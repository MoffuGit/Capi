use leptos::{html, prelude::*};
use leptos_use::{UseElementBoundingReturn, use_element_bounding};
use tailwind_fuse::tw_merge;

use super::{MenuAlign, MenuPositionReturn, MenuProviderContext, MenuSide, use_menu_position};
use crate::common::floating::{
    HoverAreaProvider, UseHoverHandlers, use_hover_area_item_handlers, use_is_hovering_area,
};
use crate::common::status::{TransitionStatusState, use_transition_status};
use crate::portal::Portal;
use leptos::context::Provider;

#[derive(Clone)]
pub struct SubMenuProviderContext {
    pub open: RwSignal<bool>,
    pub hidden: RwSignal<bool>,
    pub trigger_ref: NodeRef<html::Div>,
    pub trigger_width: RwSignal<f64>,
    pub trigger_height: RwSignal<f64>,
    pub trigger_x: RwSignal<f64>,
    pub trigger_y: RwSignal<f64>,
    pub content_ref: NodeRef<html::Div>,
    pub transition_status: TransitionStatusState,
    pub open_on_hover: RwSignal<bool>,
}

#[component]
pub fn SubMenuProvider(
    children: Children,
    #[prop(optional, into)] open: RwSignal<bool>,
    #[prop(optional, into)] hidden: RwSignal<bool>,
    #[prop(optional, into)] trigger_ref: NodeRef<html::Div>,
    #[prop(optional, into)] content_ref: NodeRef<html::Div>,
    #[prop(optional, into, default = RwSignal::new(true))] open_on_hover: RwSignal<bool>,
) -> impl IntoView {
    let transition_status = use_transition_status(open.into(), content_ref, true, true);

    view! {
        <Provider
            value=SubMenuProviderContext {
                open_on_hover,
                transition_status,
                open,
                hidden,
                trigger_ref,
                content_ref,
                trigger_width: RwSignal::new(0.0),
                trigger_height: RwSignal::new(0.0),
                trigger_x: RwSignal::new(0.0),
                trigger_y:  RwSignal::new(0.0),
            }
        >
            <HoverAreaProvider is_hovering=open timeout_duration_ms=0 enabled=open_on_hover>
                {children()}
            </HoverAreaProvider>
        </Provider>
    }
}

#[component]
pub fn SubMenuTrigger(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let context =
        use_context::<SubMenuProviderContext>().expect("should access the sub menu context");
    let trigger_ref = context.trigger_ref;

    #[cfg(feature = "hydrate")]
    {
        let UseElementBoundingReturn {
            width,
            height,
            x,
            y,
            update,
            ..
        } = use_element_bounding(trigger_ref);
        Effect::new(move |_| {
            context.trigger_width.set(width.get());
            context.trigger_height.set(height.get());
            context.trigger_x.set(x.get());
            context.trigger_y.set(y.get());
        });

        Effect::new(move |_| {
            if context.open.get() {
                update()
            }
        });
    }

    let UseHoverHandlers {
        on_pointer_enter,
        on_pointer_leave,
        open,
        ..
    } = use_hover_area_item_handlers();

    view! {
        <div
            on:pointerenter=move |evt| {
                on_pointer_enter.run(evt);
            }
            on:pointerleave=move |evt| {
                on_pointer_leave.run(evt);
            }
            class=move || {
                tw_merge!(class.get())
            }
            on:click=move |_| {
                if !context.open_on_hover.get() {
                    open.run(())
                }
            }
            node_ref=trigger_ref
            data-state=move || context.transition_status.transition_status.get().to_string()
        >
            {children.map(|children| children())}
        </div>
    }
}

#[component]
pub fn SubMenuPortal(children: ChildrenFn) -> impl IntoView {
    let children = StoredValue::new(children);
    let context: SubMenuProviderContext =
        use_context().expect("should access the sub menu context");
    let mounted = context.transition_status.mounted;
    view! {
        <Show when=move || mounted.get()>
            <Portal>
                    {children.get_value()()}
            </Portal>
        </Show>
    }
}

#[component]
pub fn SubMenuContent(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(into, optional, default = Signal::derive(|| MenuSide::Right))] side: Signal<MenuSide>,
    #[prop(into,optional, default = Signal::derive(|| 0.0))] side_of_set: Signal<f64>,
    #[prop(into,optional, default = Signal::derive(|| MenuAlign::Start))] align: Signal<MenuAlign>,
    #[prop(into,optional, default = Signal::derive(|| 0.0))] align_of_set: Signal<f64>,
    #[prop(into, default = None)] limit_y: Option<Signal<f64>>,
    #[prop(optional)] arrow: bool,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context =
        use_context::<SubMenuProviderContext>().expect("should access the sub menu context");
    let content_ref = context.content_ref;

    let mount_ref = NodeRef::new();

    let MenuPositionReturn { x, y } = use_menu_position(
        mount_ref,
        context.trigger_width,
        context.trigger_height,
        context.trigger_x,
        context.trigger_y,
        side,
        side_of_set,
        align,
        align_of_set,
    );

    let y_position = move || {
        if limit_y.is_some_and(|limit| limit.get() < y.get()) {
            limit_y.unwrap().get()
        } else {
            y.get()
        }
    };

    let transition_status_state = context.transition_status;

    let arrow = move || {
        if arrow {
            match side.get() {
                MenuSide::Bottom => {
                    "after:content-[' '] after:absolute after:bottom-[100%] after:left-[50%] after:ml-[-3px] after:border-[3px] after:border-solid after:border-transparent after:border-b-inherit"
                }
                MenuSide::Right => {
                    "after:content-[' '] after:absolute after:right-[100%] after:top-[50%] after:mt-[-3px] after:border-[3px] after:border-solid after:border-transparent after:border-r-inherit"
                }
                MenuSide::Left => {
                    "after:content-[' '] after:absolute after:left-[100%] after:top-[50%] after:mt-[-3px] after:border-[3px] after:border-solid after:border-transparent after:border-l-inherit"
                }
                MenuSide::Top => {
                    "after:content-[' '] after:absolute after:top-[100%] after:left-[50%] after:ml-[-3px] after:border-[3px] after:border-solid after:border-transparent after:border-t-inherit"
                }
            }
        } else {
            ""
        }
    };

    let position = Signal::derive(move || format!("translate: {}px {}px;", x.get(), y_position()));

    let helper = Signal::derive(move || {
        format!(
            "--radix-menu-content-transform-origin: {}",
            match side.get() {
                MenuSide::Bottom => "bottom",
                MenuSide::Left => "left",
                MenuSide::Right => "right",
                MenuSide::Top => "top",
            }
        )
    });

    let UseHoverHandlers {
        on_pointer_enter,
        on_pointer_leave,
        ..
    } = use_hover_area_item_handlers();

    view! {
        <div
            style=move || format!("{}; {}", position.get(), helper.get())
            style:visibility=move || if context.hidden.get() {
                "hidden"
            } else {
                "visible"
            }
            class=format!("absolute z-50 left-0 top-0")
            node_ref=mount_ref
            on:pointerenter=move |evt| {
                on_pointer_enter.run(evt);
            }
            on:pointerleave=move |evt| {
                on_pointer_leave.run(evt);
            }
            data-state=move || transition_status_state.transition_status.get().to_string()
        >
            <div
                data-side=move || match side.get() {
                    MenuSide::Bottom => "bottom",
                    MenuSide::Left => "left",
                    MenuSide::Right => "right",
                    MenuSide::Top => "top",
                }
                node_ref=content_ref
                data-state=move || transition_status_state.transition_status.get().to_string()
                class=move || {
                    tw_merge!(
                        arrow(),
                        class.get()
                    )
                }
            >
                {children.get_value().map(|children| children())}
            </div>
        </div>
    }
}
