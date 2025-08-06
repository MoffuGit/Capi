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
                    {..}
                    class="z-20"
                >
                    <Tab set_on_click=false value="servers">
                        "Servers"
                    </Tab>
                </A>
                <A
                    href="/servers/#discover"
                    {..}
                    class="z-20"
                >
                    <Tab set_on_click=false value="discover"
                        {..}
                        on:click=move |_| {

                        }
                    >
                        "Discover"
                    </Tab>
                </A>
                <TabIndicator class="z-10"/>
            </TabsList>
        </header>
    }
}
