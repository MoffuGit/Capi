use common::convex::Channel;
use leptos::prelude::*;

use crate::components::icons::{IconCirclePlus, IconSticker};
use crate::components::ui::button::{Button, ButtonSizes, ButtonVariants};

#[component]
pub fn Content(channel: RwSignal<Option<Channel>>) -> impl IntoView {
    view! {
        <div class="flex h-full w-full flex-col">
            <div class="flex min-h-0 flex-1 flex-col gap-2 overflow-auto">
            </div>
            <div class="flex flex-col gap-2 p-5">
                <div class="border-input dark:bg-input/30 flex w-full rounded-md border bg-transparent px-3 py-2 text-base shadow-xs md:text-sm justify-between">
                    <div class="flex items-center justify-center">
                        <Button size=ButtonSizes::Icon variant=ButtonVariants::Ghost>
                            <IconCirclePlus/>
                        </Button>
                    </div>
                    <div>
                    </div>
                    <div class="flex items-center">
                        <Button size=ButtonSizes::Icon variant=ButtonVariants::Ghost>
                            <IconSticker/>
                        </Button>
                    </div>
                </div>
            </div>
        </div>
    }
}
