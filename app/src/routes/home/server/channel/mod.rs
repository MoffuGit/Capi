mod components;

use common::convex::{Channel, Member, Role};
use convex_client::leptos::{Mutation, Query, UseMutation, UseQuery};
use leptos::prelude::*;
use leptos_dom::error;
use leptos_router::hooks::use_location;
use serde::{Deserialize, Serialize};

use crate::components::auth::use_auth;
use crate::components::ui::sidebar::{SidebarInset, SidebarProvider};

use self::components::chat::Chat;
use self::components::header::Header;
use self::components::sidebar::MembersSideBar;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct GetMemberForServerByUser {
    #[serde(rename = "serverId")]
    server: String,
    auth: i64,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct MemberWithRole {
    member: Member,
    roles: Vec<Role>,
}

impl Query<Option<MemberWithRole>> for GetMemberForServerByUser {
    fn name(&self) -> String {
        "user:getMemberForServerByUser".to_string()
    }
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct GetChannel {
    #[serde(rename = "channelId")]
    channel: String,
    #[serde(rename = "serverId")]
    server: String,
    auth: i64,
}

impl Query<Option<Channel>> for GetChannel {
    fn name(&self) -> String {
        "channel:get".to_string()
    }
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct SetLastVisitedChannel {
    auth: i64,
    member: String,
    channel: String,
}

impl Mutation for SetLastVisitedChannel {
    type Output = ();

    fn name(&self) -> String {
        "member:setLastVisitedChannel".into()
    }
}

#[component]
pub fn Channel() -> impl IntoView {
    let auth = use_auth().auth();
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

    let member_with_role = UseQuery::new(move || {
        let auth = auth.get().and_then(|res| res.ok()).flatten()?;

        server.get().map(|server_id| GetMemberForServerByUser {
            server: server_id,
            auth: auth.id,
        })
    });

    let member: Signal<Option<Member>> = Signal::derive(move || {
        member_with_role
            .get()
            .and_then(|res| res.ok())
            .flatten()
            .map(|data| data.member)
    });

    let channel_query_signal_result = UseQuery::new(move || {
        let server = server.get()?;
        let channel = channel.get()?;
        let auth = auth.get().and_then(|res| res.ok()).flatten()?;

        Some(GetChannel {
            server,
            channel,
            auth: auth.id,
        })
    });

    let current_channel: Signal<Option<Channel>> = Signal::derive(move || {
        channel_query_signal_result
            .get()
            .and_then(|query_res| query_res.ok().flatten())
    });

    let set_last_visited_channel = UseMutation::new::<SetLastVisitedChannel>();

    Effect::watch(
        move || current_channel.get(),
        move |channel, _, _| {
            let auth = auth.get().and_then(|auth| auth.ok()).flatten();
            if let (Some(auth), Some(channel), Some(member)) = (auth, channel, member.get()) {
                set_last_visited_channel.dispatch(SetLastVisitedChannel {
                    auth: auth.id,
                    member: member.id,
                    channel: channel.id.to_string(),
                });
            }
        },
        false,
    );

    let open = RwSignal::new(false);

    view! {
        <Header channel=current_channel members_open=open />
        <SidebarProvider class="flex-1 min-h-0 min-w-0" open=open main=false style="--sidebar-width: 250px" shortcut="u">
            <SidebarInset class="flex-1 max-h-screen">
                <Chat channel=current_channel member=member/>
            </SidebarInset>
            <MembersSideBar server=server member=member/>
        </SidebarProvider>
    }
}
