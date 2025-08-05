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

use capi_primitives::context_menu::{
    ContextMenuContent as ContextMenuContentPrimitive,
    ContextMenuGroup as ContextMenuGroupPrimitive,
    ContextMenuGroupLabel as ContextMenuGroupLabelPrimitive,
    ContextMenuItem as ContextMenuItemPrimitive, ContextMenuTrigger as ContextMenuTriggerPrimitive,
    ContextPortal as ContextMenuPortalPrimitive, ContextProvider as ContextMenuPrimitive,
    ContextSeparator as ContextMenuSeparatorPrimitive,
};
use capi_primitives::menu::{MenuAlign, MenuSide};
use leptos::{html, prelude::*};
use tailwind_fuse::tw_merge;

#[component]
pub fn ContextMenu(
    children: Children,
    #[prop(optional, default = true)] modal: bool,
    #[prop(optional, into)] open: RwSignal<bool>,
    #[prop(optional, into)] hidden: RwSignal<bool>,
    #[prop(optional, into)] trigger_ref: NodeRef<html::Div>,
    #[prop(optional, into)] content_ref: NodeRef<html::Div>,
    #[prop(optional)] dismissible: bool,
) -> impl IntoView {
    view! {
        <ContextMenuPrimitive
            modal=modal
            open=open
            hidden=hidden
            trigger_ref=trigger_ref
            content_ref=content_ref
            dismissible=dismissible
        >
            {children()}
        </ContextMenuPrimitive>
    }
}

#[component]
pub fn ContextMenuPortal(children: ChildrenFn) -> impl IntoView {
    view! {
        <ContextMenuPortalPrimitive>
            {children()}
        </ContextMenuPortalPrimitive>
    }
}

#[component]
pub fn ContextMenuTrigger(
    #[prop(optional, default = true)] pointer: bool,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <ContextMenuTriggerPrimitive
            class=class
            pointer={pointer}
            {..}
            data-slot="context-menu-trigger"
        >
            {children.map(|c| c())}
        </ContextMenuTriggerPrimitive>
    }
}

#[component]
pub fn ContextMenuContent(
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
    let base_class = "bg-popover text-popover-foreground data-[state=closed]:invisible data-[state=opening]:animate-in data-[state=closing]:animate-out data-[state=closing]:fade-out-0 data-[state=opening]:fade-in-0 data-[state=closing]:zoom-out-95 data-[state=opening]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-50 max-h-(--radix-context-menu-content-available-height) min-w-[12rem] origin-(--radix-context-menu-content-transform-origin) overflow-x-hidden overflow-y-auto rounded-md border p-1 shadow-md duration-150 ease-out-quad";
    let children = StoredValue::new(children);
    view! {
        <ContextMenuPortal>
            <ContextMenuContentPrimitive
                class=Signal::derive(move || tw_merge!(base_class, class.get()))
                side=side
                side_of_set=side_of_set
                align=align
                align_of_set=align_of_set
                limit_y=limit_y
                // ignore=ignore
                arrow=arrow
            >
                {children.get_value()()}
            </ContextMenuContentPrimitive>
        </ContextMenuPortal>
    }
}

#[component]
pub fn ContextMenuGroup(children: Children) -> impl IntoView {
    view! {
        <ContextMenuGroupPrimitive
        >
            {children()}
        </ContextMenuGroupPrimitive>
    }
}

#[component]
pub fn ContextMenuItem(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] close_on_click: bool,
    children: Children,
) -> impl IntoView {
    //NOTE: for now, i will use hover and not focus
    let base_class = "hover:bg-accent hover:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:hover:bg-destructive/10 dark:data-[variant=destructive]:hover:bg-destructive/20 data-[variant=destructive]:hover:text-destructive data-[variant=destructive]:*:[svg]:!text-destructive [&_svg:not([class*='text-'])]:text-muted-foreground relative flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden select-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 data-[inset]:pl-8 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4";
    view! {
        <ContextMenuItemPrimitive
            class=Signal::derive(move || tw_merge!(base_class, class.get()))
            close_on_click=close_on_click
        >
            {children()}
        </ContextMenuItemPrimitive>
    }
}

#[component]
pub fn ContextMenuLabel(
    children: Children,
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    view! {
        <ContextMenuGroupLabelPrimitive
            class=Signal::derive(move || tw_merge!(
                "text-sidebar-foreground/70 ring-sidebar-ring flex h-auto shrink-0 items-center rounded-md px-2 py-1 text-xs font-medium outline-hidden transition-[margin,opacity] duration-200 ease-linear focus-visible:ring-2 [&>svg]:size-4 [&>svg]:shrink-0",
                "group-data-[collapsible=icon]:-mt-8 group-data-[collapsible=icon]:opacity-0",
                class.get()
            ))
        >
            {children()}
        </ContextMenuGroupLabelPrimitive>
    }
}

#[component]
pub fn ContextMenuSeparator(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    let base_class = "bg-border -mx-1 my-1 h-px";
    view! {
        <ContextMenuSeparatorPrimitive
            class=Signal::derive(move || tw_merge!(base_class, class.get()))
        />
    }
}
