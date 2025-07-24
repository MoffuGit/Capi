use leptos::html::Span;
use leptos::prelude::*;
use leptos_dom::log;
use tailwind_fuse::tw_merge;
use web_sys::HtmlDivElement;

use crate::components::primitives::tabs::{use_tabs_context, TabsContext};

pub fn get_indicator_position(trigger: HtmlDivElement) -> (String, String, String) {
    let trigger_x = trigger.offset_left() as f64;
    let trigger_y = trigger.offset_top() as f64;
    let trigger_height = trigger.offset_height() as f64;
    let trigger_width = trigger.offset_width() as f64;

    // Calculate the transform string using translateX and translateY
    // Assuming the indicator's parent is the offsetParent for the triggers,
    // trigger_x and trigger_y are directly the desired translation values.
    let transform_style = format!("translateX({trigger_x}px) translateY({trigger_y}px)");

    (
        transform_style,
        format!("{trigger_width}px"),  // Set width to trigger's width
        format!("{trigger_height}px"), // Set height to trigger's height
    )
}

#[component]
pub fn TabIndicator(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let TabsContext {
        orientation,
        selected_tab,
        tabs,
        ..
    } = use_tabs_context();

    let position = RwSignal::new(None);

    let selected_ref =
        Signal::derive(move || tabs.with(|tabs| tabs.get(&selected_tab.get()).cloned()));

    Effect::new(move |_| {
        if let Some(tab) = selected_ref.get() {
            if let Some(trigger) = (tab.get()) {
                position.set(Some(get_indicator_position(trigger)));
            }
        }
    });

    let children = StoredValue::new(children);

    view! {
        {
            move || {
                position.get().map(|position| {
                let position = StoredValue::new(position);
                    view!{
                        <span
                            data-orientation=orientation.to_string()
                            class=move || tw_merge!("absolute top-0 left-0", class.get())
                            style:transform=position.get_value().0
                            style:width= position.get_value().1
                            style:height= position.get_value().2
                        >
                            {children.get_value()()}
                        </span>
                    }
                })
            }
        }
    }
}
