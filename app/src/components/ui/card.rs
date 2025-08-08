use leptos::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn Card(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <div
          data-slot="card"
              class=tw_merge!(
                "bg-card text-card-foreground flex flex-col gap-6 rounded-xl border py-6 shadow-sm",
                class
              )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CardHeader(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <div
            data-slot="card-header"
            class=tw_merge!(
            "@container/card-header grid auto-rows-min grid-rows-[auto_auto] items-start gap-1.5 px-6 has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6",
            class
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CardTitle(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <div
            data-slot="card-title"
            class=tw_merge!(
            "leading-none font-semibold",
            class
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CardDescription(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <div
            data-slot="card-description"
            class=tw_merge!(
            "text-muted-foreground text-sm",
            class
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CardAction(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <div
            data-slot="card-action"
            class=tw_merge!(
            "col-start-2 row-span-2 row-start-1 self-start justify-self-end",
            class
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CardContent(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <div
            data-slot="card-content"
            class=tw_merge!(
            "px-6",
            class
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CardFooter(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <div
            data-slot="card-footer"
            class=tw_merge!(
            "flex items-center px-6 [.border-t]:pt-6",
            class
            )
        >
            {children()}
        </div>
    }
}
