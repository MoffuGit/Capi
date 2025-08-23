use capi_ui::avatar::*;
use capi_ui::card::*;
use common::convex::Member;
use leptos::prelude::*;

#[component]
pub fn MemberCard(member: Member) -> impl IntoView {
    let member = StoredValue::new(member);
    view! {
        <Card {..} class="p-2 gap-2 items-start w-72">
            <CardHeader class="p-0 min-w-0 w-full">
                <UserBanner url=member.get_value().banner_url>
                    <UserImage url=member.get_value().image_url name=member.get_value().name/>
                </UserBanner>
            </CardHeader>
            <CardContent class="px-1">
                <CardTitle class="capitalize">
                    {member.get_value().name}
                </CardTitle>
                // <CardDescription>
                //     {member.status}
                // </CardDescription>
            </CardContent>
        </Card>
    }
}

#[component]
pub fn UserBanner(children: Children, url: Option<String>) -> impl IntoView {
    view! {
        <Avatar class="flex relative bg-muted w-full h-20 items-center justify-center rounded-lg min-w-0">
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
