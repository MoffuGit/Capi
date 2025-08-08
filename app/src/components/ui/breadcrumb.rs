use leptos::prelude::*;
use leptos_router::components::A;
use tailwind_fuse::tw_merge;

use crate::components::icons::IconEllipsis;

#[component]
pub fn Breadcrumb(children: Children) -> impl IntoView {
    view! {
        <nav
            aria-label="breadcrumb"
        >
            {children()}
        </nav>
    }
}

#[component]
pub fn BreadcrumbList(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <ol
            data-slot="breadcrumb-list"
            class=move || tw_merge!("text-muted-foreground flex flex-wrap items-center gap-1.5 text-sm break-words sm:gap-2.5", class.get())>
            {children()}
        </ol>
    }
}

#[component]
pub fn BreadcrumbItem(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <li
            data-slot="breadcrumb-item"
            class=move || tw_merge!("inline-flex items-center gap-1.5", class.get())
        >
            {children()}
        </li>
    }
}

#[component]
pub fn BreadcrumbLink(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
    #[prop(into, optional)] href: Signal<String>,
) -> impl IntoView {
    view! {
        <A
            href=move || href.get()
            {..}
            class=move || tw_merge!("hover:text-foreground transition-colors", class.get())
            data-slot="breadcrumb-link"
        >
            {children()}
        </A>
    }
}

#[component]
pub fn BreadcrumbPage(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
    <span
      data-slot="breadcrumb-page"
      role="link"
      aria-disabled="true"
      aria-current="page"
      class=move || tw_merge!("text-foreground font-normal", class.get())
    >
        {children()}
    </span>
    }
}

#[component]
pub fn BreadcrumbSeparator(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <li
          data-slot="breadcrumb-separator"
          role="presentation"
          aria-hidden="true"
          class=move || tw_merge!("[&>svg]:size-3.5", class.get())
        >
          {children.map(|children| children())}
        </li>
    }
}

#[component]
pub fn BreadcrumbEllipsis(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <span
          data-slot="breadcrumb-ellipsis"
          role="presentation"
          aria-hidden="true"
          class=move || tw_merge!("flex size-9 items-center justify-center", class.get())
        >
            {children()}
            <IconEllipsis class="size-4" />
            <span class="sr-only">More</span>
        </span>

    }
}
