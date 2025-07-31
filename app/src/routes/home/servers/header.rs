use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::ui::tabs::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="bg-background sticky top-0 flex shrink-0 items-center gap-2 p-3 border-b">
            <TabsList class="gap-2">
                <A
                    href="/servers"
                >
                    <Tab set_on_click=false value="servers">
                        "Servers"
                    </Tab>
                </A>
                <A
                    href="/servers/#discover"
                >
                    <Tab set_on_click=false value="discover"
                        {..}
                        on:click=move |_| {

                        }
                    >
                        "Discover"
                    </Tab>
                </A>
                <TabIndicator/>
            </TabsList>
        </header>
    }
}
