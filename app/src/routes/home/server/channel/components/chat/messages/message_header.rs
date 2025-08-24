use chrono::{Duration, Local};
use leptos::prelude::*;

use common::convex::Member;

use crate::routes::home::server::channel::components::chat::messages::utils::get_date;
use crate::routes::server::channel::components::sidebar::card::MemberCard;
use capi_ui::avatar::*;
use capi_ui::dropwdown::*;

#[component]
pub fn MessageHeader(member: Member, date: f64) -> impl IntoView {
    let formatted_date = Signal::derive(move || {
        get_date(date).map(|date| {
            let today = Local::now().date_naive();
            let yesterday = (Local::now() - Duration::days(1)).date_naive();
            let message_date = date.date_naive();

            if message_date == today {
                format!("today at {}", date.format("%I:%M %p"))
            } else if message_date == yesterday {
                format!("yesterday at {}", date.format("%I:%M %p"))
            } else {
                date.format("%m/%d/%y, %I:%M %p").to_string()
            }
        })
    });

    let member = StoredValue::new(member);

    view! {
        <div class="flex items-center gap-1 -translate-x-3">
            <DropdownMenu>
                <DropdownMenuTrigger
                    class="cursor-pointer active:scale-[.97]"
                    {..}
                    on:dblclick=move |evt| {
                        evt.stop_propagation();
                    }
                >
                    <Avatar class="flex bg-accent aspect-square size-6 items-center justify-center rounded-md group-data-[state=collapsed]:opacity-0 group-data-[state=expanded]:opacity-100 ease-in-out duration-150 transition-opacity">
                        <AvatarImage url=member.get_value().image_url.clone()/>
                        <AvatarFallback class="rounded-lg select-none bg-transparent">
                            {member.get_value().name.chars().next()}
                        </AvatarFallback>
                    </Avatar>
                </DropdownMenuTrigger>
                <DropdownMenuContent
                    side=DropdownMenuSide::Right
                    align=DropdownMenuAlign::Start
                >
                    <MemberCard member=member.get_value() />
                </DropdownMenuContent>
            </DropdownMenu>
            <div>
                <span class="font-medium">
                    {member.get_value().name}
                </span>
                <span class="text-muted-foreground text-xs ml-1">
                    {formatted_date}
                </span>
            </div>
        </div>
    }
}
