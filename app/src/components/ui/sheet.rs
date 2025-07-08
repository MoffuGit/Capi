use crate::components::primitives::common::Side;
use crate::components::primitives::dialog::DialogOverlay as SheetOverlayPrimitive;
use crate::components::primitives::dialog::DialogPopup as SheetPopupPrimitive;
use crate::components::primitives::dialog::DialogPortal as SheetPortalPrimitive;
use crate::components::primitives::dialog::DialogRoot as SheetPrimitive;
use crate::components::primitives::dialog::DialogTrigger as SheetTriggerPrimitive;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;

#[component]
pub fn Sheet(
    #[prop(into, default = RwSignal::new(false))] open: RwSignal<bool>,
    #[prop(default = true)] modal: bool,
    #[prop(default = true)] dismissible: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <SheetPrimitive open=open modal=modal dismissible=dismissible>
            {children()}
        </SheetPrimitive>
    }
}

#[component]
pub fn SheetTrigger(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    view! {
        <SheetTriggerPrimitive
            as_child=as_child node_ref={node_ref}
            {..} class=move || class.get()>
            {children.clone().map(|children| children())}
        </SheetTriggerPrimitive>
    }
}

#[component]
pub fn SheetPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(default = 200)] open_duration: u64,
    #[prop(default = 200)] close_duration: u64,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <SheetPortalPrimitive container=container container_ref=container_ref as_child=as_child node_ref=node_ref children=children open_duration=open_duration close_duration=close_duration/>
    }
}

const SHEET_OVERLAY: &str = "data-[state=open]:animate-in data-[state=undefined]:opacity-0 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 fixed inset-0 z-50 bg-black/50 data-[state=closed]:duration-300 data-[state=open]:duration-500";

#[component]
pub fn SheetOverlay(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <SheetOverlayPrimitive as_child=as_child node_ref=node_ref class=Signal::derive(move || format!("{} {}", SHEET_OVERLAY, class.get()))>
            {children.get_value().map(|children| children())}
        </SheetOverlayPrimitive>
    }
}

const SHEET_POPUP: &str = "bg-background data-[state=open]:animate-in data-[state=closed]:animate-out fixed z-50 flex flex-col gap-4 shadow-lg transition ease-in-out data-[state=closed]:duration-300 data-[state=open]:duration-500";

#[component]
pub fn SheetPopup(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(default = Side::Right)] side: Side,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let sheet_popup = match side {
        Side::Top => "data-[state=closed]:slide-out-to-top data-[state=open]:slide-in-from-top inset-x-0 top-0 h-auto border-b",
        Side::Bottom => "data-[state=closed]:slide-out-to-bottom data-[state=open]:slide-in-from-bottom inset-x-0 bottom-0 h-auto border-t",
        Side::Left => "data-[state=closed]:slide-out-to-left data-[state=open]:slide-in-from-left inset-y-0 left-0 h-full w-3/4 border-r sm:max-w-sm",
        Side::Right =>  "data-[state=closed]:slide-out-to-right data-[state=open]:slide-in-from-right inset-y-0 right-0 h-full w-3/4 border-l sm:max-w-sm",
    };
    view! {
        <SheetPortal open_duration=500 close_duration=300>
            <SheetOverlay/>
            <SheetPopupPrimitive as_child=as_child node_ref=node_ref class=Signal::derive(move || format!("{} {} {}",SHEET_POPUP, sheet_popup, class.get()))>
                {children.get_value().map(|children| children())}
            </SheetPopupPrimitive>
        </SheetPortal>
    }
}
// function SheetClose({
//   ...props
// }: React.ComponentProps<typeof SheetPrimitive.Close>) {
//   return <SheetPrimitive.Close data-slot="sheet-close" {...props} />
// }
// }

//         <SheetPrimitive.Close className="ring-offset-background focus:ring-ring data-[state=open]:bg-secondary absolute top-4 right-4 rounded-xs opacity-70 transition-opacity hover:opacity-100 focus:ring-2 focus:ring-offset-2 focus:outline-hidden disabled:pointer-events-none">
//           <XIcon className="size-4" />
//           <span className="sr-only">Close</span>
//         </SheetPrimitive.Close>
// function SheetHeader({ className, ...props }: React.ComponentProps<"div">) {
//   return (
//     <div
//       data-slot="sheet-header"
//       className={cn("flex flex-col gap-1.5 p-4", className)}
//       {...props}
//     />
//   )
// }
//
// function SheetFooter({ className, ...props }: React.ComponentProps<"div">) {
//   return (
//     <div
//       data-slot="sheet-footer"
//       className={cn("mt-auto flex flex-col gap-2 p-4", className)}
//       {...props}
//     />
//   )
// }
//
// function SheetTitle({
//   className,
//   ...props
// }: React.ComponentProps<typeof SheetPrimitive.Title>) {
//   return (
//     <SheetPrimitive.Title
//       data-slot="sheet-title"
//       className={cn("text-foreground font-semibold", className)}
//       {...props}
//     />
//   )
// }
//
// function SheetDescription({
//   className,
//   ...props
// }: React.ComponentProps<typeof SheetPrimitive.Description>) {
//   return (
//     <SheetPrimitive.Description
//       data-slot="sheet-description"
//       className={cn("text-muted-foreground text-sm", className)}
//       {...props}
//     />
//   )
// }
