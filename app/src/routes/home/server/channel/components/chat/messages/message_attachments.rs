use crate::components::copy::Copy;
use capi_ui::dialog::DialogPopup;
use capi_ui::dismissible::DismissibleOptions;
use capi_ui::toast::use_toast_store;
use capi_ui::toast::*;
use common::convex::{Attachment, FileType};
use icons::{IconDownLoad, IconExpand2, IconFile, IconMinimize2};
use leptos::prelude::*;
use uuid::Uuid;

use capi_ui::button::*;
use capi_ui::dialog::*;

#[component]
pub fn MessageAttachments(attachments: Vec<Attachment>) -> impl IntoView {
    let store = use_toast_store();
    let on_copy = Callback::new(move |_| {
        store.toasts().update(move |toasts| {
            toasts.push(ToastData {
                id: Uuid::new_v4().as_u128(),
                title: "".into(),
                _type: "".into(),
                description: "Copy image URL to clipboard.".into(),
                removed: false,
                timeout: 2500,
                height: 0.0,
            });
        });
    });
    view! {
        {
            attachments
                .into_iter()
                .filter_map(|att| {
                    let url = StoredValue::new(att.url.clone()?);
                    let name = StoredValue::new(att.name);
                    let data = att.metadata?;

                    Some(match data.content_type {
                        FileType::Jpeg | FileType::Png | FileType::Gif | FileType::Webp => {
                            let open = RwSignal::new(false);
                            view! {
                                <div class="w-fit h-fit relative group/attachment mb-2">
                                    <div class="absolute flex items-center justify-center top-1 right-1 p-1 gap-1.5 bg-popover rounded-md opacity-0 group-hover/attachment:opacity-100">
                                        <Button
                                            size=ButtonSizes::IconXs
                                            variant=ButtonVariants::Ghost
                                        >
                                            <IconDownLoad/>
                                        </Button>
                                        <Copy
                                            on_copy=on_copy
                                            size=ButtonSizes::IconXs
                                            variant=ButtonVariants::Ghost
                                            text=url.get_value()
                                        />
                                        <Dialog open=open dismiss_opts=DismissibleOptions {
                                            escape_key: true,
                                            outside_press: false
                                        }>
                                            <DialogTrigger>
                                                <Button
                                                    size=ButtonSizes::IconXs
                                                    variant=ButtonVariants::Ghost
                                                >
                                                    <IconExpand2/>
                                                </Button>
                                            </DialogTrigger>
                                            <DialogPortal>
                                                <DialogPopup
                                                    overlay=false
                                                    class="bg-background data-[state=closed]:invisible data-[state=opening]:animate-in data-[state=closing]:animate-out data-[state=closing]:fade-out-0 data-[state=opening]:fade-in-0 data-[state=closing]:zoom-out-95 data-[state=opening]:zoom-in-95 fixed top-[50%] left-[50%] z-60 grid max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] gap-4  border shadow-lg duration-200 ease-out-cubic sm:max-w-lg max-h-[715px] rounded-xl overflow-hidden p-0 w-[1150px] md:max-w-[700px] lg:max-w-[800px] xl:max-w-[1150px] h-auto"
                                                >
                                                    <img class="w-full max-h-full object-contain rounded-lg" src=url.get_value()/>
                                                </DialogPopup>
                                                <DialogOverlay class="bg-background">
                                                    <div class="absolute flex items-center justify-center top-2 right-2 p-1 gap-1 rounded-md">
                                                        <Button
                                                            size=ButtonSizes::Icon
                                                            variant=ButtonVariants::Ghost
                                                        >
                                                            <IconDownLoad/>
                                                        </Button>
                                                        <Copy
                                                            on_copy=on_copy
                                                            size=ButtonSizes::IconXs
                                                            variant=ButtonVariants::Ghost
                                                            text=url.get_value()
                                                        />
                                                        <Button
                                                            on:click=move |_| {
                                                                open.set(false);
                                                            }
                                                            size=ButtonSizes::Icon
                                                            variant=ButtonVariants::Ghost
                                                        >
                                                            <IconMinimize2/>
                                                        </Button>
                                                    </div>
                                                </DialogOverlay>
                                            </DialogPortal>
                                        </Dialog>
                                    </div>
                                    <img class="max-w-136 w-full h-auto rounded-lg" src=url.get_value()/>
                                </div>
                            }
                            .into_any()
                        }
                        _ => {
                            let display_size = if data.size > 0.0 {
                                let size_kb = data.size / 1024.0;
                                let size_mb = size_kb / 1024.0;
                                if size_mb >= 1.0 {
                                    format!("{size_mb:.2} MB")
                                } else if size_kb >= 1.0 {
                                    format!("{size_kb:.2} KB")
                                } else {
                                    format!("{:.0} bytes", data.size)
                                }
                            } else {
                                String::new()
                            };

                            view! {
                                <div
                                    class="flex group items-center p-4 w-fit gap-4 justify-center rounded-lg border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 cursor-pointer mb-2"
                                >
                                    <IconFile class="size-4 text-muted-foreground"/>
                                    <div class="flex flex-col items-start">
                                        <span>{name.get_value().unwrap_or(data.content_type.to_string())}</span>
                                        {
                                            if !display_size.is_empty() {
                                                view! { <span class="text-xs text-muted-foreground">{display_size}</span> }
                                                    .into_any()
                                            } else {
                                                ().into_any()
                                            }
                                        }
                                    </div>
                                    <Button
                                        size=ButtonSizes::Icon
                                        variant=ButtonVariants::Outline
                                        class="opacity-0 group-hover:opacity-100"
                                    >
                                        <IconDownLoad/>
                                    </Button>
                                </div>
                            }
                            .into_any()
                        }
                    })
                })
                .collect_view()
        }
    }
}
