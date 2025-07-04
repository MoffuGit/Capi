pub use super::menu::GroupLabel as ContextMenuGroupLabel;
pub use super::menu::MenuBackDrop as ContextMenuBackDrop;
pub use super::menu::MenuContent as ContextMenuContent;
pub use super::menu::MenuGroup as ContextMenuGroup;
pub use super::menu::MenuItem as ContextMenuItem;
pub use super::menu::MenuPortal as ContextPortal;
pub use super::menu::MenuProvider as ContextProvider;
pub use super::menu::MenuSeparator as ContextSeparator;

use leptos::prelude::*;
// use leptos_use::use_element_bounding;
use leptos_use::use_mouse;
// use leptos_use::UseElementBoundingReturn;
use leptos_use::UseMouseReturn;
use tailwind_fuse::tw_merge;

use crate::components::primitives::menu::MenuProviderContext;

#[component]
pub fn ContextMenuTrigger(
    #[prop(optional)] class: &'static str,
    #[prop(optional, default = true)] pointer: bool,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let context = use_context::<MenuProviderContext>().expect("acces to menu context");
    let open = context.open;
    let hidden = context.hidden;
    let trigger_ref = context.trigger_ref;
    #[cfg(feature = "hydrate")]
    {
        let UseMouseReturn { x, y, .. } = use_mouse();
        Effect::new(move |_| {
            if context.open.get() {
                if !pointer {
                    use leptos_use::{use_element_bounding, UseElementBoundingReturn};

                    let UseElementBoundingReturn {
                        width,
                        height,
                        x,
                        y,
                        ..
                    } = use_element_bounding(trigger_ref);
                    context.trigger_width.set(width.get_untracked());
                    context.trigger_height.set(height.get_untracked());
                    context.trigger_x.set(x.get_untracked());
                    context.trigger_y.set(y.get_untracked());
                } else {
                    context.trigger_width.set(0.0);
                    context.trigger_height.set(0.0);
                    context.trigger_x.set(x.get_untracked());
                    context.trigger_y.set(y.get_untracked());
                }
            }
        });
    }
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
                hidden.set(false);
            }
            node_ref=trigger_ref
        >
            {children.map(|children| children())}
        </div>

    }
}
