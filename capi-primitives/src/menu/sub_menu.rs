use leptos::either::Either;
use leptos::{html, prelude::*};
use tailwind_fuse::tw_merge;

use super::{MenuAlign, MenuSide};
use crate::common::floating::{FloatingPosition, use_floating};
use crate::common::hover::{HoverAreaProvider, UseHoverHandlers, use_hover_area_item_handlers};
use crate::common::status::use_transition_status;
use crate::menu::MenuProviderContext;
use crate::portal::Portal;
use leptos::context::Provider;

#[component]
pub fn SubMenuProvider(
    children: Children,
    #[prop(optional, into)] open: RwSignal<bool>,
    #[prop(optional, into)] trigger_ref: NodeRef<html::Div>,
    #[prop(optional, into)] content_ref: NodeRef<html::Div>,
    #[prop(optional, into, default = RwSignal::new(true))] open_on_hover: RwSignal<bool>,
) -> impl IntoView {
    let transition_status = use_transition_status(open.into(), content_ref, true, true);

    let mount_ref = NodeRef::new();

    let floating = use_floating(trigger_ref, mount_ref);

    view! {
        <Provider
            value=MenuProviderContext {
                transition_status,
                open,
                trigger_ref,
                content_ref,
                mount_ref,
                dismissible: true,
                modal: true,
                floating
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
    let context = use_context::<MenuProviderContext>().expect("should access the sub menu context");
    let trigger_ref = context.trigger_ref;

    let UseHoverHandlers {
        on_pointer_enter,
        on_pointer_leave,
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
    let context: MenuProviderContext = use_context().expect("should access the sub menu context");
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
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = use_context::<MenuProviderContext>().expect("should access the sub menu context");
    let content_ref = context.content_ref;

    let mount_ref = NodeRef::new();

    let FloatingPosition { x, y, .. } =
        context
            .floating
            .get_floating_position(side, side_of_set, align, align_of_set, None);

    let transition_status_state = context.transition_status;

    let UseHoverHandlers {
        on_pointer_enter,
        on_pointer_leave,
        ..
    } = use_hover_area_item_handlers();

    view! {
        <div
            style:position="absolute"
            style:left=move || format!("{}px", x())
            style:top=move || format!("{}px",  y())
            style=move || format!("--radix-menu-content-transform-origin: {}", side())
            class="z-50"
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
                data-side=move || side.get().to_string()
                node_ref=content_ref
                data-state=move || transition_status_state.transition_status.get().to_string()
                class=class
            >
                {if let Some(children) = children.get_value() {
                    Either::Left(children())
                } else {
                    Either::Right(())
                }}
            </div>
        </div>
    }
}
