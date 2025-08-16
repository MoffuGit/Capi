use leptos::prelude::*;

use crate::menu::MenuProviderContext;

#[component]
pub fn MenuTrigger(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let context = use_context::<MenuProviderContext>().expect("acces to menu context");
    let open = context.open;
    let trigger_ref = context.trigger_ref;
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
            on:click=move |_| {
                open.set(true);
            }
            node_ref=trigger_ref
        >
            {children.map(|children| children())}
        </div>

    }
}
