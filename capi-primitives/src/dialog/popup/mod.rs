use std::sync::Arc;

use leptos::either::Either;
use leptos::{html, prelude::*};
use leptos_dom::error;
use leptos_node_ref::AnyNodeRef;

use crate::common::status::TransitionStatus;
use crate::dialog::root::use_dialog_root_context;

#[component]
pub fn DialogPopup(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let context = use_dialog_root_context();
    let transition_state = context.transition_status.transition_status;
    let children = StoredValue::new(children);
    view! {
        <div
            class=class
            node_ref=context.popup_ref
            data-state=move || transition_state.get().to_string()
        >
            {if let Some(children) = children.get_value() {
                Either::Left(children())
            } else {
                Either::Right(())
            }}
        </div>
    }
}
