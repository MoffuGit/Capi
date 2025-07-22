use api::channel::{preload_channels, GetChannels};
use common::convex::{Channel, Server};
use convex_client::leptos::UseQuery;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;
use tailwind_fuse::tw_merge;

use crate::components::icons::IconEllipsis;
use crate::components::ui::sidebar::{
    SidebarMenu, SidebarMenuAction, SidebarMenuButton, SidebarMenuItem,
};

#[component]
pub fn ChannelsItems(
    server: Memo<Option<Server>>,
    #[prop(into, optional)] category: Option<String>,
) -> impl IntoView {
    let location = use_location();
    let path = location.pathname;
    let current_channel = Memo::new(move |_| {
        path.get()
            .split('/')
            .nth(3)
            .map(|channel| channel.to_string())
    });
    let category = StoredValue::new(category);
    let preloaded_channels = Resource::new(
        move || (server.get(), category.get_value()),
        move |(server, category)| preload_channels(server.map(|server| server.id), category),
    );
    view! {
        <SidebarMenu>
            <Transition>
                {
                    move || {
                        preloaded_channels.and_then(|channels| {
                            let channels = UseQuery::with_preloaded(move || {
                                server.get().map(|server| {
                                    GetChannels {
                                        server: server.id,
                                        category: category.get_value(),
                                    }
                                })
                            }, channels.clone());
                            let channels = Signal::derive(move || channels.get().and_then(|res| res.ok()));
                            view!{
                                {
                                    move || {
                                        channels.get().map(|channels| {
                                            channels.into_iter().map(|channel| view!{
                                                    <ChannelItem channel=channel current_channel=current_channel/>
                                                }
                                            ).collect_view()
                                        })
                                    }
                                }
                            }
                        })
                    }
                }
            </Transition>
        </SidebarMenu>
    }
}

#[component]
pub fn ChannelItem(channel: Channel, current_channel: Memo<Option<String>>) -> impl IntoView {
    let name = StoredValue::new(channel.name);
    let id = StoredValue::new(channel.id);
    view! {
        <A href=move || format!("/servers/{}/{}", channel.server,  id.get_value())>
            <SidebarMenuItem>
                <SidebarMenuButton
                    is_active=Signal::derive(
                        move || {
                            current_channel.get().is_some_and(|curr| {
                                 id.get_value() == curr
                            })
                        }
                    )
                    class="group/button">
                    <span
                        class=tw_merge!(
                            "text-sidebar-foreground/70 inline-flex flex-col items-start font-normal",
                            "group-data-[active=true]/button:font-bold group-hover/button:text-sidebar-foreground",
                            "transition-[color,font-weight] duration-150 ease-out",
                            "after:content-[attr(data-text)] after:h-0 after:hidden after:overflow-hidden after:select-none after:pointer-events-none after:font-bold"
                        )
                         data-text={name.get_value()}
                    >
                        {name.get_value()}
                    </span>
                </SidebarMenuButton>
                <SidebarMenuAction show_on_hover=true>
                    <IconEllipsis />
                    <span class="sr-only">More</span>
                </SidebarMenuAction>
            </SidebarMenuItem>
        </A>

    }
}
