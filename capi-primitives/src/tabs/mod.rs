mod indicator;
mod list;
mod panel;
mod tab;

use std::collections::HashMap;

pub use indicator::TabIndicator;
use leptos::html::Div;
pub use list::TabsList;
pub use panel::Panel;
pub use tab::Tab;

use leptos::context::Provider;
use leptos::prelude::*;

use super::common::Orientation;

#[derive(Debug, Clone)]
pub struct TabsContext {
    orientation: Orientation,
    selected_tab: RwSignal<String>,
    tabs: RwSignal<HashMap<String, NodeRef<Div>>>,
}
pub fn use_tabs_context() -> TabsContext {
    use_context().expect("should acces to the use_tabs_context")
}

#[component]
pub fn TabsRoot(
    #[prop(optional, into)] orientation: Orientation,
    #[prop(optional, into)] tab: RwSignal<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let tabs = RwSignal::new(HashMap::default());
    view! {
        <Provider value=TabsContext {selected_tab: tab, orientation, tabs }>
            <div class=class data-orientation=orientation.to_string()>
                {children()}
            </div>
        </Provider>
    }
}
