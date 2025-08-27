pub mod conversation;

use capi_ui::avatar::*;
use capi_ui::card::*;
use capi_ui::divider::Separator;
use capi_ui::Orientation;
use convex_client::leptos::{Query, UseQuery};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::auth::use_auth;
use crate::components::ui::sidebar::SidebarTrigger;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FriendDetails {
    pub _id: String,
    pub name: String,
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
    #[serde(rename = "bannerUrl")]
    pub banner_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PendingFriendRequest {
    pub _id: String,
    #[serde(rename = "senderId")]
    pub sender_id: String,
    #[serde(rename = "senderName")]
    pub sender_name: String,
    #[serde(rename = "senderImageUrl")]
    pub sender_image_url: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SentFriendRequest {
    pub _id: String,
    #[serde(rename = "receiverId")]
    pub receiver_id: String,
    #[serde(rename = "receiverName")]
    pub receiver_name: String,
    #[serde(rename = "receiverImageUrl")]
    pub receiver_image_url: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct GetFriends {
    pub auth: i64,
}

impl Query<Vec<FriendDetails>> for GetFriends {
    fn name(&self) -> String {
        "friends:getFriends".to_string()
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct GetPendingFriendRequests {
    pub auth: i64,
}

impl Query<Vec<PendingFriendRequest>> for GetPendingFriendRequests {
    fn name(&self) -> String {
        "friends:getPendingFriendRequests".to_string()
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct GetSentFriendRequests {
    pub auth: i64,
}

impl Query<Vec<SentFriendRequest>> for GetSentFriendRequests {
    fn name(&self) -> String {
        "friends:getSentFriendRequests".to_string()
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct GetFriendshipStatus {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub auth: i64,
}

impl Query<String> for GetFriendshipStatus {
    fn name(&self) -> String {
        "friends:getFriendshipStatus".to_string()
    }
}

#[component]
pub fn Friends() -> impl IntoView {
    let auth = use_auth().auth;
    let friends = UseQuery::new(move || {
        auth.get()
            .and_then(|auth| auth.ok())
            .flatten()
            .map(|auth| GetFriends { auth: auth.id })
    });
    view! {
        <Header/>
        <div class="grid gap-4 grid-cols-[repeat(auto-fill,minmax(min(300px,100%),1fr))] p-4">
            <For
                each=move || friends.get().and_then(|friends| friends.ok()).unwrap_or_default()
                key=|friend| friend._id.clone()
                children=move |friend| {
                    let name = StoredValue::new(friend.name);

                    view!{
                        <Card class="p-2 min-w-0 gap-2">
                            <CardHeader class="p-0 min-w-0">
                                <UserBanner banner_url=friend.banner_url>
                                    <UserImage image_url=friend.image_url name=name.get_value()/>
                                </UserBanner>
                            </CardHeader>
                            <CardContent class="px-4">
                                <CardTitle class="capitalize">
                                    {name.get_value()}
                                </CardTitle>
                            </CardContent>
                        </Card>

                    }
                }
            />
        </div>
    }
}

#[component]
pub fn UserBanner(children: Children, banner_url: Option<String>) -> impl IntoView {
    view! {
        <Avatar class="flex relative bg-muted w-full h-20 items-center justify-center rounded-lg min-w-0">
            <AvatarImage url=banner_url class="w-full h-full object-cover rounded-lg"/>
            <AvatarFallback >
                <div/>
            </AvatarFallback>
            {children()}
        </Avatar>
    }
}

#[component]
pub fn UserImage(image_url: Option<String>, name: String) -> impl IntoView {
    view! {
        <Avatar class="absolute shadow-xs left-2 size-16 bg-background p-1 rounded-lg overflow-visible min-w-0">
            <AvatarImage url=image_url class="rounded-md w-full h-full object-cover"/>
            <AvatarFallback class="rounded-lg text-xl">
                {name.chars().next()}
            </AvatarFallback>
        </Avatar>
    }
}

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="bg-background sticky top-0 flex shrink-0 items-center gap-2 p-3 border-b">
            <SidebarTrigger class="-ml-1" />
            <Separator
                orientation=Orientation::Vertical
                class="mr-2 data-[orientation=vertical]:h-4"
            />
            <div class="ml-auto mr-0 space-x-1">
            </div>
        </header>

    }
}
