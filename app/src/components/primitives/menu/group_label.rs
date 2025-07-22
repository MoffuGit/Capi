use leptos::prelude::*;

#[component]
pub fn GroupLabel(
    children: Children,
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    // let GroupContext { label } = use_context().expect("should acces to the gruop context");
    // let node_ref: NodeRef<Div> = NodeRef::new();
    //
    // Effect::new(move || {
    //     node_ref.get().unwrap()
    // });

    view! {
        <div class=class /* node_ref=node_ref */>
            {children()}
        </div>
    }
}
