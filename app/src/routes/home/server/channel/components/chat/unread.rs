use capi_ui::button::{Button, ButtonSizes, ButtonVariants};
use common::convex::ChannelMessage;
use leptos::prelude::*;

#[component]
pub fn UnreadMessagesButton(
    unread_count: Signal<f64>,
    last_read_message: Signal<Option<ChannelMessage>>,
    scroll_to_message: Callback<String>,
) -> impl IntoView {
    view! {
        <Show
            when=move || (unread_count.get() > 0.0)
        >
            <Button
                variant=ButtonVariants::Secondary
                size=ButtonSizes::Sm
                class="absolute top-4 left-1/2 -translate-x-1/2 z-10 w-full mx-4"
                on:click=move |_| {
                    if let Some(msg) = last_read_message.get() {
                        scroll_to_message.run(msg.id);
                    }
                }
            >
                {move || format!("{} new messages", unread_count.get())}
            </Button>
        </Show>
    }
}
