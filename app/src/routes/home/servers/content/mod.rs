mod discover;
mod servers;

use convex_client::leptos::UseQuery;
use leptos::prelude::*;

use capi_ui::tabs::TabPanel;

use crate::components::auth::use_auth;

use self::discover::{Discover, GetPublicServers};
use self::servers::Servers;

#[component]
pub fn Content() -> impl IntoView {
    let auth = use_auth().auth;

    let data = UseQuery::new(move || {
        auth.get()
            .and_then(|res| res.ok())
            .flatten()
            .map(|auth| GetPublicServers { auth: auth.id })
    });

    let data = Signal::derive(move || data.get().and_then(|res| res.ok()));
    view! {
        <div class="w-full h-full flex p-4">
            <TabPanel value="servers">
                <Servers/>
            </TabPanel>
            <TabPanel value="discover">
                <Discover data=data />
            </TabPanel>
        </div>
    }
}
