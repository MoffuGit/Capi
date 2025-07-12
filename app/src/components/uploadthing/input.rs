use leptos::html::Input as HtmlInput;
use leptos::prelude::*;
use uploadthing::UploadthingFile;

use super::read_file;

#[component]
pub fn FileInput(
    files: RwSignal<Vec<UploadthingFile>>,
    #[prop(into)] class: Signal<String>,
) -> impl IntoView {
    let input_ref: NodeRef<HtmlInput> = NodeRef::new();

    let on_change = move |_| {
        let input = input_ref.get().expect("input ref to be valid");
        if let Some(file_list) = input.files() {
            for idx in 0..file_list.length() {
                if let Some(file) = file_list.get(idx) {
                    read_file(file.into(), move |file| {
                        if let Ok(file) = file {
                            files.update(|files| files.push(file));
                        }
                    });
                }
            }
        }
    };
    view! {
        <input
            type="file"
            class=class
            node_ref=input_ref
            on:change=on_change
            multiple=true
        />
    }
}
