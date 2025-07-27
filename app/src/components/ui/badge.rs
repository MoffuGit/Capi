use leptos::prelude::*;
use tailwind_fuse::AsTailwindClass;
use tailwind_fuse::{tw_merge, TwVariant};

#[derive(Debug, PartialEq, TwVariant)]
pub enum BadgeVariant {
    #[tw(
        default,
        class = "border-transparent bg-primary text-primary-foreground [a&]:hover:bg-primary/90"
    )]
    Default,
    #[tw(
        class = "border-transparent bg-secondary text-secondary-foreground [a&]:hover:bg-secondary/90"
    )]
    Secondary,
    #[tw(
        class = "border-transparent bg-destructive text-white [a&]:hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60"
    )]
    Destructive,
    #[tw(class = "text-foreground [a&]:hover:bg-accent [a&]:hover:text-accent-foreground")]
    Outline,
    //Status
    #[tw(
        class = "border-transparent bg-lime-50 text-secondary-foreground [a&]:hover:bg-secondary/90"
    )]
    Online,
    #[tw(
        class = "border-transparent bg-lime-300 text-secondary-foreground [a&]:hover:bg-secondary/90"
    )]
    Idle,
    #[tw(
        class = "border-transparent bg-lime-300 text-secondary-foreground [a&]:hover:bg-secondary/90"
    )]
    NotDisturb,
    #[tw(
        class = "border-transparent bg-lime-300 text-secondary-foreground [a&]:hover:bg-secondary/90"
    )]
    Invisible,
}

#[component]
pub fn Badge(
    children: Children,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] variant: Signal<BadgeVariant>,
) -> impl IntoView {
    view! {
        <div
            class=Signal::derive(move || {
                tw_merge!(
                    "inline-flex items-center justify-center rounded-md border px-2 py-0.5 text-xs font-medium w-fit whitespace-nowrap shrink-0 [&>svg]:size-3 gap-1 [&>svg]:pointer-events-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive transition-[color,box-shadow] overflow-hidden",
                    variant.get(),
                    class.get()
                )
            })
            data-slot="badge"
        >
            {children()}
        </div>
    }
}
