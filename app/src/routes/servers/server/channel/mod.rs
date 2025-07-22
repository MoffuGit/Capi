mod components;

use common::convex::{Channel, Member};
use convex_client::leptos::{Query, UseQuery};
use leptos::prelude::*;
use leptos_router::hooks::use_location;
use serde::Serialize;

use crate::components::auth::use_auth;
use crate::components::ui::sidebar::{SidebarInset, SidebarProvider};
use crate::routes::use_profile;

use self::components::chat::Chat;
use self::components::header::Header;
use self::components::sidebar::MembersSideBar;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct GetMemberForServerByUser {
    #[serde(rename = "serverId")]
    server: String,
    user: String,
}

impl Query<Option<Member>> for GetMemberForServerByUser {
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
    #[serde(rename = "memberId")]
    member: String,
}

impl Query<Option<Channel>> for GetChannel {
    fn name(&self) -> String {
        "channel:get".to_string()
    }
}

#[component]
pub fn Channel() -> impl IntoView {
    let user = use_profile();
    // let set_last_visited: ServerAction<SetLastVisitedChannel> = ServerAction::new();
    let location = use_location();
    let path = location.pathname;

    let server_id_memo = Memo::new(move |_| {
        path.get()
            .split('/')
            .nth(2)
            .map(|server| server.to_string())
    });

    let channel_id_memo = Memo::new(move |_| {
        path.get()
            .split('/')
            .nth(3)
            .map(|channel| channel.to_string())
    });

    let member_query_signal_result = UseQuery::new(move || {
        user.get().and_then(|user_data| {
            server_id_memo
                .get()
                .map(|server_id| GetMemberForServerByUser {
                    server: server_id,
                    user: user_data.id,
                })
        })
    });

    let channel_query_signal_result = UseQuery::new(move || {
        // This closure must return Option<GetChannel>
        let server = server_id_memo.get()?;
        let channel = channel_id_memo.get()?;

        let member_result = member_query_signal_result.get()?; // Option<Result<Option<Member>, String>>
        let member_option = member_result.ok()?; // Option<Member>
        let member = member_option?; // Member

        Some(GetChannel {
            server,
            channel,
            member: member.id,
        })
    });

    let current_member: Signal<Option<Member>> = Signal::derive(move || {
        member_query_signal_result.get().and_then(|query_res| {
            query_res.ok().flatten() // Takes Option<Result<Option<Member>, String>> -> Option<Option<Member>> -> Option<Member>
        })
    });

    let current_channel: Signal<Option<Channel>> = Signal::derive(move || {
        channel_query_signal_result.get().and_then(|query_res| {
            query_res.ok().flatten() // Takes Option<Result<Option<Channel>, String>> -> Option<Option<Channel>> -> Option<Channel>
        })
    });

    let open = RwSignal::new(false);

    view! {
        <Header channel=current_channel members_open=open />
        <SidebarProvider class="flex-1 min-h-0" open=open main=false style="--sidebar-width: 250px" shortcut="u">
            <SidebarInset class="flex-1 max-h-screen">
                <Chat channel=current_channel member=current_member/>
            </SidebarInset>
            <MembersSideBar server=server_id_memo member=current_member/>
        </SidebarProvider>
    }
}
