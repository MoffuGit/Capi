use common::convex::FileType;
use leptos::prelude::*;

use crate::components::icons::IconFile;
use crate::components::icons::IconFileArchive;
use crate::components::icons::IconFileAudio;
use crate::components::icons::IconTrash;
use crate::components::ui::avatar::*;
use crate::components::ui::button::*;
use crate::components::ui::collapsible::*;
use crate::routes::server::channel::components::chat::ClientFileMetaData;

#[component]
pub fn AttachmentPreviewList(attachments: RwSignal<Vec<ClientFileMetaData>>) -> impl IntoView {
    let open = RwSignal::new(false);

    Effect::new(move |_| {
        open.set(!attachments.get().is_empty());
    });
    view! {
        <Collapsible open=open>
            <CollapsiblePanel
                class="w-full duration-200 bg-transparent"
            >
                <div class="h-20 gap-1 bg-background w-full flex items-center text-sm group">
                    {
                        move || {
                            attachments.get().iter().enumerate().map(|(idx, att)| {
                                view!{
                                    <Attachment attachment=att.clone() idx=idx attachments=attachments/>
                                }
                            }).collect_view()
                        }
                    }
                </div>
            </CollapsiblePanel>
        </Collapsible>
    }
}

#[component]
pub fn Attachment(
    attachment: ClientFileMetaData,
    idx: usize,
    attachments: RwSignal<Vec<ClientFileMetaData>>,
) -> impl IntoView {
    let ClientFileMetaData {
        name,
        url,
        content_type,
        ..
    } = attachment;
    view! {
        <Avatar class="relative size-20 border-input shadow-xs rounded-md flex flex-col items-center justify-center isolate group/attachment">
            <Button
                size=ButtonSizes::Icon
                variant=ButtonVariants::Secondary
                on:click=move |_| {
                    attachments.update(|attachments| {
                        attachments.remove(idx);
                    });
                }
                class="size-6 hover:text-destructive opacity-0 group-hover/attachment:opacity-100 absolute top-1 right-1"
            >
                <IconTrash/>
            </Button>
            {
                match content_type {
                    FileType::Jpeg | FileType::Png | FileType::Gif | FileType::Webp => {
                        view! {
                            <AvatarImage url=url/>
                        }.into_any()
                    }
                    _ => {
                        view! {
                            <IconFile class="size-6 text-muted-foreground"/>
                        }.into_any()
                    }
                }
            }
            <AvatarFallback class="w-full text-center max-h-4 text-xs text-nowrap truncate inline-block px-1">
                {name.clone()}
            </AvatarFallback >
        </Avatar>
    }
}
