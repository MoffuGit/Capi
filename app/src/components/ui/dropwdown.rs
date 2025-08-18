use capi_primitives::dropdown_menu::{
    DropdownMenuContent as DropdownMenuContentPrimitive,
    DropdownMenuGroup as DropdownMenuGroupPrimitive,
    DropdownMenuGroupLabel as DropdownMenuLabelPrimitive,
    DropdownMenuItem as DropdownMenuItemPrimitive,
    DropdownMenuSubPortal as DropdownMenuSubPortalPrimitive,
    DropdownMenuTrigger as DropdownMenuTriggerPrimitive,
    DropdownPortal as DropdownMenuPortalPrimitive, DropdownProvider as DropdownMenuPrimitive,
    DropdownRadioGroupProvider as DropdownRadioGroupProviderPrimitive,
    DropdownRadioItem as DropdownRadioItemPrimitive,
    DropdownRadioItemIndicator as DropdownRadioItemIndicatorPrimitive,
    DropdownSeparator as DropdownMenuSeparatorPrimitive,
    DropdownSubMenuContent as DropdownMenuSubContentPrimitive,
    DropdownSubMenuProvider as DropdownMenuSubProviderPrimitive, DropdownSubMenuTrigger,
};
use capi_primitives::menu::{MenuAlign, MenuSide};
use leptos::{html, prelude::*};
use tailwind_fuse::tw_merge;

use icons::{IconChevronRight, IconCircle};

#[component]
pub fn DropdownMenu(
    children: Children,
    #[prop(optional, default = true)] modal: bool,
    #[prop(optional, into)] open: RwSignal<bool>,
    #[prop(optional, into)] trigger_ref: NodeRef<html::Div>,
    #[prop(optional, into)] content_ref: NodeRef<html::Div>,
    #[prop(optional, default = true)] dismissible: bool,
    #[prop(optional, into)] on_close: Option<Callback<()>>,
) -> impl IntoView {
    view! {
        <DropdownMenuPrimitive
            on_close=on_close
            modal=modal
            open=open
            trigger_ref=trigger_ref
            content_ref=content_ref
            dismissible={dismissible}
            {..}
            data-slot="dropdown-menu"
        >
            {children()}
        </DropdownMenuPrimitive>
    }
}

#[component]
pub fn DropdownMenuPortal(children: ChildrenFn) -> impl IntoView {
    view! {
        <DropdownMenuPortalPrimitive
            {..}
            data-slot="dropdown-menu-portal"
        >
            {children()}
        </DropdownMenuPortalPrimitive>
    }
}

#[component]
pub fn DropdownMenuTrigger(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <DropdownMenuTriggerPrimitive
            class={class}
            {..}
            data-slot="dropdown-menu-trigger"
        >
            {children.map(|c| c())}
        </DropdownMenuTriggerPrimitive>
    }
}

#[component]
pub fn DropdownMenuContent(
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
    #[prop(into, optional, default = Signal::derive(|| MenuSide::Bottom))] side: Signal<MenuSide>,
    #[prop(into, optional, default = Signal::derive(|| 0.0))] side_of_set: Signal<f64>,
    #[prop(into, optional, default = Signal::derive(|| MenuAlign::Center))] align: Signal<
        MenuAlign,
    >,
    #[prop(into, optional, default = Signal::derive(|| 0.0))] align_of_set: Signal<f64>,
) -> impl IntoView {
    let base_class = "bg-popover text-popover-foreground data-[state=closed]:invisible data-[state=opening]:animate-in data-[state=closing]:animate-out data-[state=closing]:fade-out-0 data-[state=opening]:fade-in-0 data-[state=closing]:zoom-out-95 data-[state=opening]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-50 max-h-[var(--radix-dropdown-menu-content-available-height)] min-w-[12rem] origin-[var(--radix-menu-content-transform-origin)] overflow-x-hidden overflow-y-auto rounded-md border p-1 shadow-md duration-150";
    let children = StoredValue::new(children);
    view! {
        <DropdownMenuPortal>
            <DropdownMenuContentPrimitive
                side=side
                side_of_set=side_of_set
                align=align
                align_of_set=align_of_set
                class=Signal::derive(move || tw_merge!(base_class, class.get()))
                {..}
                data-slot="dropdown-menu-content"
            >
                {children.get_value()()}
            </DropdownMenuContentPrimitive>
        </DropdownMenuPortal>
    }
}

#[component]
pub fn DropdownMenuGroup(children: Children) -> impl IntoView {
    view! {
        <DropdownMenuGroupPrimitive
            {..}
            data-slot="dropdown-menu-group"
        >
            {children()}
        </DropdownMenuGroupPrimitive>
    }
}

#[component]
pub fn DropdownMenuItem(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] close_on_click: bool,
    children: Children,
) -> impl IntoView {
    let base_class = "text-muted-foreground hover:bg-accent hover:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:hover:bg-destructive/10 dark:data-[variant=destructive]:hover:bg-destructive/20 data-[variant=destructive]:hover:text-destructive data-[variant=destructive]:*:[svg]:!text-destructive [&_svg:not([class*='text-'])]:text-muted-foreground relative flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden select-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 data-[inset]:pl-8 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4";
    view! {
        <DropdownMenuItemPrimitive
            class=Signal::derive(move || tw_merge!(base_class, class.get()))
            close_on_click={close_on_click}
            {..}
            data-slot="dropdown-menu-item"
        >
            {children()}
        </DropdownMenuItemPrimitive>
    }
}

