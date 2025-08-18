use crate::common::dismissible::use_dismiss;
use crate::common::floating::TriggerBoundingRect;
use crate::menu::MenuProviderContext;

pub use super::menu::GroupLabel as ContextMenuGroupLabel;
pub use super::menu::MenuBackDrop as ContextMenuBackDrop;
pub use super::menu::MenuContent as ContextMenuContent;
pub use super::menu::MenuGroup as ContextMenuGroup;
pub use super::menu::MenuItem as ContextMenuItem;
pub use super::menu::MenuPortal as ContextPortal;
pub use super::menu::MenuProvider as ContextProvider;
pub use super::menu::MenuSeparator as ContextSeparator;
pub use super::menu::SubMenuContent as ContextSubMenuContent;
pub use super::menu::SubMenuPortal as ContextSubMenuPortal;
pub use super::menu::SubMenuProvider as ContextSubMenuProvider;
pub use super::menu::SubMenuTrigger as ContextSubMenuTrigger;

use leptos::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn ContextMenuTrigger(
    #[prop(optional)] class: &'static str,
    #[prop(optional, default = true)] pointer: bool,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let context = use_context::<MenuProviderContext>().expect("acces to menu context");
    let set_trigger_rect = context.floating.position_ref;
    let open = context.open;
    use_dismiss(&context.floating, context.dismissible);
    let trigger_ref = context.trigger_ref;
    view! {
        <div
            class=move || {
                tw_merge!(
                    match open.get() {
                        true => "pointer-events-none",
                        false => "",
                    },
                    class,
                )
            }
            on:contextmenu=move |evt| {
                evt.prevent_default();
                if pointer {
                set_trigger_rect.set(Some(TriggerBoundingRect {
                    x: evt.client_x().into(),
                    y: evt.client_y().into(),
                    width: 0.0,
                    height: 0.0,
                }));
                }
                open.set(true);
            }
            node_ref=trigger_ref
        >
            {children.map(|children| children())}
        </div>

    }
}
