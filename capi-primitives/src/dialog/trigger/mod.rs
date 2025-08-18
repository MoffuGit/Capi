use leptos::{html, prelude::*};
use leptos_node_ref::AnyNodeRef;

use crate::common::floating::{ClickHandlers, use_click};
use crate::dialog::root::use_dialog_root::DialogRootContext;
use crate::dialog::root::use_dialog_root_context;
use crate::primitive::Primitive;

#[component]
pub fn DialogTrigger(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let DialogRootContext { open, floating, .. } = use_dialog_root_context();
    let ClickHandlers { on_click } = use_click(&floating);
    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref={node_ref}
            // {..attrs}
            {..}
            on:click=move |evt| {
                on_click.run(evt);
            }
        >
            {children.get_value().map(|children| children())}
        </Primitive>
    }
}
