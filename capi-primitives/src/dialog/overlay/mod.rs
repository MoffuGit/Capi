use leptos::either::Either;
use leptos::{html, prelude::*};
use leptos_node_ref::AnyNodeRef;

use crate::common::status::TransitionStatus;
use crate::dialog::root::use_dialog_root::DialogRootContext;
use crate::dialog::root::use_dialog_root_context;
use crate::primitive::Primitive;

#[component]
pub fn DialogOverlay(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let DialogRootContext {
        modal,
        dismissible,
        transition_status,
        ..
    } = use_dialog_root_context();

    let children = StoredValue::new(children);

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref={node_ref}
            {..}
            class=class
            data-state=move || transition_status.transition_status.get().to_string()
            data-modal=move || modal.to_string()
        >
            {if let Some(children) = children.get_value() {
                Either::Left(children())
            } else {
                Either::Right(())
            }}
        </Primitive>
    }
}
