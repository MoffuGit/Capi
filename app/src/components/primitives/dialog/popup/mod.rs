use std::sync::Arc;

use leptos::{html, prelude::*};
use leptos_node_ref::AnyNodeRef;

use crate::components::primitives::common::status::TransitionStatus;
use crate::components::primitives::primitive::Primitive;

#[component]
pub fn DialogPopup(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let transition_status =
        use_context::<RwSignal<TransitionStatus>>().expect("should acces the transition context");
    let children = StoredValue::new(children);
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
             >
            {children.get_value().map(|children| children())}
        </Primitive>
    }
}
