use leptos::prelude::*;

use super::PrivateMessageDetails;

#[component]
pub fn Messages(
    messages: ReadSignal<Option<Result<Vec<PrivateMessageDetails>, String>>>,
) -> impl IntoView {
    view! {
        <div class="flex min-h-0 flex-1 flex-col overflow-auto pt-4 scrollbar-thin scrollbar-track-background pb-[var(--sender-height)]">
            {
                move || {
                    messages.get().and_then(|res| res.ok()).unwrap_or_default().into_iter().map(|message| {
                        view! {
                            <div class="flex items-start relative">
                                <div class="flex flex-col text-sm font-light w-full">
                                    <div
                                        class="w-full h-auto transition-colors ease-in-out-quad duration-180 gap-2 py-1
                                            data-[response=true]:bg-purple/10 data-[response=true]:border-l-purple
                                            data-[highlight=true]:bg-purple/10 data-[highlight=true]:border-l-purple
                                            border-l border-l-transparent data-[context=true]:bg-accent/50 hover:bg-accent/50 px-8 group min-h-9 flex flex-col justify-center relative"
                                    >
                                        {message.content}
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect_view()
                }
            }
        </div>
    }
}
