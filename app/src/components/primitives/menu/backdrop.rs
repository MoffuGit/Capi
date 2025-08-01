use leptos::{html, prelude::*};
use leptos_node_ref::AnyNodeRef;

use crate::components::primitives::common::status::TransitionStatus;
use crate::components::primitives::menu::MenuProviderContext;
use crate::components::primitives::primitive::Primitive;

#[component]
pub fn MenuBackDrop(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let MenuProviderContext {
        open,
        modal,
        dismissible,
        ..
    } = use_context().expect("should acces the menu context");
    let transition_status =
        use_context::<RwSignal<TransitionStatus>>().expect("should access the transition context");

    let children = StoredValue::new(children);

    let on_click_handler = move |_| {
        // If the dialog is dismissible and not modal, close it when the overlay is clicked.
        if dismissible {
            open.set(false);
        }
    };

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref={node_ref}
            {..}
            class=class
            data-state=move || {
                match transition_status.get() {
                    TransitionStatus::Starting => "open",
                    TransitionStatus::Ending => "closed",
                    TransitionStatus::Idle => "",
                    TransitionStatus::Undefined => "undefined",
                }
            }
            data-modal=move || modal.to_string()
            on:click=on_click_handler
        >
            {children.get_value().map(|children| children())}
        </Primitive>
    }
}
