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
use leptos_use::UseMouseReturn;
use leptos_use::use_mouse;
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
    let trigger_ref = context.trigger_ref;
    let UseMouseReturn { x, y, .. } = use_mouse();
    Effect::new(move |_| {
        if context.open.get() && pointer {
            set_trigger_rect.set(Some(TriggerBoundingRect {
                x: x.get_untracked(),
                y: y.get_untracked(),
                width: 0.0,
                height: 0.0,
            }));
        }
    });
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
                open.set(true);
            }
            node_ref=trigger_ref
        >
            {children.map(|children| children())}
        </div>

    }
}
