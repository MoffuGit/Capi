use api::convex::Query;
use common::convex::{Category, Server};
use leptos::prelude::*;
use serde_json::json;

use crate::components::icons::IconPlus;
use crate::components::ui::sidebar::{
    SidebarGroup, SidebarGroupAction, SidebarGroupContent, SidebarGroupLabel,
};
use crate::hooks::sycn::SyncSignal;
use crate::routes::servers::components::variants::server::channels::ChannelsItems;

#[component]
pub fn CategoriesItems(server: Memo<Option<Server>>) -> impl IntoView {
    let categories: SyncSignal<Vec<Category>> = SyncSignal::new(Memo::new(move |_| {
        server.get().map(|server| Query {
            name: "server:getCategories".to_string(),
            args: json!({
                "server": server.id,
            }),
        })
    }));
    view! {
        <Show when=move || categories.signal.get().is_some()>
            <For
                each=move || categories.signal.get().unwrap()
                key=|category| category.id.clone()
                children=move |category| {
                    view!{
                        <SidebarGroup>
                            <SidebarGroupLabel>
                                {category.name}
                            </SidebarGroupLabel>
                            <SidebarGroupAction>
                                <IconPlus/>
                                <span class="sr-only">Add channel</span>
                            </SidebarGroupAction>
                            <SidebarGroupContent>
                                <ChannelsItems server=server category=category.id />
                            </SidebarGroupContent>
                        </SidebarGroup>
                    }
                }
            />
        </Show>
    }
}
