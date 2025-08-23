mod discover;
mod servers;

use leptos::prelude::*;

use capi_ui::tabs::TabPanel;

use self::discover::Discover;
use self::servers::Servers;

#[component]
pub fn Content() -> impl IntoView {
    view! {
        <div class="w-full h-full flex p-4">
            <TabPanel value="servers">
                <Servers/>
            </TabPanel>
            <TabPanel value="discover">
                <Discover />
            </TabPanel>
        </div>
    }
}
