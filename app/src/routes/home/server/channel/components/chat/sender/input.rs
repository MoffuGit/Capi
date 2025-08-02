use leptos::html::Div;
use leptos::prelude::*;

#[component]
pub fn MessageInputArea(
    message: RwSignal<String>,
    content_ref: NodeRef<Div>,
    #[prop(into)] channel_name: Signal<Option<String>>,
) -> impl IntoView {
    let on_input = move |_| {
        if let Some(div) = content_ref.get() {
            message.set(div.inner_text());
        }
    };

    view! {
        <div class="relative self-center h-fit w-full overflow-y-auto overflow-x-hidden ">
            <div class="text-sm font-normal relative">
                <div>
                    <Show when=move || message.get().is_empty()>
                        <div class="absolute left-0 select-none text-muted-foreground">
                            {
                                move || {
                                    channel_name.get().map(|channel| format!("Send a message to {channel}"))
                                }
                            }
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
    }
}
