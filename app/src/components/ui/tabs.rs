use leptos::prelude::*;
use tailwind_fuse::tw_merge;

use crate::components::primitives::common::Orientation;
use crate::components::primitives::tabs::{
    Panel as TabsPanelPrmitive, Tab as TabPrimitive, TabIndicator as TabIndicatorPrimitive,
    TabsList as TabsListPrimitive, TabsRoot as TabsRooutPrimitive,
};

#[component]
pub fn Tabs(
    #[prop(optional, into)] orientation: Orientation,
    #[prop(optional, into)] tab: RwSignal<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <TabsRooutPrimitive
            orientation=orientation
            tab=tab
            class=MaybeProp::derive(move || {
                Some(tw_merge!(
                    "flex flex-col gap-2",
                    class.get()
                ))
            })
            {..}
            data-slot="tabs"
        >
            {children()}
        </TabsRooutPrimitive>
    }
}

#[component]
pub fn TabsList(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <TabsListPrimitive
            class=MaybeProp::derive(move || Some(tw_merge!("relative text-muted-foreground inline-flex h-9 w-fit items-center justify-center rounded-lg p-[3px]", class.get())))
            {..}
            data-slot="tabs-list"
        >
            {children()}
        </TabsListPrimitive>
    }
}

#[component]
pub fn Tab(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into, default = true)] set_on_click: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <TabPrimitive
            set_on_click=set_on_click
            value=value
            class=MaybeProp::derive(move || {
                Some(tw_merge!(
                    "dark:data-[state=active]:text-foreground focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:outline-ring select-none",
                    "text-foreground dark:text-muted-foreground inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center gap-1.5 rounded-md border border-transparent px-2 py-1",
                    "text-sm font-medium whitespace-nowrap transition-[color,box-shadow] ease-in-out focus-visible:ring-[3px] focus-visible:outline-1 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
                    class.get()
                ))
            })
            {..}
             data-slot="tabs-tab"
        >
            {children()}
        </TabPrimitive>
    }
}

#[component]
pub fn TabPanel(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: MaybeProp<String>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <TabsPanelPrmitive
            value=value
            class=MaybeProp::derive(move || {
                Some(tw_merge!(
                    "flex-1 outline-none",
                    class.get()
                ))
            })
        >
            {children()}
        </TabsPanelPrmitive>
    }
}

#[component]
pub fn TabIndicator() -> impl IntoView {
    view! {
        <TabIndicatorPrimitive class="flex items-end transition-transform ease-in-out">
            <div class="mt-auto w-full h-0.5 bg-foreground"/>
        </TabIndicatorPrimitive>
    }
}
