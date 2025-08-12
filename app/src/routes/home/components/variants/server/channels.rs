use common::convex::Channel;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;
use tailwind_fuse::tw_merge;

use icons::{IconEllipsis, IconTrash};
use crate::components::ui::dropwdown::{
    DropdownMenu, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuLabel,
    DropdownMenuTrigger,
};
use crate::components::ui::sidebar::{
    SidebarMenu, SidebarMenuAction, SidebarMenuButton, SidebarMenuItem,
};
use capi_primitives::menu::{MenuAlign, MenuSide};

#[component]
pub fn ChannelsItems(channels: ReadSignal<Option<Result<Vec<Channel>, String>>>) -> impl IntoView {
    let location = use_location();
    let path = location.pathname;
    let current_channel = Memo::new(move |_| {
        path.get()
            .split('/')
            .nth(3)
            .map(|channel| channel.to_string())
    });
    let channels = Signal::derive(move || channels.get().and_then(|res| res.ok()));
    view! {
        <SidebarMenu>
            <For
                each=move || channels.get().unwrap_or_default()
                key=|channel| channel.id.clone()
                let:channel
            >
                <ChannelItem channel=channel current_channel=current_channel />
            </For>
        </SidebarMenu>
    }
}

#[component]
pub fn ChannelItem(channel: Channel, current_channel: Memo<Option<String>>) -> impl IntoView {
    let name = StoredValue::new(channel.name);
    let id = StoredValue::new(channel.id);
    view! {
        <SidebarMenuItem>
            <A href=move || format!("/servers/{}/{}", channel.server,  id.get_value())>
                <SidebarMenuButton
                    is_active=Signal::derive(
                        move || {
                            current_channel.get().is_some_and(|curr| {
                                 id.get_value() == curr
                            })
                        }
                    )
                    class="group/button group-data-[collapsible=icon]:size-auto! group-data-[collapsible=icon]:h-8! group-data-[collapsible=icon]:p-2!">
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
            </A>
            <DropdownMenu>
                    <SidebarMenuAction show_on_hover=true>
                        <DropdownMenuTrigger class="size-4">
                            <IconEllipsis class="size-4"/>
                            <span class="sr-only">More</span>
                        </DropdownMenuTrigger>
                    </SidebarMenuAction>
                <DropdownMenuContent side=MenuSide::Right align=MenuAlign::Start>
                    <DropdownMenuGroup>
                        <DropdownMenuLabel>
                            {name.get_value()}
                        </DropdownMenuLabel>
                        <DropdownMenuItem class="hover:text-destructive/70 group">
                            <IconTrash class="group-hover:text-destructive/70"/>
                            "Delete Channel"
                        </DropdownMenuItem>
                    </DropdownMenuGroup>
                </DropdownMenuContent>
            </DropdownMenu>
        </SidebarMenuItem>

    }
}
