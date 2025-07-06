use api::convex::mutations::member::SetLastVisitedChannel;
use api::convex::Query;
use common::convex::{Channel, Member};
use leptos::prelude::*;
use leptos_router::hooks::use_location;
use serde_json::json;

use crate::components::auth::use_auth;
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

    view! {
        <div>
        {
            move || {
                channel.signal.get().map(|channel| channel.name)
            }
        }
        </div>
    }
}
