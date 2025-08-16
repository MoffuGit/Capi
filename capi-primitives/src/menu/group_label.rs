use leptos::prelude::*;

#[component]
pub fn GroupLabel(
    children: Children,
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    view! {
        <div class=class /* node_ref=node_ref */>
            {children()}
        </div>
    }
}
