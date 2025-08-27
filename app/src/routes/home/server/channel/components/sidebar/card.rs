use capi_ui::avatar::*;
use capi_ui::badge::*;
use capi_ui::button::*;
use capi_ui::card::*;
use capi_ui::input::Input;
use capi_ui::toast::use_toast_store;
use capi_ui::toast::ToastData;
use capi_ui::toast::ToastStoreStoreFields;
use capi_ui::tooltip::*;
use common::convex::Member;
use common::convex::PresenceStatus;
use convex_client::leptos::ConvexClient;
use convex_client::leptos::Mutation;
use convex_client::leptos::UseMutation;
use icons::IconUserPlus;
use leptos::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::components::auth::use_auth;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendFriendRequest {
    auth: i64,
    #[serde(rename = "receiverId")]
    receiver: String,
}

impl Mutation for SendFriendRequest {
    type Output = ();

    fn name(&self) -> String {
        "friends:sendFriendRequest".into()
    }
}

#[component]
pub fn MemberCard(
    member: Member,
    #[prop(into, optional)] status: Signal<Option<PresenceStatus>>,
) -> impl IntoView {
    let member = StoredValue::new(member);
    let auth = use_auth().auth;
    let store = use_toast_store();
    let on_friend_request = move |msg: String| {
        store.toasts().update(move |toasts| {
            toasts.push(ToastData {
                id: Uuid::new_v4().as_u128(),
                title: "".into(),
                _type: "".into(),
                description: msg,
                removed: false,
                timeout: 4000,
                height: 0.0,
            });
        });
    };
    let send_friend_request = UseMutation::with_local_fn(
        move |(mutation, client): (&SendFriendRequest, &mut ConvexClient)| {
            let mutation = mutation.to_owned();
            let mut client = client.to_owned();
            async move {
                let result = mutation.run(&mut client).await;
                if result.is_ok() {
                    on_friend_request("Friend request sent!".into())
                } else {
                    on_friend_request("Something go wrong with your friend request".into())
                }
            }
        },
    );
    view! {
        <Card {..} class="p-2 gap-2 items-start w-72 relative">
            <CardHeader class="p-0 min-w-0 w-full">
                <UserBanner url=member.get_value().banner_url>
                    <UserImage url=member.get_value().image_url name=member.get_value().name/>
                </UserBanner>
            </CardHeader>
            <CardContent class="px-1 gap-2 flex flex-col">
                <div class="flex items-center justify-between gap-2">
                    <CardTitle class="capitalize">
                        {member.get_value().name}
                    </CardTitle>
                    <ToolTip>
                        <ToolTipTrigger>
                            <Button
                                variant=ButtonVariants::Secondary
                                size=ButtonSizes::IconXs
                                on:click=move |_| {
                                    if let Some(auth) = auth.get().and_then(|auth| auth.ok()).flatten() {
                                        send_friend_request.dispatch(SendFriendRequest { auth: auth.id, receiver: member.get_value().user });
                                    }
                                }
                            >
                                <IconUserPlus />
                            </Button>
                        </ToolTipTrigger>
                        <ToolTipContent side=ToolTipSide::Top>
                            "Add Friend"
                        </ToolTipContent>
                    </ToolTip>
                </div>
                {
                    move || {
                        status.get().map(|status| {
                            view!{
                                <Badge class="h-4 rounded-sm px-1.5" variant=Signal::derive(move || {
                                    match status {
                                        PresenceStatus::Online => BadgeVariant::Online,
                                        PresenceStatus::NotDisturb => BadgeVariant::NotDisturb,
                                        PresenceStatus::Idle => BadgeVariant::Idle,
                                        _ => BadgeVariant::Secondary
                                    }
                                })>
                                    {
                                        status.to_string()
                                    }
                                </Badge>
                            }
                        })
                    }
                }
                <Input class="w-full placeholder:truncate" {..} placeholder=format!("Send message to @{}", member.get_value().name )/>
            </CardContent>
        </Card>
    }
}

#[component]
pub fn UserBanner(children: Children, url: Option<String>) -> impl IntoView {
    view! {
        <Avatar class="flex relative bg-muted w-full h-20 items-center justify-center rounded-md min-w-0">
            <AvatarImage url=url class="w-full h-full object-cover rounded-lg"/>
            <AvatarFallback >
                <div/>
            </AvatarFallback>
            {children()}
        </Avatar>
    }
}

#[component]
pub fn UserImage(url: Option<String>, name: String) -> impl IntoView {
    view! {
        <Avatar class="absolute shadow-xs left-2 size-16 bg-background p-1 rounded-lg overflow-visible min-w-0">
            <AvatarImage url=url class="rounded-md w-full h-full object-cover"/>
            <AvatarFallback class="rounded-lg text-xl">
                {name.chars().next()}
            </AvatarFallback>
        </Avatar>
    }
}
