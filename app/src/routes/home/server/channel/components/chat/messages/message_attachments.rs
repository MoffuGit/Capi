use common::convex::FileType;
use leptos::prelude::*;

#[component]
pub fn MessageAttachments(attachments: Vec<common::convex::Attachment>) -> impl IntoView {
    view! {
        {
            attachments.into_iter().map(|att| {
                att.metadata.map(|data| {
                    match data.content_type {
                        FileType::Jpeg | FileType::Png => {
                            view!{
                                <img class="max-w-136 w-full h-auto flex rounded-lg mb-2" src=att.url.clone()/>
                            }.into_any()
                        },
                        _ => ().into_any()
                    }
                })
            }).collect_view()
        }
    }
}
