use crate::components::primitives::dialog::DialogOverlay as DialogOverlayPrimitive;
use crate::components::primitives::dialog::DialogPopup as DialogPopupPrimitive;
use crate::components::primitives::dialog::DialogPortal as DialogPortalPrimitive;
use crate::components::primitives::dialog::DialogRoot as DialogPrimitive;
use crate::components::primitives::dialog::DialogTrigger as DialogTriggerPrimitive;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;

#[component]
pub fn Dialog(
    #[prop(into, default = RwSignal::new(false))] open: RwSignal<bool>,
    #[prop(default = true)] modal: bool,
    #[prop(default = true)] dismissible: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <DialogPrimitive open=open modal=modal dismissible=dismissible>
            {children()}
        </DialogPrimitive>
    }
}

#[component]
pub fn DialogTrigger(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    view! {
        <DialogTriggerPrimitive as_child=as_child node_ref=node_ref>
            {children.clone().map(|children| children())}
        </DialogTriggerPrimitive>
    }
}

#[component]
pub fn DialogPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <DialogPortalPrimitive container=container container_ref=container_ref as_child=as_child node_ref=node_ref children=children/>
    }
}

const DIALOG_POPUP: &str = "bg-background data-[state=undefined]:opacity-0 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 fixed top-[50%] left-[50%] z-50 grid w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] gap-4 rounded-lg border p-6 shadow-lg duration-200 sm:max-w-lg";

#[component]
pub fn DialogPopup(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <DialogPortal>
            <DialogOverlay/>
            <DialogPopupPrimitive as_child=as_child node_ref=node_ref class=Signal::derive(move || format!("{} {}",DIALOG_POPUP, class.get()))>
                {children.get_value().map(|children| children())}
            </DialogPopupPrimitive>
        </DialogPortal>
    }
}

const DIALOG_OVERLAY: &str = "data-[state=open]:animate-in data-[state=undefined]:opacity-0 data-[modal=true]:cursor-pointer-none data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 fixed inset-0 z-50 bg-black/50 duration-200";

#[component]
pub fn DialogOverlay(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <DialogOverlayPrimitive as_child=as_child node_ref=node_ref class=Signal::derive(move || format!("{} {}", DIALOG_OVERLAY, class.get()))>
            {children.get_value().map(|children| children())}
        </DialogOverlayPrimitive>
    }
}

// function DialogContent({
//   className,
//   children,
//   showCloseButton = true,
//   ...props
// }: React.ComponentProps<typeof DialogPrimitive.Content> & {
//   showCloseButton?: boolean
// }) {
//   return (
//     <DialogPortal data-slot="dialog-portal">
//       <DialogOverlay />
//       <DialogPrimitive.Content
//         data-slot="dialog-content"
//         className={cn(
//           "",
//           className
//         )}
//         {...props}
//       >
//         {children}
//         {showCloseButton && (
//           <DialogPrimitive.Close
//             data-slot="dialog-close"
//             className="ring-offset-background focus:ring-ring data-[state=open]:bg-accent data-[state=open]:text-muted-foreground absolute top-4 right-4 rounded-xs opacity-70 transition-opacity hover:opacity-100 focus:ring-2 focus:ring-offset-2 focus:outline-hidden disabled:pointer-events-none [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4"
//           >
//             <XIcon />
//             <span className="sr-only">Close</span>
//           </DialogPrimitive.Close>
//         )}
//       </DialogPrimitive.Content>
//     </DialogPortal>
//   )
// }

// function DialogClose({
//   ...props
// }: React.ComponentProps<typeof DialogPrimitive.Close>) {
//   return <DialogPrimitive.Close data-slot="dialog-close" {...props} />
// }
//
//
//
// function DialogHeader({ className, ...props }: React.ComponentProps<"div">) {
//   return (
//     <div
//       data-slot="dialog-header"
//       className={cn("flex flex-col gap-2 text-center sm:text-left", className)}
//       {...props}
//     />
//   )
// }
//
// function DialogFooter({ className, ...props }: React.ComponentProps<"div">) {
//   return (
//     <div
//       data-slot="dialog-footer"
//       className={cn(
//         "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end",
//         className
//       )}
//       {...props}
//     />
//   )
// }
//
// function DialogTitle({
//   className,
//   ...props
// }: React.ComponentProps<typeof DialogPrimitive.Title>) {
//   return (
//     <DialogPrimitive.Title
//       data-slot="dialog-title"
//       className={cn("text-lg leading-none font-semibold", className)}
//       {...props}
//     />
//   )
// }
//
// function DialogDescription({
//   className,
//   ...props
// }: React.ComponentProps<typeof DialogPrimitive.Description>) {
//   return (
//     <DialogPrimitive.Description
//       data-slot="dialog-description"
//       className={cn("text-muted-foreground text-sm", className)}
//       {...props}
//     />
//   )
// }
//
// export {
//   Dialog,
//   DialogClose,
//   DialogContent,
//   DialogDescription,
//   DialogFooter,
//   DialogHeader,
//   DialogOverlay,
//   DialogPortal,
//   DialogTitle,
//   DialogTrigger,
// }
//
