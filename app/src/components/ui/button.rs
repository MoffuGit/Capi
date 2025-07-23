use leptos::prelude::*;
use tailwind_fuse::*;

#[derive(Debug, PartialEq, TwVariant)]
pub enum ButtonVariants {
    #[tw(
        default,
        class = "bg-primary text-primary-foreground shadow-xs hover:bg-primary/90"
    )]
    Default,
    #[tw(
        class = "bg-destructive text-white shadow-xs hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60"
    )]
    Destructive,
    #[tw(
        class = "border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50"
    )]
    Outline,
    #[tw(class = "bg-secondary text-secondary-foreground shadow-xs hover:bg-secondary/80")]
    Secondary,
    #[tw(class = "hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50")]
    Ghost,
    #[tw(class = "text-primary underline-offset-4 hover:underline")]
    Link,
}

#[derive(Debug, PartialEq, TwVariant)]
pub enum ButtonSizes {
    #[tw(default, class = "h-9 px-4 py-2 has-[>svg]:px-3")]
    Default,
    #[tw(class = "h-8 rounded-md gap-1.5 px-3 has-[>svg]:px-2.5")]
    Sm,
    #[tw(class = "h-10 rounded-md px-6 has-[>svg]:px-4")]
    Lg,
    #[tw(class = "size-8")]
    Icon,
}

#[component]
pub fn Button(
    #[prop(optional, into)] variant: Signal<ButtonVariants>,
    #[prop(optional, into)] size: Signal<ButtonSizes>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into, default = Signal::from(false))] disabled: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Memo::new(move |_| {
        tw_merge!(
            "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
            variant.get(),
            size.get(),
            class.get()
        )
    });

    view! {
        <button
            data-slot="button"
            class=computed_class
            disabled=disabled
        >
            {children.map(|children| children())}
        </button>
    }
}
