use leptos::{html, prelude::*};
use leptos_node_ref::AnyNodeRef;

use crate::components::primitives::common::status::TransitionStatus;
use crate::components::primitives::dialog::root::use_dialog_root::DialogRootContext;
use crate::components::primitives::dialog::root::use_dialog_root_context;
use crate::components::primitives::primitive::Primitive;

#[component]
pub fn DialogOverlay(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let DialogRootContext {
        set_open,
        modal,
        dismissible,
        ..
    } = use_dialog_root_context();
    let transition_status =
        use_context::<RwSignal<TransitionStatus>>().expect("should access the transition context");

    let children = StoredValue::new(children);

    let on_click_handler = move |_| {
        // If the dialog is dismissible and not modal, close it when the overlay is clicked.
        if dismissible {
            set_open.set(false);
        }
    };

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref={node_ref}
            {..}
            class=class
            data-state=move || transition_status.get().to_string()
            data-modal=move || modal.to_string()
            on:click=on_click_handler
        >
            {children.get_value().map(|children| children())}
        </Primitive>
    }
}
