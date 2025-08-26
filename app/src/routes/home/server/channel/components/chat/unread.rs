use chrono::{DateTime, Datelike, Local};
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
            <div class="absolute top-0 left-1/2 -translate-x-1/2 z-10 w-full px-4 flex items-center justify-center">
                <div
                    class="w-full h-full rounded-b-md bg-primary border-border hover:bg-primary/80 text-primary-foreground text-[13px] py-1 px-3"
                    on:click=move |_| {
                        if let Some(msg) = last_read_message.get() {
                            scroll_to_message.run(msg.id);
                        }
                    }
                >
                    {move || {
                        let count = unread_count.get();
                        let message_plural = if count == 1.0 { "message" } else { "messages" };
                        let mut display_message = format!("{count:.0} new {message_plural}");

                        if let Some(msg) = last_read_message.get() {
                            let seconds = (msg.creation_time / 1000.0) as i64;
                            let nanos = ((msg.creation_time.fract() * 1_000_000_000.0) as u32).min(999_999_999);

                            if let Some(datetime_utc) = chrono::DateTime::from_timestamp(seconds, nanos) {
                                let local_datetime: DateTime<Local> = datetime_utc.with_timezone(&Local);
                                let now: DateTime<Local> = Local::now();

                                let formatted_date = if local_datetime.year() == now.year() && local_datetime.ordinal() == now.ordinal() {
                                    local_datetime.format("%I:%M %p").to_string()
                                } else {
                                    local_datetime.format("%b %d, %Y %I:%M %p").to_string()
                                };
                                display_message.push_str(&format!(" since {formatted_date}"));
                            }
                        }
                        display_message
                    }}
                </div>
            </div>
        </Show>
    }
}
