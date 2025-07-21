use common::convex::{Category, Server};
use convex_client::leptos::{Query, UseQuery};
use leptos::prelude::*;
use serde::Serialize;

use crate::components::icons::IconPlus;
use crate::components::ui::sidebar::{
    SidebarGroup, SidebarGroupAction, SidebarGroupContent, SidebarGroupLabel,
};
use crate::routes::servers::components::variants::server::channels::ChannelsItems;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct GetCategories {
    server: String,
}

impl Query<Vec<Category>> for GetCategories {
    fn name(&self) -> String {
        "server:getCategories".to_string()
    }
}

#[component]
pub fn CategoriesItems(server: Memo<Option<Server>>) -> impl IntoView {
    let categories = UseQuery::new(move || {
        server
            .get()
            .map(|server| GetCategories { server: server.id })
    });
    let categories = Signal::derive(move || categories.get().and_then(|res| res.ok()));
    view! {
        <Show when=move || categories.get().is_some()>
            <For
                each=move || categories.get().unwrap()
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
