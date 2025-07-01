use leptos::{html, prelude::*};
use leptos_node_ref::AnyNodeRef;

use crate::components::primitives::primitive::Primitive;
#[component]
pub fn Button(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into, default = Signal::from(false))] disabled: Signal<bool>,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <Primitive
            element=html::button
            as_child=as_child
            node_ref={node_ref}
            {..}
            disabled=disabled
            class=class
        >
            {children.get_value().map(|children| children())}
        </Primitive>
    }
}
