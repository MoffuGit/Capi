use common::convex::ChannelMessage;
use common::convex::Member;
use convex_client::leptos::UseMutation;
use emojis::Emoji;
use icons::IconCornerUpLeft;
use icons::IconSmile;
use leptos::prelude::*;

use crate::components::emojis::EmojiSelector;
use crate::routes::server::channel::components::chat::messages::message_reactions::AddReaction;
use crate::routes::server::channel::components::chat::ChatContext;
use capi_ui::button::*;
use capi_ui::dropwdown::*;

#[component]
pub fn MessageActions(
    msg: StoredValue<ChannelMessage>,
    member: Signal<Option<Member>>,
) -> impl IntoView {
    let context: ChatContext = use_context().expect("should return teh chat context");
    let active = RwSignal::new(false);
    let add_reaction = UseMutation::new::<AddReaction>();
    let on_select_emoji = Callback::new(move |emoji: &'static Emoji| {
        if let Some(member) = member.get() {
            add_reaction.dispatch(AddReaction {
                message: msg.get_value().id,
                member: member.id,
                emoji: emoji.to_string(),
            });
        }
    });
    view! {
        <div data-active=move || active.get().to_string() class="absolute data-[active=true]:opacity-100 bg-popover gap-1 text-popover-foreground flex items-center h-auto z-10 w-auto overflow-x-hidden overflow-y-auto rounded-md border p-1 shadow-md group-hover:opacity-100 opacity-0 top-0 right-4 -translate-y-1/2">
            <DropdownMenu on_close=move || active.set(false)>
                <DropdownMenuTrigger>
                    <Button
                        variant=ButtonVariants::Ghost
                        size=ButtonSizes::IconXs
                        on:click=move |evt| {
                            evt.prevent_default();
                            active.set(true);
                        }
                    >
                        <IconSmile />
                    </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent side=DropdownMenuSide::Left align=DropdownMenuAlign::Start side_of_set=10.0 class="max-w-full p-0">
                    <EmojiSelector history=context.reactions class="p-1" on_select_emoji=on_select_emoji/>
                </DropdownMenuContent>
            </DropdownMenu>
            <MessageReferenceButton msg=msg.get_value()/>
        </div>
    }
}

#[component]
pub fn MessageReferenceButton(msg: ChannelMessage) -> impl IntoView {
    let ChatContext { msg_reference, .. } =
        use_context::<ChatContext>().expect("should access to the chat context");
    view! {
        <Button
            variant=ButtonVariants::Ghost
            size=ButtonSizes::IconXs
            on:click=move |_| {
                msg_reference.set(Some(msg.clone()));
            }
        >
            <IconCornerUpLeft />
        </Button>
    }
}
