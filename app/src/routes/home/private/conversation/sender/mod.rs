mod input;

use leptos::html::Div;
use leptos::prelude::*;

use self::input::MessageInputArea;

#[component]
pub fn Sender(sender_ref: NodeRef<Div>) -> impl IntoView {
    let content_ref = NodeRef::new();
    let msg = RwSignal::new(String::default());
    view! {
        <div class="w-full absolute bottom-0 bg-transparent flex flex-col z-20 isolate" node_ref=sender_ref>
            <div class="w-full px-5">
                <div class="p-1 border border-input rounded-lg backdrop-blur-xs bg-muted/30">
                    <div class="flex flex-col items-center justify-center shadow-xs bg-background text-base rounded-md gap-2 p-2">
                        <MessageInputArea content_ref=content_ref message=msg conversation_name="" />
                    </div>
                </div>
            </div>
            <div class="absolute inset-0 mb-5 bg-gradient-to-b -z-10 from-transparent to-background"/>
            <div class="bg-background w-full h-5"/>
        </div>
    }
}
