use common::convex::ChannelMessage;
use convex_client::leptos::Mutation;
use convex_client::leptos::UseMutation;
use icons::IconPin;
use icons::IconPinOff;
use leptos::either::Either;
use leptos::prelude::*;
use serde::Serialize;
use web_sys::MouseEvent;

use crate::components::auth::use_auth;
use crate::components::roles::CanPinMessages;
use capi_ui::button::*;

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct PinMessage {
    auth: i64,
    #[serde(rename = "messageId")]
    message: String,
    #[serde(rename = "channelId")]
    channel: String,
}

impl Mutation for PinMessage {
    type Output = ();

    fn name(&self) -> String {
        "messages:pinMessage".into()
    }
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct UnpinMessage {
    auth: i64,
    #[serde(rename = "messageId")]
    message: String,
    #[serde(rename = "channelId")]
    channel: String,
}

impl Mutation for UnpinMessage {
    type Output = ();

    fn name(&self) -> String {
        "messages:unpinMessage".into()
    }
}

#[component]
pub fn PinMessageButton(msg: StoredValue<ChannelMessage>) -> impl IntoView {
    let auth = use_auth().auth;
    let pin_message_mutation = UseMutation::new::<PinMessage>();
    let unpin_message_mutation = UseMutation::new::<UnpinMessage>();

    let is_pinned = msg.get_value().pinned;

    let is_pending =
        move || pin_message_mutation.pending().get() || unpin_message_mutation.pending().get();

    view! {
        <CanPinMessages>
            <Button
                variant=ButtonVariants::Ghost
                size=ButtonSizes::IconXs
                disabled=Signal::derive(is_pending)
                on:click=move |evt: MouseEvent| {
                    evt.prevent_default();
                    if let Some(auth) = auth.get().and_then(|auth| auth.ok()).flatten() {
                        if is_pinned {
                            unpin_message_mutation.dispatch(UnpinMessage {
                                auth: auth.id,
                                message: msg.get_value().id,
                                channel: msg.get_value().channel,
                            });
                        } else {
                            pin_message_mutation.dispatch(PinMessage {
                                auth: auth.id,
                                message: msg.get_value().id,
                                channel: msg.get_value().channel,
                            });
                        }
                    }
                }
            >
                {
                    if is_pinned {
                        Either::Left(view!{<IconPinOff/>})
                    } else {
                         Either::Right(view!{<IconPin />})
                    }
                }
            </Button>
        </CanPinMessages>
    }
}
