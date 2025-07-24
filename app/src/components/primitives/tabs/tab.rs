use leptos::html::Div;
use leptos::prelude::*;

use crate::components::primitives::tabs::{use_tabs_context, TabsContext};

#[component]
pub fn Tab(
    value: String,
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let value = StoredValue::new(value);
    let tab_ref: NodeRef<Div> = NodeRef::new();
    let TabsContext {
        selected_tab, tabs, ..
    } = use_tabs_context();
    Effect::new(move |_| {
        tabs.update(|tabs| {
            tabs.insert(value.get_value(), tab_ref);
        });
    });
    view! {
        <div
            node_ref=tab_ref
            class=class
            on:click=move |_| {
                selected_tab.set(value.get_value());
            }
            data-state=move || selected_tab.with(|selected| if selected == &value.get_value() {
                "active"
            } else {
                ""
            })
        >
            {children()}
        </div>
    }
}
