use leptos::context::Provider;
use leptos::prelude::*;

#[derive(Clone)]
pub struct GroupContext {
    label: RwSignal<Option<String>>,
}

#[component]
pub fn MenuGroup(children: Children) -> impl IntoView {
    let label: RwSignal<Option<String>> = RwSignal::new(None);
    view! {
        <Provider value=GroupContext {
            label
        }>
            <div>
                {children()}
            </div>
        </Provider>
    }
}
