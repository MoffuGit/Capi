use common::convex::ChannelMessage;
use leptos::prelude::*;

use crate::components::icons::IconX;
use crate::components::ui::button::*;
use crate::components::ui::collapsible::*;
use crate::routes::server::channel::components::chat::ChatContext;

#[component]
pub fn MsgRefDisplay(
    #[prop(into)] msg_ref: Signal<Option<ChannelMessage>>,
    on_clear_ref: Callback<()>,
) -> impl IntoView {
    let context: ChatContext = use_context().expect("should return teh chat context");
    let cached_members = context.cached_members;
    let cached_member = Memo::new(move |prev| {
        if let Some(msg) = msg_ref.get() {
            cached_members
                .get()
                .and_then(|members| members.get(&msg.sender).map(|member| member.name.clone()))
        } else {
            prev.flatten().cloned()
        }
    });
    let open = RwSignal::new(false);
    Effect::new(move |_| {
        open.set(msg_ref.get().is_some());
    });
    view! {
        <Collapsible open=open>
            <CollapsiblePanel class="w-full bg-transparent overflow-hidden">
                <div class="flex items-center justify-between h-auto">
                    <div class="flex text-xs text-base-content/70 truncate">
                        <div class="text-xs">
                            <span class="text-muted-foreground">
                                 "Replying to "
                            </span>
                            <span class="font-medium text-base-content">
                                {move || {
                                    cached_member.get()
                                }}
                            </span>
                        </div>
                    </div>
                    <Button size=ButtonSizes::Icon variant=ButtonVariants::Ghost class="size-6 text-muted-foreground" on:click=move |_| on_clear_ref.run(())>
                        <IconX />
                    </Button>
                </div>
            </CollapsiblePanel>
        </Collapsible>
    }
}
