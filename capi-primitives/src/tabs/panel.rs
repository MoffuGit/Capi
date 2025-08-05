use leptos::prelude::*;

use crate::tabs::{TabsContext, use_tabs_context};

#[component]
pub fn Panel(
    value: String,
    #[prop(optional, into)] class: MaybeProp<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let value = StoredValue::new(value);
    let TabsContext { selected_tab, .. } = use_tabs_context();
    view! {
        <Show when=move || selected_tab.with(|selected| selected == &value.get_value())>
            <div class=class>
                {children()}
            </div>
        </Show>
    }
}
