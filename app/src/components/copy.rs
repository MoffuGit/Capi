use capi_primitives::common::status::{use_transition_status, TransitionStatus};
use icons::{IconCheck, IconCopy};
use leptos::leptos_dom::helpers::set_timeout;
use leptos::task::spawn_local;
use leptos::{logging::log, prelude::*};
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

use crate::components::ui::button::{Button, ButtonSizes, ButtonVariants};

#[component]
pub fn Copy(
    #[prop(into)] text: Signal<String>,
    #[prop(optional, into)] variant: Signal<ButtonVariants>,
    #[prop(optional, into)] size: Signal<ButtonSizes>,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    let (copied, set_copied) = signal(false);
    let button_ref = NodeRef::new();

    let transition_status_state = use_transition_status(copied.into(), button_ref);

    let on_click_handler = move |_| {
        let text_to_copy = text.get();
        spawn_local(async move {
            if let Some(clipboard) = window().map(|win| win.navigator().clipboard()) {
                match JsFuture::from(clipboard.write_text(&text_to_copy)).await {
                    Ok(_) => {
                        set_copied.set(true);
                        set_timeout(
                            move || {
                                set_copied.set(false);
                            },
                            std::time::Duration::from_millis(1500),
                        );
                    }
                    Err(e) => {
                        log!("Failed to copy: {:?}", e);
                    }
                }
            } else {
                log!("Clipboard API not available.");
            }
        });
    };

    view! {
        <div class="hidden duration-150" node_ref=button_ref/>
        <Button
            variant=variant
            size=size
            class="relative overflow-hidden disabled:opacity-100"
            disabled=Signal::derive(move || copied() | disabled())
            on:mouseup=on_click_handler
        >
            <IconCheck
                class=Signal::derive(
                    move || {
                        let status = transition_status_state.transition_status.get();
                        let is_visible = matches!(status, TransitionStatus::Opening | TransitionStatus::Open);
                        format!(
                            "absolute left-[50%] top-[50%] origin-center -translate-x-1/2 -translate-y-1/2 transition-all duration-150 ease-out size-4 {}",
                            if is_visible {
                                "opacity-100 scale-100"
                            } else {
                                "opacity-0 scale-50 blur-xs"
                            }
                        )
                    }
                )
            />
            <IconCopy
                class=Signal::derive(move || {
                    let status = transition_status_state.transition_status.get();
                    let is_visible = matches!(status, TransitionStatus::Closing | TransitionStatus::Closed);
                    format!(
                        "absolute left-[50%] top-[50%] origin-center -translate-x-1/2 -translate-y-1/2 transition-all duration-150 ease-out size-4 {}",
                        if is_visible {
                            "opacity-100 scale-100"
                        } else {
                            "opacity-0 scale-50 blur-xs"
                        }
                    )
                })
            />
        </Button>
    }
}
