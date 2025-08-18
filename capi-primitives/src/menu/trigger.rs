use leptos::prelude::*;

use crate::common::dismissible::use_dismiss;
use crate::common::floating::{ClickHandlers, use_click};
use crate::menu::MenuProviderContext;

#[component]
pub fn MenuTrigger(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let context = use_context::<MenuProviderContext>().expect("acces to menu context");
    let open = context.open;
    let ClickHandlers { on_click } = use_click(&context.floating);
    let trigger_ref = context.trigger_ref;
    use_dismiss(&context.floating, context.dismissible);
    view! {
        <div
            class=move || {
                format!(
                    "{} {}",
                    class,
                    match open.get() {
                        true => "pointer-events-none",
                        false => "",
                    },
                )
            }
            on:click=move |evt| {
                on_click.run(evt);
            }
            node_ref=trigger_ref
        >
            {children.map(|children| children())}
        </div>

    }
}
