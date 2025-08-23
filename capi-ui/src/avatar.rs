use leptos::prelude::*;
use tailwind_fuse::tw_merge;

use capi_primitives::avatar::AvatarFallback as AvatarFallbackPrimitive;
use capi_primitives::avatar::AvatarImage as AvatarImagePrimitive;
use capi_primitives::avatar::AvatarRoot as AvatarPrimitive;

#[component]
pub fn Avatar(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <AvatarPrimitive
            class=MaybeProp::derive(move || Some(tw_merge!("relative flex size-8 shrink-0 overflow-hidden rounded-full", class.get())))
        >
            {children()}
        </AvatarPrimitive>
    }
}

#[component]
pub fn AvatarImage(
    #[prop(optional, into)] url: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <AvatarImagePrimitive
            image_url=url
            class=MaybeProp::derive(move || Some(tw_merge!("aspect-square size-full object-cover", class.get())))
        />
    }
}

#[component]
pub fn AvatarFallback(
    children: ChildrenFn,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <AvatarFallbackPrimitive
            class=MaybeProp::derive(move || Some(tw_merge!("bg-muted flex size-full items-center justify-center rounded-full", class.get())))
        >
            {children()}
        </AvatarFallbackPrimitive>
    }
}
