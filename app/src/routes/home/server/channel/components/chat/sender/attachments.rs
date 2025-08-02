use leptos::prelude::*;

use crate::components::icons::IconTrash;
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
                class="w-full p-1 peer duration-200 first:rounded-t-lg bg-background"
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
        size: _,
        content_type,
        ..
    } = attachment;
    view! {
        <div class="relative size-20 bg-muted h-full rounded-lg flex flex-col items-center justify-around isolate group/attachment">
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
            <div class="w-full text-start max-h-4 text-xs text-nowrap truncate inline-block">
                {format!("{}-{}", name, content_type)}
            </div>
        </div>

    }
}
