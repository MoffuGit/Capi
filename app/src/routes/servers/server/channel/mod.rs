use api::convex::mutations::member::SetLastVisitedChannel;
use api::convex::Query;
use common::convex::{Channel, Member};
use leptos::prelude::*;
use leptos_router::hooks::use_location;
use serde_json::json;
use tailwind_fuse::merge::tw_merge;

use crate::components::auth::use_auth;
use crate::components::icons::{IconSearch, IconUsers};
use crate::components::primitives::common::{Orientation, Side};
use crate::components::ui::breadcrumb::{
    Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
};
use crate::components::ui::button::{Button, ButtonSizes, ButtonVariants};
use crate::components::ui::divider::Separator;
use crate::components::ui::sheet::{Sheet, SheetPopup, SheetTrigger};
use crate::components::ui::sidebar::{
    use_sidebar, SideBarVariant, Sidebar, SidebarInset, SidebarProvider, SidebarTrigger,
};
use crate::hooks::sycn::SyncSignal;

#[component]
pub fn Channel() -> impl IntoView {
    let user = use_auth().user;
    let set_last_visited: ServerAction<SetLastVisitedChannel> = ServerAction::new();
    let location = use_location();
    let path = location.pathname;

    let server = Memo::new(move |_| {
        path.get()
            .split('/')
            .nth(2)
            .map(|server| server.to_string())
    });

    let channel = Memo::new(move |_| {
        path.get()
            .split('/')
            .nth(3)
            .map(|channel| channel.to_string())
    });
    let member_signal: SyncSignal<Member> = SyncSignal::new(Memo::new(move |_| {
        server.get().and_then(|server| {
            user.get().flatten().map(|user| Query {
                name: "user:getMemberForServerByUser".to_string(),
                args: json!({
                    "serverId": server,
                    "user": user.id
                }),
            })
        })
    }));
    let channel: SyncSignal<Channel> = SyncSignal::new(Memo::new(move |_| {
        server
            .get()
            .and_then(|server| {
                channel.get().map(|channel| {
                    member_signal.signal.get().map(|member| Query {
                        name: "channel:get".to_string(),
                        args: json!({
                            "channelId": channel,
                            "serverId": server,
                            "memberId": member.id
                        }),
                    })
                })
            })
            .flatten()
    }));

    let open = RwSignal::new(false);

    view! {
        <header class="bg-background sticky top-0 flex shrink-0 items-center gap-2 border-b p-4">
            <SidebarTrigger class="-ml-1" />
            <Separator
                orientation=Orientation::Vertical
                class="mr-2 data-[orientation=vertical]:h-4"
            />
            {
                move || {
                    channel.signal.get().map(|channel| view!{
                        <Breadcrumb>
                            <BreadcrumbList>
                                <BreadcrumbItem>
                                    <BreadcrumbPage>{channel.name}</BreadcrumbPage>
                                </BreadcrumbItem>
                                <Separator
                                    orientation=Orientation::Vertical
                                    class="data-[orientation=vertical]:h-4"
                                />
                                {
                                    channel.topic.map(|topic| view!{
                                        <BreadcrumbItem>
                                            <BreadcrumbPage>{topic}</BreadcrumbPage>
                                        </BreadcrumbItem>
                                    })
                                }
                            </BreadcrumbList>
                          </Breadcrumb>

                    })
                }
            }
            <div class="ml-auto mr-0 space-x-1">
                <Button
                    variant=ButtonVariants::Ghost
                    size=ButtonSizes::Icon
                    class="size-7"
                    {..}
                    on:click=move |_| {
                        open.update(|open| *open = !*open);
                    }
                >
                    <IconUsers/>
                </Button>
                <Sheet>
                    <SheetTrigger as_child=true >
                        <Button
                            variant=ButtonVariants::Ghost
                            size=ButtonSizes::Icon
                            class="size-7"
                        >
                            <IconSearch/>
                        </Button>
                    </SheetTrigger>
                    <SheetPopup side=Side::Right>
                        <div>
                        </div>
                    </SheetPopup>
                </Sheet>
            </div>
        </header>
        <SidebarProvider class="flex-1 min-h-0" open=open main=false style="--sidebar-width: 250px" shortcut="u">
            <SidebarInset class="flex-1">
                {
                    move || {
                        channel.signal.get().map(|channel| channel.name)
                    }
                }
            </SidebarInset>
            <Sidebar class="mt-[61px] h-auto" side=Side::Right>
                <div/>
            </Sidebar>
        </SidebarProvider>
    }
}
