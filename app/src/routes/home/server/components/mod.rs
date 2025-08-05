use leptos::prelude::*;

use crate::components::icons::IconSearch;
use crate::components::ui::button::*;
use crate::components::ui::divider::Separator;
use crate::components::ui::sheet::*;
use crate::components::ui::sidebar::SidebarTrigger;
use capi_primitives::common::{Orientation, Side};

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="bg-background sticky top-0 flex shrink-0 items-center gap-2 p-3 border-b">
            <SidebarTrigger class="-ml-1" />
            <Separator
                orientation=Orientation::Vertical
                class="mr-2 data-[orientation=vertical]:h-4"
            />
            <div class="ml-auto mr-0 space-x-1">
                <Sheet>
                    <SheetTrigger as_child=true >
                        <Button
                            variant=ButtonVariants::Ghost
                            size=ButtonSizes::Icon
                            class="size-7"
                        >
                            <IconSearch class="size-4"/>
                        </Button>
                    </SheetTrigger>
                    <SheetPopup side=Side::Right>
                        <div>
                        </div>
                    </SheetPopup>
                </Sheet>
            </div>
        </header>

    }
}
