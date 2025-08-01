use std::str::FromStr;

use anyhow::anyhow;
use common::convex::FileType;
use gloo_file::futures::read_as_bytes;
use gloo_file::{Blob, File};
use leptos::html::Input;
use leptos::prelude::*;
use leptos::task::{spawn_local, spawn_local_scoped_with_cancellation};
use wasm_bindgen::JsCast as _;
use web_sys::{Event, HtmlInputElement, Url};

use crate::components::icons::{IconPaperClip, IconSend, IconSticker};
use crate::components::ui::button::*;
use crate::routes::server::channel::components::chat::ClientFileMetaData;

fn read_file<F>(file: File, callback: F)
where
    F: FnOnce(anyhow::Result<ClientFileMetaData>) + 'static,
{
    let name = file.name();
    let content_type = FileType::from_str(&file.raw_mime_type()).unwrap_or_default();
    let size = file.size() as usize;
    let file_blob: Blob = file.into(); // Convert gloo_file::File to gloo_file::Blob
    let file_blob_clone = file_blob.clone(); // Clone for creating object URL

    spawn_local_scoped_with_cancellation(async move {
        let result: Result<ClientFileMetaData, anyhow::Error> = async {
            let url = Url::create_object_url_with_blob(&file_blob_clone.into())
                .map_err(|e| anyhow!("Failed to create object URL: {:?}", e))?;

            let chunks = read_as_bytes(&file_blob)
                .await
                .map_err(|e| anyhow!("Failed to read file bytes: {:?}", e))?;

            Ok(ClientFileMetaData {
                name,
                size,
                content_type,
                chunks,
                url,
            })
        }
        .await;

        callback(result);
    });
}

#[component]
pub fn MessageActionButtons(
    on_send: Callback<()>,
    attachments: RwSignal<Vec<ClientFileMetaData>>,
) -> impl IntoView {
    let file_input_ref: NodeRef<Input> = NodeRef::new();

    let on_file_selected = move |event: Event| {
        let input = event.target().unwrap().unchecked_into::<HtmlInputElement>();
        if let Some(files) = input.files() {
            let num_files = files.length();

            spawn_local(async move {
                for i in 0..num_files {
                    if let Some(file) = files.get(i) {
                        read_file(file.into(), move |file| {
                            if let Ok(file) = file {
                                attachments.update(|current| {
                                    current.push(file);
                                });
                            }
                        });
                    }
                }
            });
            input.set_value("");
        }
    };
    view! {
        <input type="file" multiple=true class="hidden" node_ref=file_input_ref on:change=on_file_selected />
        <div class="flex items-center gap-3 p-1">
            <Button
                on:click=move |_| {
                    if let Some(input) = file_input_ref.get() {
                        input.click();
                    }
                }
                size=ButtonSizes::Icon variant=ButtonVariants::Ghost class="size-6 text-muted-foreground hover:text-foreground">
                <IconPaperClip/>
            </Button>
            <Button
                size=ButtonSizes::Icon variant=ButtonVariants::Ghost
                class="size-6 text-muted-foreground hover:text-foreground"
            >
                <IconSticker/>
            </Button>
            <Button size=ButtonSizes::Icon
                variant=ButtonVariants::Secondary
                class="size-6 text-muted-foreground hover:text-foreground"
                on:click=move |_| on_send.run(())
            >
                <IconSend/>
            </Button>
        </div>
    }
}
