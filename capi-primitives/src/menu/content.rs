use leptos::either::Either;
use leptos::{html, prelude::*};
use leptos_use::{OnClickOutsideOptions, on_click_outside_with_options};

use crate::common::floating::FloatingPosition;
use crate::menu::MenuProviderContext;

use super::{MenuAlign, MenuSide};

#[component]
pub fn MenuContent(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional)] ignore: Vec<NodeRef<html::Div>>,
    #[prop(into, optional, default = Signal::derive(|| MenuSide::Bottom))] side: Signal<MenuSide>,
    #[prop(into,optional, default = Signal::derive(|| 0.0))] side_of_set: Signal<f64>,
    #[prop(into,optional, default = Signal::derive(|| MenuAlign::Center))] align: Signal<MenuAlign>,
    #[prop(into,optional, default = Signal::derive(|| 0.0))] align_of_set: Signal<f64>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = use_context::<MenuProviderContext>().expect("acces to menu context");
    let content_ref = context.content_ref;

    let FloatingPosition { x, y, .. } =
        context
            .floating
            .get_floating_position(side, side_of_set, align, align_of_set, None);

    Effect::new(move |_| {
        if context.modal
            && let Some(app) = document().get_element_by_id("app")
        {
            if context.open.get() {
                let _ = app.class_list().add_1("pointer-events-none");
            } else {
                let _ = app.class_list().remove_1("pointer-events-none");
            }
        }
    });
    on_cleanup(move || {
        if let Some(app) = document().get_element_by_id("app") {
            let _ = app.class_list().remove_1("pointer-events-none");
        }
    });

    let _ = on_click_outside_with_options(
        context.content_ref,
        move |_| {
            if context.open.get() {
                context.open.set(false)
            }
        },
        OnClickOutsideOptions::default().ignore(ignore),
    );

    let transition_status = context.transition_status;

    view! {
        <div
            style:top=move || format!("{}px", y())
            style:left=move || format!("{}px", x())
            style=move || format!("--radix-menu-content-transform-origin: {}", side())
            class="z-50 absolute"
            data-state=move || transition_status.transition_status.get().to_string()
            node_ref=context.mount_ref
        >
            <div
                data-side=move || side().to_string()
                node_ref=content_ref
                data-state=move || transition_status.transition_status.get().to_string()
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
