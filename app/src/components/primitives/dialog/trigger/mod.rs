use leptos::{html, prelude::*};
use leptos_node_ref::AnyNodeRef;

use crate::components::primitives::dialog::root::use_dialog_root::DialogRootContext;
use crate::components::primitives::dialog::root::use_dialog_root_context;
use crate::components::primitives::primitive::Primitive;

#[component]
pub fn DialogTrigger(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let DialogRootContext { open, set_open, .. } = use_dialog_root_context();
    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref={node_ref}
            // {..attrs}
            {..}
            on:click=move |_| {
                set_open.update(|open| *open = !*open);
            }
        >
            {children.get_value().map(|children| children())}
        </Primitive>
    }
}
