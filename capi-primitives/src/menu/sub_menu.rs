use leptos::{html, prelude::*};
use leptos_use::{UseElementBoundingReturn, on_click_outside_with_options, use_element_bounding};
use tailwind_fuse::tw_merge;

use super::{MenuAlign, MenuPositionReturn, MenuProviderContext, MenuSide, use_menu_position};
use crate::common::floating::{
    HoverAreaProvider, use_hover_area_item_handlers, use_is_hovering_area,
};
use crate::common::status::{TransitionStatusState, use_transition_status}; // Import TransitionStatusState
use crate::portal::Portal;
use leptos::context::Provider; // Add this import

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
    // New field to enable/disable hover logic for the HoverAreaProvider
    pub open_on_hover_enabled: RwSignal<bool>,
}

#[component]
pub fn SubMenuProvider(
    children: Children,
    #[prop(optional, into)] open: RwSignal<bool>,
    #[prop(optional, into)] hidden: RwSignal<bool>,
    #[prop(optional, into)] trigger_ref: NodeRef<html::Div>,
    #[prop(optional, into)] content_ref: NodeRef<html::Div>,
) -> impl IntoView {
    let parent_menu_context =
        use_context::<MenuProviderContext>().expect("should acces the menu context");

    let open_on_hover_enabled = RwSignal::new(false);

    // Effect to close submenu when parent menu closes
    Effect::new(move |_| {
        if !parent_menu_context.open.get() {
            open.set(false);
        }
    });

    view! {
        <Provider
            value=SubMenuProviderContext {
                open,
                hidden,
                trigger_ref,
                content_ref,
                trigger_width: RwSignal::new(0.0),
                trigger_height: RwSignal::new(0.0),
                trigger_x: RwSignal::new(0.0),
                trigger_y:  RwSignal::new(0.0),
                open_on_hover_enabled,
            }
        >
            <HoverAreaProvider timeout_duration_ms=0 enabled=open_on_hover_enabled>
                {children()}
            </HoverAreaProvider>
        </Provider>
    }
}

#[component]
pub fn SubMenuTrigger(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] open_on_hover: bool, // This prop controls if hover logic applies
) -> impl IntoView {
    let context =
        use_context::<SubMenuProviderContext>().expect("should access the sub menu context");
    let open = context.open;
    let hidden = context.hidden;
    let trigger_ref = context.trigger_ref;

    // Propagate the `open_on_hover` prop value to the context for HoverAreaProvider
    Effect::new(move |_| {
        context.open_on_hover_enabled.set(open_on_hover);
    });

    let parent_menu_context =
        use_context::<MenuProviderContext>().expect("should acces the menu context");

    Effect::new(move |_| {
        if context.open.get() {
            let UseElementBoundingReturn {
                width,
                height,
                x,
                y,
                ..
            } = use_element_bounding(trigger_ref);
            context.trigger_width.set(width.get_untracked());
            context.trigger_height.set(height.get_untracked());
            context.trigger_x.set(x.get_untracked());
            context.trigger_y.set(y.get_untracked());
        }
    });

    let is_hovering_area = use_is_hovering_area();
    // Update the `open` state based on the debounced hover area state
    Effect::new(move |_| {
        if open_on_hover {
            // Only manage open state via hover if open_on_hover is true
            if is_hovering_area.get() {
                open.set(true);
            } else {
                open.set(false);
            }
        }
    });

    // Get hover handlers from the new reusable hook
    let (on_pointer_enter_handler, on_pointer_leave_handler) = use_hover_area_item_handlers();

    view! {
        <div
            on:pointerenter=on_pointer_enter_handler
            on:pointerleave=on_pointer_leave_handler
            class=move || {
                tw_merge!(class.get())
            }
            on:click=move |_| {
                // Only allow click if not opening on hover
                if !open_on_hover {
                    open.set(!open.get());
                    hidden.set(false);
                }
            }
            node_ref=trigger_ref
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
    let transition_status =
        use_transition_status(context.open.into(), context.content_ref, true, true);
    let mounted = transition_status.mounted;
    view! {
        <Show when=move || mounted.get()>
            <Provider value=transition_status>
                <Portal>
                        {children.get_value()()}
                </Portal>
            </Provider>
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
    #[prop(optional)] ignore: Vec<NodeRef<html::Div>>,
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

    let transition_status_state = use_context::<TransitionStatusState>()
        .expect("should access the transition context provided by SubMenuPortal");

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

    let (on_pointer_enter_handler, on_pointer_leave_handler) = use_hover_area_item_handlers();

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
            on:pointerenter=on_pointer_enter_handler
            on:pointerleave=on_pointer_leave_handler
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
