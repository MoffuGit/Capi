use common::convex::Server;
use convex_client::leptos::Mutation;
use convex_client::leptos::Query;
use convex_client::leptos::UseMutation;
use convex_client::leptos::UseQuery;
use leptos::prelude::*;
use serde::Serialize;

use crate::components::auth::use_auth;
use crate::routes::use_profile;
use capi_ui::avatar::*;
use capi_ui::button::*;
use capi_ui::card::*;

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct GetPublicServers {
    pub auth: i64,
}

impl Query<Vec<Server>> for GetPublicServers {
    fn name(&self) -> String {
        "server:getPublicServers".into()
    }
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct JoinServer {
    #[serde(rename = "serverId")]
    server: String,
    #[serde(rename = "userId")]
    user: String,
}

impl Mutation for JoinServer {
    type Output = String;

    fn name(&self) -> String {
        "server:joinServer".into()
    }
}

#[component]
pub fn Discover(data: Signal<Option<Vec<Server>>>) -> impl IntoView {
    let join_server_action = UseMutation::new::<JoinServer>();

    let auth = use_auth().auth;

    let auth_id_signal =
        Signal::derive(move || auth.get().and_then(|res| res.ok()).flatten().map(|a| a.id));

    view! {
        <div class="grid gap-4 grid-cols-[repeat(auto-fill,minmax(min(300px,100%),1fr))]">
            <For
                each=move || data.get().unwrap_or_default()
                key=|server| server.id.clone()
                let(
                    server
                )
            >
                <ServerItem server=server join_server_action=join_server_action auth_id=auth_id_signal />
            </For>
        </div>
    }
}

#[component]
pub fn ServerItem(
    server: Server,
    join_server_action: Action<JoinServer, Result<String, String>>,
    auth_id: Signal<Option<i64>>,
) -> impl IntoView {
    let server = StoredValue::new(server);
    let user = use_profile();

    let on_join_click = move |_| {
        if let Some(user) = user.get() {
            join_server_action.dispatch(JoinServer {
                server: server.get_value().id,
                user: user.id.clone(),
            });
        }
    };

    view! {
        <Card class="p-2 min-w-0 gap-2">
            <CardHeader class="p-0 min-w-0">
                <ServerBanner server=server>
                    <ServerImage server=server/>
                </ServerBanner>
            </CardHeader>
            <CardContent class="px-0 gap-2 flex flex-col">
                <CardTitle class="capitalize">
                    {move || server.get_value().name}
                </CardTitle>
                <CardDescription>
                    {move || server.get_value().description}
                </CardDescription>
                <Button
                    class="w-full overflow-hidden relative"
                    variant=ButtonVariants::Secondary
                    on:click=on_join_click
                    size=ButtonSizes::Sm
                    disabled=Signal::derive(move || join_server_action.pending().get() | auth_id.get().is_none())
                >
                    "Join Server"
                </Button>
            </CardContent>
        </Card>
    }
}

#[component]
pub fn ServerBanner(children: Children, server: StoredValue<Server>) -> impl IntoView {
    view! {
        <Avatar class="flex relative bg-muted w-full h-20 items-center justify-center rounded-lg min-w-0">
            <AvatarImage url=MaybeProp::derive(move || server.get_value().banner_url) class="w-full h-full object-cover rounded-lg"/>
            <AvatarFallback >
                <div/>
            </AvatarFallback>
            {children()}
        </Avatar>
    }
}

#[component]
pub fn ServerImage(server: StoredValue<Server>) -> impl IntoView {
    view! {
        <Avatar class="absolute shadow-xs left-2 size-16 bg-background p-1 rounded-lg overflow-visible min-w-0">
            <AvatarImage url=MaybeProp::derive(move || server.get_value().image_url) class="rounded-md w-full h-full object-cover"/>
            <AvatarFallback class="rounded-lg text-xl">
                {move || server.get_value().name.chars().next()}
            </AvatarFallback>
        </Avatar>
    }
}
