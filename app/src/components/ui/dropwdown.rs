// Missing Components and Capabilities:
// - DropdownMenuCheckboxItem (No corresponding primitive component)
// - DropdownMenuRadioGroup (No corresponding primitive component)
// - DropdownMenuRadioItem (No corresponding primitive component)
// - DropdownMenuShortcut (No corresponding primitive component)
// - DropdownMenuSub (No corresponding primitive component)
// - DropdownMenuSubTrigger (No corresponding primitive component)
// - DropdownMenuSubContent (No corresponding primitive component)
// - CheckIcon (No icon component provided)
// - CircleIcon (No icon component provided)
// - ChevronRightIcon (No icon component provided)
// - `children` prop on DropdownMenuItem (Primitive MenuItem does not support children)
// - `class` prop on DropdownMenuGroup (Primitive MenuGroup does not support a `class` prop)
// - `class` prop on DropdownMenuLabel (Primitive GroupLabel does not support a `class` prop)
// - `inset` and `variant` props on DropdownMenuItem (Primitive MenuItem does not support these props)
// - `inset` prop on DropdownMenuLabel (Primitive GroupLabel does not support this prop)

use crate::components::primitives::dropdown_menu::{
    DropdownMenuContent as DropdownMenuContentPrimitive,
    DropdownMenuGroup as DropdownMenuGroupPrimitive,
    DropdownMenuGroupLabel as DropdownMenuLabelPrimitive,
    DropdownMenuItem as DropdownMenuItemPrimitive,
    DropdownMenuTrigger as DropdownMenuTriggerPrimitive,
    DropdownPortal as DropdownMenuPortalPrimitive, DropdownProvider as DropdownMenuPrimitive,
    DropdownSeparator as DropdownMenuSeparatorPrimitive,
};
use crate::components::primitives::menu::{MenuAlign, MenuSide};
use leptos::{html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use tailwind_fuse::tw_merge;

#[component]
pub fn DropdownMenu(
    children: Children,
    #[prop(optional, default = true)] modal: bool,
    #[prop(optional, into)] open: RwSignal<bool>,
    #[prop(optional, into)] hidden: RwSignal<bool>,
    #[prop(optional, into)] trigger_ref: NodeRef<html::Div>,
    #[prop(optional, into)] content_ref: NodeRef<html::Div>,
    #[prop(optional)] dismissible: bool,
) -> impl IntoView {
    view! {
        <DropdownMenuPrimitive
            modal=modal
            open=open
            hidden=hidden
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
pub fn DropdownMenuPortal(
    children: ChildrenFn,
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(default = 200)] open_duration: u64,
    #[prop(default = 200)] close_duration: u64,
) -> impl IntoView {
    view! {
        <DropdownMenuPortalPrimitive
            container=container
            container_ref=container_ref
            as_child=as_child
            node_ref=node_ref
            open_duration=open_duration
            close_duration={close_duration}
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
    #[prop(into, optional, default = Signal::derive(|| 4.0))] side_of_set: Signal<f64>,
    #[prop(into, optional, default = Signal::derive(|| MenuAlign::Center))] align: Signal<
        MenuAlign,
    >,
    #[prop(into, optional, default = Signal::derive(|| 0.0))] align_of_set: Signal<f64>,
    #[prop(into, default = None)] limit_y: Option<Signal<f64>>,
    // #[prop(optional)] ignore: Vec<NodeRef<html::Div>>,
    #[prop(optional)] arrow: bool,
) -> impl IntoView {
    let base_class = "bg-popover text-popover-foreground data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-50 max-h-[var(--radix-dropdown-menu-content-available-height)] min-w-[8rem] origin-[var(--radix-menu-content-transform-origin)] overflow-x-hidden overflow-y-auto rounded-md border p-1 shadow-md";
    let children = StoredValue::new(children);
    view! {
        <DropdownMenuPortal close_duration=50>
            <DropdownMenuContentPrimitive
                class=Signal::derive(move || tw_merge!(base_class, class.get()))
                side=side
                side_of_set=side_of_set
                align=align
                align_of_set=align_of_set
                limit_y=limit_y
                // ignore=ignore
                arrow={arrow}
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
    //NOTE: for now, i will use hover and not focus
    let base_class = "hover:bg-accent hover:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:hover:bg-destructive/10 dark:data-[variant=destructive]:hover:bg-destructive/20 data-[variant=destructive]:hover:text-destructive data-[variant=destructive]:*:[svg]:!text-destructive [&_svg:not([class*='text-'])]:text-muted-foreground relative flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden select-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 data-[inset]:pl-8 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4";
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
pub fn DropdownMenuLabel(children: Children) -> impl IntoView {
    view! {
        <DropdownMenuLabelPrimitive
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
