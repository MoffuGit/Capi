// use leptos::prelude::*;
//
// use crate::components::primitives::tooltip::ToolTipArrow as ToolPrimitiveArrow;
// use crate::components::primitives::tooltip::ToolTipContent as ToolContentPrimitive;
// use crate::components::primitives::tooltip::ToolTipPortal as ToolPortalPrimitive;
// use crate::components::primitives::tooltip::ToolTipProvider as ToolProviderPrimitive;
// use crate::components::primitives::tooltip::ToolTipRoot as ToolRootPrimitive;
// use crate::components::primitives::tooltip::ToolTipTrigger as ToolTriggerPrimitive;
//
// #[component]
// pub fn ToolTipProvider(children: Children) -> impl IntoView {
//     view! {
//         <ToolProviderPrimitive
//             {..}
//             data-slot="tooltip-provider"
//         >
//             {children()}
//         </ToolProviderPrimitive>
//     }
// }
//
// #[component]
// pub fn ToolTip(children: ChildrenFn) -> impl IntoView {
//     view! {
//         <ToolTipProvider>
//             <ToolRootPrimitive>
//                 {children()}
//             </ToolRootPrimitive>
//         </ToolTipProvider>
//     }
// }
//
// #[component]
// pub fn ToolTipTrigger(children: ChildrenFn) -> impl IntoView {
//     view! {
//         <ToolTriggerPrimitive
//             // {..}
//             // data-slot="tooltip-trigger"
//         >
//             {children()}
//         </ToolTriggerPrimitive>
//     }
// }
//
// #[component]
// pub fn ToolTipContent(children: ChildrenFn) -> impl IntoView {
//     let children = StoredValue::new(children);
//     view! {
//         <ToolPortalPrimitive>
//             <ToolContentPrimitive class= "bg-primary text-primary-foreground animate-in fade-in-0 zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-50 w-fit origin-(--radix-tooltip-content-transform-origin) rounded-md px-3 py-1.5 text-xs text-balance">
//                 {children.get_value()()}
//                 <ToolPrimitiveArrow class="bg-primary fill-primary z-50 size-2.5 translate-y-[calc(-50%_-_2px)] rotate-45 rounded-[2px]" />
//             </ToolContentPrimitive>
//         </ToolPortalPrimitive>
//     }
// }
//
// // function TooltipContent({
// //   className,
// //   sideOffset = 0,
// //   children,
// //   ...props
// // }: React.ComponentProps<typeof TooltipPrimitive.Content>) {
// //   return (
// //     <TooltipPrimitive.Portal>
// //       <TooltipPrimitive.Content
// //         data-slot="tooltip-content"
// //         sideOffset={sideOffset}
// //         className={cn(
// //           "",
// //           className
// //         )}
// //         {...props}
// //       >
// //         {children}
// //         <TooltipPrimitive.Arrow className="bg-primary fill-primary z-50 size-2.5 translate-y-[calc(-50%_-_2px)] rotate-45 rounded-[2px]" />
// //       </TooltipPrimitive.Content>
// //     </TooltipPrimitive.Portal>
// //   )
// // }
