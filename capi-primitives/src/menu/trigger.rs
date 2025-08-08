use leptos::prelude::*;
use leptos_use::{UseElementBoundingReturn, use_element_bounding};

use crate::menu::MenuProviderContext;

#[component]
pub fn MenuTrigger(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let context = use_context::<MenuProviderContext>().expect("acces to menu context");
    let open = context.open;
    let hidden = context.hidden;
    let trigger_ref = context.trigger_ref;
    #[cfg(feature = "hydrate")]
    {
        let UseElementBoundingReturn {
            width,
            height,
            x,
            y,
            update,
            ..
        } = use_element_bounding(trigger_ref);
        Effect::new(move |_| {
            context.trigger_width.set(width.get());
            context.trigger_height.set(height.get());
            context.trigger_x.set(x.get());
            context.trigger_y.set(y.get());
        });

        Effect::new(move |_| {
            if context.open.get() {
                update()
            }
        });
    }
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
                hidden.set(false);
            }
            node_ref=trigger_ref
        >
            {children.map(|children| children())}
        </div>

    }
}
