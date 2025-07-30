mod discover;
mod servers;

use leptos::prelude::*;

use crate::components::ui::tabs::TabPanel;

use self::servers::Servers;

#[component]
pub fn Content() -> impl IntoView {
    view! {
        <div class="w-full h-full flex p-3">
            <TabPanel value="servers">
                <Servers/>
            </TabPanel>
            <TabPanel value="discover">
                "discover"
            </TabPanel>
        </div>
    }
}
