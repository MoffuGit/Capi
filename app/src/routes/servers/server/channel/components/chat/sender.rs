use crate::components::icons::{IconCirclePlus, IconSend, IconSticker};
use crate::components::ui::button::{Button, ButtonSizes, ButtonVariants};
use common::convex::Channel;
use leptos::html::Div;
use leptos::prelude::*;

#[component]
pub fn Sender(channel: RwSignal<Option<Channel>>) -> impl IntoView {
    let message = RwSignal::new(String::default());
    let content_ref: NodeRef<Div> = NodeRef::new();
    let on_input = move |_| {
        if let Some(div) = content_ref.get() {
            message.set(div.inner_text());
        }
    };

    view! {
        <div class="flex flex-col gap-2 p-5">
            <div class="border-input dark:bg-input/30 flex w-full rounded-md border bg-transparent px-3 py-2 text-base shadow-xs md:text-sm justify-between">
                <div class="flex items-center justify-center">
                    <Button size=ButtonSizes::Icon variant=ButtonVariants::Ghost>
                        <IconCirclePlus/>
                    </Button>
                </div>
                <div class="relative self-center h-fit w-full" /* style=move || format!("height: {}px", height.get()) */>
                    <div class="text-sm font-normal relative mx-2">
                        <div>
                            <Show when=move || message.get().is_empty()>
                                <div class="absolute left-0 select-none text-base-content/40">
                                    {move || channel.get().map(|channel| format!("Message #{}", channel.name))}
                                </div>
                            </Show>
                        </div>
                        <div
                            on:input=on_input
                            node_ref=content_ref
                            class="relative outline-0 wrap-break-word text-left whitespace-break-spaces"
                            contenteditable="true"
                            aria-multiline="true"
                            spellcheck="true"
                            aria-invalid="false">
                        </div>
                    </div>
                </div>

                <div class="flex items-center">
                    <Button size=ButtonSizes::Icon variant=ButtonVariants::Ghost>
                        <IconSticker/>
                    </Button>
                    <Button size=ButtonSizes::Icon>
                        <IconSend/>
                    </Button>
                </div>
            </div>
        </div>

    }
}
