use api::server::{GetServers, ServerData};
use common::convex::{Member, Role, Server};
use convex_client::leptos::UseQuery;
use leptos::prelude::*;

use crate::components::auth::use_auth;
use crate::components::ui::avatar::*;
use crate::components::ui::card::*;

#[component]
pub fn Servers() -> impl IntoView {
    let auth = use_auth().auth();

    let data = UseQuery::new(move || {
        auth.get()
            .and_then(|res| res.ok())
            .flatten()
            .map(|auth| GetServers { auth: auth.id })
    });

    let data = Signal::derive(move || data.get().and_then(|res| res.ok()));

    view! {
        <div class="grid gap-4 grid-cols-[repeat(auto-fill,minmax(min(300px,100%),1fr))]">
            <For
                each=move || data.get().unwrap_or_default()
                key=|data| data.server.id.clone()
                let(
                    ServerData {
                        server,
                        member,
                        roles
                    }
                )
            >
                <ServerItem server=server member=member roles=roles />
            </For>
        </div>
    }
}

#[component]
pub fn ServerItem(server: Server, member: Member, roles: Vec<Role>) -> impl IntoView {
    let server = RwSignal::new(server);
    let _member = RwSignal::new(member);
    let _roles = RwSignal::new(roles);
    view! {
        <Card class="p-2 min-w-0 gap-2">
            <CardHeader class="p-0 min-w-0">
                <ServerBanner server=server.into()>
                    <ServerImage server=server.into()/>
                </ServerBanner>
            </CardHeader>
            <CardContent class="px-4">
                <CardTitle class="capitalize">
                    {move || server.get().name}
                </CardTitle>
                <CardDescription>
                    {move || server.get().description}
                </CardDescription>
            </CardContent>
        </Card>
    }
}

#[component]
pub fn ServerBanner(children: Children, server: Signal<Server>) -> impl IntoView {
    view! {
        <Avatar class="flex relative bg-muted w-full h-20 items-center justify-center rounded-lg min-w-0">
            <AvatarImage url=MaybeProp::derive(move || server.get().banner_url) class="w-full h-full object-cover rounded-lg"/>
            <AvatarFallback >
                <div/>
            </AvatarFallback>
            {children()}
        </Avatar>
    }
}

#[component]
pub fn ServerImage(server: Signal<Server>) -> impl IntoView {
    view! {
        // Ensuring min-w-0 here as well for robustness
        <Avatar class="absolute shadow-xs left-2 size-16 bg-background p-1 rounded-lg overflow-visible min-w-0">
            <AvatarImage url=MaybeProp::derive(move || server.get().image_url) class="rounded-md w-full h-full object-cover"/>
            <AvatarFallback class="rounded-lg text-xl">
                {move || server.get().name.chars().next()}
            </AvatarFallback>
        </Avatar>
    }
}
