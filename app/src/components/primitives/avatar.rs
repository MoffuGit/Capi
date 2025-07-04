use leptos::context::Provider;
use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq)] // Added PartialEq for comparisons
pub enum ImageLoadingStatus {
    Idle,
    Loading,
    Loaded,
    Error,
}

#[component]
pub fn AvatarRoot(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let status = RwSignal::new(ImageLoadingStatus::Idle);
    view! {
        <span class=class>
            <Provider value=status>
                {children()}
            </Provider>
        </span>
    }
}

#[component]
pub fn AvatarImage(
    image_url: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let status = use_context::<RwSignal<ImageLoadingStatus>>()
        .expect("AvatarImage expects an AvatarRoot context provider for ImageLoadingStatus");

    // Effect to set status to Loading whenever the image_url changes or component mounts
    Effect::new(move |prev_url: Option<MaybeProp<String>>| {
        // If the URL has changed (or it's the initial run), set status to Loading.
        // This ensures the loading state is active when a new image attempt begins.
        if prev_url.is_none() || prev_url.is_some_and(|url| url.get() != image_url.get()) {
            status.set(ImageLoadingStatus::Loading);
        }
        image_url
    });

    view! {
        <img
            src=image_url
            class=class
            // Update status when image loads successfully
            on:load=move |_| status.set(ImageLoadingStatus::Loaded)
            // Update status when image fails to load
            on:error=move |_| status.set(ImageLoadingStatus::Error)
            // Hide the image if it's not loaded yet or has errored,
            // allowing the fallback component to be visible.
            class:hidden=move || !matches!(status.get(), ImageLoadingStatus::Loaded)
        />
    }
}

#[component]
pub fn AvatarFallback(
    children: ChildrenFn,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let status = use_context::<RwSignal<ImageLoadingStatus>>()
        .expect("AvatarFallback expects an AvatarRoot context provider for ImageLoadingStatus");

    view! {
        <Show when=move || matches!(status.get(), ImageLoadingStatus::Loading | ImageLoadingStatus::Error)>
            <span class=class>
                {children()}
            </span>
        </Show>
    }
}
