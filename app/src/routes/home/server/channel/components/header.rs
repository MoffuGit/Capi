use common::convex::Channel;
use leptos::prelude::*;

use crate::components::icons::{IconSearch, IconUsers};
use crate::components::primitives::common::{Orientation, Side};
use crate::components::ui::breadcrumb::{
    Breadcrumb, BreadcrumbItem, BreadcrumbList, BreadcrumbPage,
};
use crate::components::ui::button::{Button, ButtonSizes, ButtonVariants};
use crate::components::ui::divider::Separator;
use crate::components::ui::sheet::{Sheet, SheetPopup, SheetTrigger};
use crate::components::ui::sidebar::SidebarTrigger;

#[component]
pub fn Header(channel: Signal<Option<Channel>>, members_open: RwSignal<bool>) -> impl IntoView {
    view! {
        <header class="bg-background sticky top-0 flex shrink-0 items-center gap-2 p-3 border-b">
            <SidebarTrigger class="-ml-1" />
            <Separator
                orientation=Orientation::Vertical
                class="mr-2 data-[orientation=vertical]:h-4"
            />
            {
                move || {
                    channel.get().map(|channel| view!{
                        <Breadcrumb>
                            <BreadcrumbList>
                                <BreadcrumbItem>
                                    <BreadcrumbPage>{channel.name}</BreadcrumbPage>
                                </BreadcrumbItem>
                                {
                                    channel.topic.map(|topic| view!{
                                        <Separator
                                            orientation=Orientation::Vertical
                                            class="data-[orientation=vertical]:h-4"
                                        />
                                        <BreadcrumbItem>
                                            <BreadcrumbPage>{topic}</BreadcrumbPage>
                                        </BreadcrumbItem>
                                    })
                                }
                            </BreadcrumbList>
                          </Breadcrumb>

                    })
                }
            }
            <div class="ml-auto mr-0 space-x-1">
                <Button
                    variant=ButtonVariants::Ghost
                    size=ButtonSizes::Icon
                    class="size-7"
                    {..}
                    on:click=move |_| {
                        members_open.update(|open| *open = !*open);
                    }
                >
                    <IconUsers/>
                </Button>
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
