use leptos::prelude::*;

use crate::components::primitives::tabs::TabsContext;

use super::use_tabs_context;

#[component]
pub fn TabsList(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let TabsContext { orientation, .. } = use_tabs_context();

    view! {
        <div class=class data-orientation=orientation.to_string()>
            {children()}
        </div>
    }
}
