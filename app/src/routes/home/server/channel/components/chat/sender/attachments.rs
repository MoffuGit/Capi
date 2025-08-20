use common::convex::FileType;
use common::files::FileMetaData;
use leptos::prelude::*;

use crate::components::ui::avatar::*;
use crate::components::ui::button::*;
use crate::components::ui::collapsible::*;
use crate::routes::server::channel::components::chat::ClientFile;
use icons::IconFile;
use icons::IconTrash;

#[component]
pub fn AttachmentPreviewList(attachments: RwSignal<Vec<ClientFile>>) -> impl IntoView {
    let open = RwSignal::new(false);

    Effect::new(move |_| {
        open.set(!attachments.get().is_empty());
    });

    view! {
        <Collapsible open=open>
            <CollapsiblePanel
                class="w-full duration-200 bg-transparent"
            >
                <div class="h-20 gap-1 bg-background w-full flex items-center text-sm group transition-[filter] ease-out-quad duration-200 group-data-[state=open]:blur-none group-data-[state=closing]:blur-xs group-data-[state=closed]:blur-xs">
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
    attachment: ClientFile,
    idx: usize,
    attachments: RwSignal<Vec<ClientFile>>,
) -> impl IntoView {
    let ClientFile {
        metadata:
            FileMetaData {
                name,
                size,
                content_type,
                url,
            },
        ..
    } = attachment;
    view! {
        <Avatar class="relative size-20 border border-input shadow-md rounded-md flex flex-col items-center justify-center isolate group/attachment">
            <Button
                size=ButtonSizes::IconXs
                variant=ButtonVariants::Outline
                on:click=move |_| {
                    attachments.update(|attachments| {
                        attachments.remove(idx);
                    });
                }
                class="size-6 hover:text-destructive opacity-0 group-hover/attachment:opacity-100 absolute top-1 right-1 transition-none"
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
