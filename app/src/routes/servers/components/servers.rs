use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::sidebar::{SidebarMenuButton, SidebarMenuItem};
use crate::components::ui::tooltip::{ToolTip, ToolTipContent, ToolTipTrigger};

use super::sidebar::SideBarData;

#[component]
pub fn ServersItems(
    data: Signal<Option<Vec<SideBarData>>>,
    set_option: Callback<()>,
) -> impl IntoView {
    let params = use_params_map();
    view! {
        {
            move || {
                data.get().map(|data| {
                    data.into_iter().map(|data| {
                        let name = StoredValue::new(data.server.name.clone());
                        let id = StoredValue::new(data.server.id);
                        let is_active = Signal::derive(move || {
                            params
                                .get()
                                .get("server")
                                .is_some_and(|s| s == id.get_value())
                        });
                        let image_url = StoredValue::new(data.server.image_url);
                        let last_visited_id = StoredValue::new(data.member.last_visited_channel);
                        let href = move || if let Some(last) = last_visited_id.get_value() {
                            format!("/servers/{}/{}", id.get_value(), last)
                        } else {
                            format!("/servers/{}", id.get_value())
                        };
                        view!{
                            <SidebarMenuItem>
                                <ToolTip>
                                    <ToolTipTrigger>
                                        <A
                                            href=move || href()
                                            {..}
                                            on:click=move |_| {
                                                set_option.run(())
                                            }
                                        >
                                            <SidebarMenuButton
                                                is_active=is_active
                                                size=crate::components::ui::sidebar::SidebarMenuButtonSize::Sm
                                                class="md:h-8 md:p-0"
                                            >
                                                <Avatar class="h-8 w-8 rounded-lg">
                                                    <AvatarImage url=image_url.get_value()/>
                                                    <AvatarFallback class="rounded-lg text-muted-foreground select-none bg-transparent">
                                                        {name.get_value().chars().next()}
                                                    </AvatarFallback>
                                                </Avatar>
                                            </SidebarMenuButton>
                                        </A>
                                    </ToolTipTrigger>
                                    <ToolTipContent side_of_set=3.0 >
                                        {name.get_value()}
                                    </ToolTipContent>
                                </ToolTip>
                          </SidebarMenuItem>
                        }

                    }).collect_view()
                })
            }
        }
    }
}
