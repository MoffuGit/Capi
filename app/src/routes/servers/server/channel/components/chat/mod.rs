mod dropzone;
mod messages;

use leptos::prelude::*;

use crate::components::icons::{IconCirclePlus, IconSticker};
use crate::components::ui::button::{Button, ButtonSizes, ButtonVariants};
use common::convex::{Channel, ChannelMessage};
use leptos::context::Provider;

use self::messages::Messages;

#[derive(Debug, Clone, Default)]
pub struct ChatContext {
    pub msg_reference: RwSignal<Option<ChannelMessage>>,
    pub attachments: RwSignal<Vec<String>>,
}

#[component]
pub fn Chat(channel: RwSignal<Option<Channel>>) -> impl IntoView {
    let chat_context = ChatContext::default();
    view! {
        <Provider value=chat_context>
            // <ChatDropZone/>
            <div class="flex h-full w-full flex-col">
                <div class="flex min-h-0 flex-1 flex-col gap-2 overflow-auto">
                    <Messages channel=channel/>
                </div>
                <div class="flex flex-col gap-2 p-5">
                    <div class="border-input dark:bg-input/30 flex w-full rounded-md border bg-transparent px-3 py-2 text-base shadow-xs md:text-sm justify-between">
                        <div class="flex items-center justify-center">
                            <Button size=ButtonSizes::Icon variant=ButtonVariants::Ghost>
                                <IconCirclePlus/>
                            </Button>
                        </div>
                        <div>
                        </div>
                        <div class="flex items-center">
                            <Button size=ButtonSizes::Icon variant=ButtonVariants::Ghost>
                                <IconSticker/>
                            </Button>
                        </div>
                    </div>
                </div>
            </div>
        </Provider>
    }
}
