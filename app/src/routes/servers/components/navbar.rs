use leptos::prelude::*;

use crate::components::auth::use_auth;
use crate::components::ui::sidebar::{SidebarMenuButton, SidebarMenuButtonSize};

#[component]
pub fn Navbar() -> impl IntoView {
    let auth = use_auth();
    view! {
        {
            move || {
                auth.user.signal.get().flatten().map(|user| {
                    view!{
                        <SidebarMenuButton
                          size=SidebarMenuButtonSize::Lg
                          class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground md:h-8 md:p-0"
                        >
                          <div class="h-8 w-8 rounded-lg">
                            <img class="w-full h-full" src={user.image_url} alt={user.name.clone()} />
                            // <div className="rounded-lg">CN</div>
                          </div>
                          // <div class="grid flex-1 text-left text-sm leading-tight">
                          //   <span class="truncate font-medium">{user.name.clone()}</span>
                          // </div>
                          // <IconChevronTop class="ml-auto size-4" />
                        </SidebarMenuButton>
                    }
                })
            }
        }
    }
}
