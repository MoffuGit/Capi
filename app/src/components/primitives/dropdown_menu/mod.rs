pub use super::menu::GroupLabel as DropdownMenuGroupLabel;
pub use super::menu::MenuBackDrop as DropdownMenuBackDrop;
pub use super::menu::MenuContent as DropdownMenuContent;
pub use super::menu::MenuGroup as DropdownMenuGroup;
pub use super::menu::MenuItem as DropdownMenuItem;
pub use super::menu::MenuPortal as DropdownPortal;
pub use super::menu::MenuProvider as DropdownProvider;
pub use super::menu::MenuSeparator as DropdownSeparator;

use leptos::prelude::*;

use crate::components::primitives::menu::MenuProviderContext;

#[component]
pub fn DropdownMenuTrigger(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let context = use_context::<MenuProviderContext>().expect("acces to menu context");
    let open = context.open;
    let hidden = context.hidden;
    let trigger_ref = context.trigger_ref;
    view! {
        <div
            class=move || {
                format!(
                    "{} {}",
                    class,
                    match open.get() {
                        true => "pointer-events-auto",
                        false => "",
                    },
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
