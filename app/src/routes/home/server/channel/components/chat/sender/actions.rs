use common::files::read_file;
use gloo_file::File;
use leptos::html::Input;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_dom::warn;
use wasm_bindgen::JsCast as _;
use web_sys::{Event, HtmlInputElement};

use crate::components::ui::button::*;
use crate::routes::server::channel::components::chat::ClientFile;
use icons::{IconPaperClip, IconSend, IconSmile};

#[component]
pub fn MessageActionButtons(
    on_send: Callback<()>,
    attachments: RwSignal<Vec<ClientFile>>,
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
                size=ButtonSizes::Icon variant=ButtonVariants::Ghost class="size-7 text-muted-foreground hover:text-foreground">
                <IconPaperClip/>
            </Button>
            <Button
                size=ButtonSizes::Icon variant=ButtonVariants::Ghost
                class="size-7 text-muted-foreground hover:text-foreground"
            >
                <IconSmile/>
            </Button>
            <Button size=ButtonSizes::Icon
                variant=ButtonVariants::Secondary
                class="size-7 text-muted-foreground hover:text-foreground"
                on:click=move |_| on_send.run(())
            >
                <IconSend/>
            </Button>
        </div>
    }
}
