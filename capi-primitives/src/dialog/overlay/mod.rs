use leptos::either::Either;
use leptos::{html, prelude::*};
use leptos_node_ref::AnyNodeRef;

use crate::common::status::TransitionStatus;
use crate::dialog::root::DialogRootContext;
use crate::dialog::root::use_dialog_root_context;
use crate::primitive::Primitive;

#[component]
pub fn DialogOverlay(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let DialogRootContext {
        modal,
        dismissible,
        overlay_status,
        backdrop_ref,
        ..
    } = use_dialog_root_context();

    let children = StoredValue::new(children);

    view! {
        <div
            node_ref=backdrop_ref
            class=class
            data-state=move || overlay_status.transition_status.get().to_string()
            data-modal=move || modal.to_string()
        >
            {if let Some(children) = children.get_value() {
                Either::Left(children())
            } else {
                Either::Right(())
            }}
        </div>
    }
}