#[component]
pub fn DropdownMenuLabel(
    children: Children,
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    view! {
        <DropdownMenuLabelPrimitive
            class=Signal::derive(move || tw_merge!(
                "text-sidebar-foreground/70 ring-sidebar-ring flex h-auto shrink-0 items-center rounded-md px-2 py-1 text-xs font-medium outline-hidden transition-[margin,opacity] duration-200 ease-linear focus-visible:ring-2 [&>svg]:size-4 [&>svg]:shrink-0",
                "group-data-[collapsible=icon]:-mt-8 group-data-[collapsible=icon]:opacity-0",
                class.get()
            ))
            {..}
            data-slot="dropdown-menu-label"
        >
            {children()}
        </DropdownMenuLabelPrimitive>
    }
}

#[component]
pub fn DropdownMenuSeparator(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    let base_class = "bg-border -mx-1 my-1 h-px";
    view! {
        <DropdownMenuSeparatorPrimitive
            class=Signal::derive(move || tw_merge!(base_class, class.get()))
            {..}
            data-slot="dropdown-menu-separator"
        />
    }
}

#[component]
pub fn DropdownMenuRadioGroup(
    children: Children,
    #[prop(into)] value: RwSignal<String>,
) -> impl IntoView {
    view! {
        <DropdownRadioGroupProviderPrimitive value=value>
            {children()}
        </DropdownRadioGroupProviderPrimitive>
    }
}

#[component]
pub fn DropdownMenuRadioItem(
    children: ChildrenFn,
    #[prop(into)] value: Signal<String>,
) -> impl IntoView {
    view! {
        <DropdownRadioItemPrimitive
            value=value
            class=tw_merge!("text-muted-foreground hover:bg-accent hover:text-accent-foreground relative flex cursor-default items-center gap-2 rounded-sm py-1.5 pr-2 pl-8 text-sm outline-hidden select-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4")
            {..}
            data-slot="context-menu-radio-item"
        >
            <span class="pointer-events-none absolute left-2 flex size-3.5 items-center justify-center">
                <DropdownRadioItemIndicatorPrimitive>
                    <IconCircle class="size-2 fill-current" />
                </DropdownRadioItemIndicatorPrimitive>
            </span>
            {children()}

        </DropdownRadioItemPrimitive>
    }
}

#[component]
pub fn DropdownMenuSub(children: Children) -> impl IntoView {
    view! {
        <DropdownMenuSubProviderPrimitive
        >
            {children()}
        </DropdownMenuSubProviderPrimitive>
    }
}

#[component]
pub fn DropdownMenuSubTrigger(
    children: Children,
    #[prop(optional, into, default = Signal::derive(move || true))] open_on_hover: Signal<bool>,
) -> impl IntoView {
    view! {
        <DropdownSubMenuTrigger
            open_on_hover=open_on_hover
            class=Signal::derive(move || tw_merge!("text-muted-foreground hover:bg-accent hover:text-accent-foreground data-[state=open]:bg-accent data-[state=open]:text-accent-foreground flex cursor-default items-center rounded-sm px-2 py-1.5 text-sm outline-hidden select-none data-[inset]:pl-8 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4"))
            {..}
            data-slot="dropdown-menu-sub-trigger"
        >
            {children()}
            <IconChevronRight class="ml-auto" />
        </DropdownSubMenuTrigger>
    }
}

#[component]
pub fn DropdownMenuSubContent(
    children: ChildrenFn,
    #[prop(into, optional, default = Signal::derive(|| MenuSide::Bottom))] side: Signal<MenuSide>,
    #[prop(into, optional, default = Signal::derive(|| 4.0))] side_of_set: Signal<f64>,
    #[prop(into, optional, default = Signal::derive(|| MenuAlign::Center))] align: Signal<
        MenuAlign,
    >,
    #[prop(into, optional, default = Signal::derive(|| 0.0))] align_of_set: Signal<f64>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <DropdownMenuSubPortalPrimitive>
            <DropdownMenuSubContentPrimitive
                side=side
                side_of_set=side_of_set
                align=align
                align_of_set=align_of_set
                class=Signal::derive(move || tw_merge!("bg-popover text-popover-foreground data-[state=opening]:animate-in data-[state=closing]:animate-out data-[state=closing]:fade-out-0 data-[state=opening]:fade-in-0 data-[state=closing]:zoom-out-95 data-[state=opening]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-50 min-w-[8rem] origin-[var(--radix-menu-content-transform-origin)] overflow-hidden rounded-md border p-1 shadow-lg duration-150 ease-out-quad"))
                {..}
                data-slot="dropdown-menu-sub-content"
            >
                {children.get_value()()}
            </DropdownMenuSubContentPrimitive>
        </DropdownMenuSubPortalPrimitive>
    }
}
