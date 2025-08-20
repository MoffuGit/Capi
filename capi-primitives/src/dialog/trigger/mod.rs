use leptos::{html, prelude::*};
use leptos_node_ref::AnyNodeRef;

use crate::common::floating::{ClickHandlers, use_click};
use crate::dialog::root::DialogRootContext;
use crate::dialog::root::use_dialog_root_context;
use crate::primitive::Primitive;

#[component]
pub fn DialogTrigger(
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let DialogRootContext {
        open,
        floating,
        trigger_ref,
        ..
    } = use_dialog_root_context();
    let ClickHandlers { on_click } = use_click(&floating);
    view! {
        <div
            node_ref=trigger_ref
            class=class
            on:click=move |evt| {
                on_click.run(evt);
            }
        >
            {children.get_value().map(|children| children())}
        </div>
    }
}
