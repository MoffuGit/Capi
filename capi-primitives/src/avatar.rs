use leptos::context::Provider;
use leptos::html;
use leptos::prelude::*; // Import for html::Img

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

    let img_ref: NodeRef<html::Img> = NodeRef::new();

    Effect::new(move |prev_url: Option<MaybeProp<String>>| {
        if prev_url.is_none() || prev_url.is_some_and(|url| url.get() != image_url.get()) {
            status.set(ImageLoadingStatus::Loading);
        }
        image_url
    });

    Effect::new(move |_| {
        if image_url.get().is_some() {
            if let Some(img_element) = img_ref.get() {
                let current_status = status.get_untracked();

                if !matches!(
                    current_status,
                    ImageLoadingStatus::Loaded | ImageLoadingStatus::Error
                ) && img_element.complete()
                {
                    status.set(ImageLoadingStatus::Loaded);
                }
            }
        }
    });

    view! {
        <img
            node_ref=img_ref
            src=image_url
            class=class
            on:load=move |_| {
                status.set(ImageLoadingStatus::Loaded);
            }
            on:error=move |_| {
                status.set(ImageLoadingStatus::Error);
            }
            class:hidden=move || {
                let current_status = status.get();
                !matches!(current_status, ImageLoadingStatus::Loaded)
            }
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
