use std::sync::Arc;

use leptos::{html, prelude::*};
use leptos_node_ref::AnyNodeRef;

use crate::components::primitives::common::status::TransitionStatus;
use crate::components::primitives::dialog::root::use_dialog_root_context;
use crate::components::primitives::primitive::Primitive;

#[component]
pub fn DialogPopup(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let context = use_dialog_root_context();
    let transition_status =
        use_context::<RwSignal<TransitionStatus>>().expect("should acces the transition context");
    let children = StoredValue::new(children);
    view! {
        <div
            class=class
            node_ref=context.popup_ref
            data-state=move || transition_status.get().to_string()
             >
            {children.get_value().map(|children| children())}
        </div>
    }
}
