mod conversations;

use convex_client::leptos::{Query, UseQuery};
use icons::IconContact;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use tailwind_fuse::tw_merge;

use crate::components::auth::use_auth;
use crate::components::ui::sidebar::*;

use self::conversations::ConversationItems;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct GetMyConversations {
    pub auth: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OtherMemberDetails {
    pub _id: String,
    pub name: String,
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LastMessageSummary {
    pub content: String,
    #[serde(rename = "_creationTime")]
    pub _creation_time: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConversationDetails {
    pub _id: String,
    #[serde(rename = "otherMember")]
    pub other_member: OtherMemberDetails,
    #[serde(rename = "lastMessage")]
    pub last_message: Option<LastMessageSummary>,
    #[serde(rename = "unreadCount")]
    pub unread_count: i64,
}

impl Query<Vec<ConversationDetails>> for GetMyConversations {
    fn name(&self) -> String {
        "privateConversations:getMyConversations".to_string()
    }
}

#[component]
pub fn PrivateSideBar() -> impl IntoView {
    let auth = use_auth().auth;
    let conversations = UseQuery::new(move || {
        auth.get()
            .and_then(|auth| auth.ok())
            .flatten()
            .map(|auth| GetMyConversations { auth: auth.id })
    });
    view! {
        <SidebarContent>
            <SidebarGroup>
                <SidebarGroupContent>
                    <SidebarMenuButton class="group">
                        <IconContact />
                        <span
                            class=tw_merge!(
                                "text-sidebar-foreground/70 inline-flex flex-col items-start font-normal",
                                "group-data-[active=true]/button:font-bold group-hover/button:text-sidebar-foreground",
                                "transition-[color,font-weight] duration-150 ease-out",
                                "after:content-[attr(data-text)] after:h-0 after:hidden after:overflow-hidden after:select-none after:pointer-events-none after:font-bold"
                            )
                        >
                            "Friends"
                        </span>
                    </SidebarMenuButton>
                </SidebarGroupContent>
            </SidebarGroup>
            <SidebarGroup>
                <SidebarGroupLabel>
                    "Direct Messages"
                </SidebarGroupLabel>
                <SidebarGroupContent>
                    <ConversationItems conversations=conversations/>
                </SidebarGroupContent>
            </SidebarGroup>
        </SidebarContent>
    }
}
