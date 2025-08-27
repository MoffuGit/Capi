use leptos::prelude::*;
use tailwind_fuse::tw_merge;

use crate::components::ui::sidebar::*;

use super::ConversationDetails;

#[component]
pub fn ConversationItems(
    conversations: ReadSignal<Option<Result<Vec<ConversationDetails>, String>>>,
) -> impl IntoView {
    // let location = use_location();
    // let path = location.pathname;
    // let current_channel = Memo::new(move |_| {
    //     path.get()
    //         .split('/')
    //         .nth(3)
    //         .map(|channel| channel.to_string())
    // });
    let conversations = Signal::derive(move || conversations.get().and_then(|res| res.ok()));
    view! {
        <SidebarMenu>
            <For
                each=move || conversations.get().unwrap_or_default()
                key=|conversation| conversation._id.clone()
                let:conversation
            >
                <ConversationItem conversation=conversation/>
            </For>
        </SidebarMenu>
    }
}

#[component]
pub fn ConversationItem(conversation: ConversationDetails) -> impl IntoView {
    let receiver = StoredValue::new(conversation.other_member);
    let id = StoredValue::new(conversation._id);
    view! {
        <SidebarMenuItem>
            // <A href=move || format!("/servers/{}/{}", )>
                <SidebarMenuButton
                    // is_active=Signal::derive(
                    //     move || {
                    //         current_channel.get().is_some_and(|curr| {
                    //              id.get_value() == curr
                    //         })
                    //     }
                    // )
                    class="group/button group-data-[collapsible=icon]:size-auto! group-data-[collapsible=icon]:h-8! group-data-[collapsible=icon]:p-2!">
                    <span
                        class=tw_merge!(
                            "text-sidebar-foreground/70 inline-flex flex-col items-start font-normal",
                            "group-data-[active=true]/button:font-bold group-hover/button:text-sidebar-foreground",
                            "transition-[color,font-weight] duration-150 ease-out",
                            "after:content-[attr(data-text)] after:h-0 after:hidden after:overflow-hidden after:select-none after:pointer-events-none after:font-bold"
                        )
                    >
                        {receiver.get_value().name}
                    </span>
                </SidebarMenuButton>
            // </A>
        </SidebarMenuItem>

    }
}
