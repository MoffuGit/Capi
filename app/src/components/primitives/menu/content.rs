use leptos::{html, prelude::*};
use leptos_use::{use_element_bounding, UseElementBoundingReturn};
use tailwind_fuse::tw_merge;

use crate::components::primitives::common::status::{TransitionStatus, TransitionStatusState};
use crate::components::primitives::menu::MenuProviderContext;

#[derive(Clone, Copy)]
pub enum MenuSide {
    Bottom,
    Left,
    Right,
    Top,
}

#[derive(Clone)]
pub enum MenuAlign {
    Start,
    Center,
    End,
}

#[component]
pub fn MenuContent(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(into, optional, default = Signal::derive(|| MenuSide::Bottom))] side: Signal<MenuSide>,
    #[prop(into,optional, default = Signal::derive(|| 0.0))] side_of_set: Signal<f64>,
    #[prop(into,optional, default = Signal::derive(|| MenuAlign::Center))] align: Signal<MenuAlign>,
    #[prop(into,optional, default = Signal::derive(|| 0.0))] align_of_set: Signal<f64>,
    #[prop(into, default = None)] limit_y: Option<Signal<f64>>,
    #[prop(optional)] ignore: Vec<NodeRef<html::Div>>,
    #[prop(optional)] arrow: bool,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = use_context::<MenuProviderContext>().expect("acces to menu context");
    let content_ref = context.content_ref;

    Effect::new(move |_| {
        if context.modal {
            if let Some(app) = document().get_element_by_id("app") {
                if context.open.get() {
                    let _ = app.class_list().add_1("pointer-events-none");
                } else {
                    let _ = app.class_list().remove_1("pointer-events-none");
                }
            }
        }
    });
    #[cfg(feature = "hydrate")]
    {
        use leptos_use::{on_click_outside_with_options, OnClickOutsideOptions};

        let _ = on_click_outside_with_options(
            content_ref,
            move |_| {
                if context.open.get() {
                    context.open.set(false)
                }
            },
            OnClickOutsideOptions::default().ignore(ignore),
        );
    }

    let MenuPositionReturn { x, y } = use_menu_position(
        content_ref,
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

    let transition_status =
        use_context::<TransitionStatusState>().expect("should acces the transition context");

    let arrow = move || {
        if arrow {
            match side.get() {
            MenuSide::Bottom => "after:content-[' '] after:absolute after:bottom-[100%] after:left-[50%] after:ml-[-3px] after:border-[3px] after:border-solid after:border-transparent after:border-b-inherit",
            MenuSide::Right => "after:content-[' '] after:absolute after:right-[100%] after:top-[50%] after:mt-[-3px] after:border-[3px] after:border-solid after:border-transparent after:border-r-inherit",
            MenuSide::Left => "after:content-[' '] after:absolute after:left-[100%] after:top-[50%] after:mt-[-3px] after:border-[3px] after:border-solid after:border-transparent after:border-l-inherit",
            MenuSide::Top => "after:content-[' '] after:absolute after:top-[100%] after:left-[50%] after:ml-[-3px] after:border-[3px] after:border-solid after:border-transparent after:border-t-inherit",
        }
        } else {
            ""
        }
    };

    let position = Signal::derive(move || format!("translate: {}px {}px;", x.get(), y_position()));

    let helper = Signal::derive(move || {
        format!(
            "--radix-menu-content-transform-origin: {}px {}px",
            x.get(),
            y_position()
        )
    });

    view! {
        <div
            style=move || format!("{}; {}", position.get(), helper.get())
            style:visibility=move || if context.hidden.get() {
                "hidden"
            } else {
                "visible"
            }
            class=format!("absolute z-50 left-0 top-0")
            node_ref=content_ref
            data-state=move || {
                match transition_status.transition_status.get() {
                    TransitionStatus::Starting => "open",
                    TransitionStatus::Ending => "closed",
                    TransitionStatus::Idle => "",
                    TransitionStatus::Undefined => "undefined",
                }
            }
        >
            <div
                data-side=move || match side.get() {
                    MenuSide::Bottom => "bottom",
                    MenuSide::Left => "left",
                    MenuSide::Right => "right",
                    MenuSide::Top => "top",
                }
                data-state=move || {
                    match transition_status.transition_status.get() {
                        TransitionStatus::Starting => "open",
                        TransitionStatus::Ending => "closed",
                        TransitionStatus::Idle => "",
                        TransitionStatus::Undefined => "undefined",
                    }
                }
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

#[derive(Debug, PartialEq)]
pub struct MenuPositionReturn {
    x: Memo<f64>,
    y: Memo<f64>,
}

fn use_menu_position(
    content_ref: NodeRef<html::Div>,
    trigger_width: RwSignal<f64>,
    trigger_height: RwSignal<f64>,
    trigger_x: RwSignal<f64>,
    trigger_y: RwSignal<f64>,
    side: Signal<MenuSide>,
    side_of_set: Signal<f64>,
    align: Signal<MenuAlign>,
    align_of_set: Signal<f64>,
) -> MenuPositionReturn {
    let UseElementBoundingReturn {
        width: content_width,
        height: content_heigt,
        ..
    } = use_element_bounding(content_ref);

    let x = Memo::new(move |_| match side.get() {
        MenuSide::Bottom => {
            trigger_x.get()
                + match align.get() {
                    MenuAlign::Start => align_of_set.get(),
                    MenuAlign::Center => (trigger_width.get() / 2.0) - (content_width.get() / 2.0),
                    MenuAlign::End => -(content_width.get()) + align_of_set.get(),
                }
        }
        MenuSide::Left => trigger_x.get() - content_width.get() - side_of_set.get(),
        MenuSide::Right => trigger_x.get() + trigger_width.get() + side_of_set.get(),
        MenuSide::Top => {
            trigger_x.get()
                + match align.get() {
                    MenuAlign::Start => align_of_set.get(),
                    MenuAlign::Center => (trigger_width.get() / 2.0) - (content_width.get() / 2.0),
                    MenuAlign::End => trigger_width.get() + align_of_set.get(),
                }
        }
    });
    let y = Memo::new(move |_| match side.get() {
        MenuSide::Bottom => trigger_y.get() + trigger_height.get() + side_of_set.get(),
        MenuSide::Left => {
            trigger_y.get()
                + match align.get() {
                    MenuAlign::Start => align_of_set.get(),
                    MenuAlign::Center => (trigger_height.get() / 2.0) - (content_heigt.get() / 2.0),
                    MenuAlign::End => trigger_height.get(),
                }
        }
        MenuSide::Right => {
            trigger_y.get()
                + match align.get() {
                    MenuAlign::Start => align_of_set.get(),
                    MenuAlign::Center => (trigger_height.get() / 2.0) - (content_heigt.get() / 2.0),
                    MenuAlign::End => trigger_height.get(),
                }
        }
        MenuSide::Top => trigger_y.get() - content_heigt.get() + side_of_set.get(),
    });
    MenuPositionReturn { x, y }
}
