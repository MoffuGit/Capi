use leptos::prelude::*;

#[component]
pub fn GroupLabel(children: Children) -> impl IntoView {
    // let GroupContext { label } = use_context().expect("should acces to the gruop context");
    // let node_ref: NodeRef<Div> = NodeRef::new();
    //
    // Effect::new(move || {
    //     node_ref.get().unwrap()
    // });

    view! {
        <div /* node_ref=node_ref */>
            {children()}
        </div>
    }
}
