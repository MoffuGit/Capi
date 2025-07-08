use api::sidebar::{SideBarState, ToggleSideBar, SIDEBAR_COOKIE_NAME};
use leptos::prelude::*;
use tailwind_fuse::tw_merge;
use web_sys::MouseEvent;

const SIDEBAR_WIDTH: &str = "16rem";
const SIDEBAR_WIDTH_MOBILE: &str = "18rem";
const SIDEBAR_WIDTH_ICON: &str = "3rem";
const SIDEBAR_KEYBOARD_SHORTCUT: &str = "b";

use crate::components::icons::IconPanelLeft;
use crate::components::primitives::common::{is_mobile, Side};
use crate::components::ui::button::{Button, ButtonSizes, ButtonVariants};
use crate::components::ui::divider::Separator;
use crate::components::ui::input::Input;
use crate::components::ui::sheet::{Sheet, SheetPopup};
use crate::components::ui::skeleton::Skeleton;

#[derive(Clone)]
pub struct SidebarContextValue {
    open: RwSignal<bool>,
    open_mobile: RwSignal<bool>,
    is_mobile: Memo<bool>,
    state: Memo<SideBarState>,
    toggle_sidebar: Callback<()>,
}

pub fn use_sidebar() -> SidebarContextValue {
    use_context::<SidebarContextValue>().expect("useSidebar must be used within a SidebarProvider.")
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SidebarMenuButtonVariant {
    Default,
    Outline,
}

impl Default for SidebarMenuButtonVariant {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Debug, Clone, Copy, PartialEq, strum_macros::Display)]
pub enum SidebarMenuButtonSize {
    Default,
    Sm,
    Lg,
}

impl Default for SidebarMenuButtonSize {
    fn default() -> Self {
        Self::Default
    }
}

fn sidebar_menu_button_variants(
    variant: SidebarMenuButtonVariant,
    size: SidebarMenuButtonSize,
) -> String {
    let base_classes = "peer/menu-button flex w-full items-center gap-2 overflow-hidden rounded-md p-2 text-left text-sm outline-hidden ring-sidebar-ring transition-[width,height,padding] hover:bg-sidebar-accent hover:text-sidebar-accent-foreground focus-visible:ring-2 active:bg-sidebar-accent active:text-sidebar-accent-foreground disabled:pointer-events-none disabled:opacity-50 group-has-data-[sidebar=menu-action]/menu-item:pr-8 aria-disabled:pointer-events-none aria-disabled:opacity-50 data-[active=true]:bg-sidebar-accent data-[active=true]:font-medium data-[active=true]:text-sidebar-accent-foreground data-[state=open]:hover:bg-sidebar-accent data-[state=open]:hover:text-sidebar-accent-foreground group-data-[collapsible=icon]:size-8! group-data-[collapsible=icon]:p-2! [&>span:last-child]:truncate [&>svg]:size-4 [&>svg]:shrink-0";
    let variant_classes = match variant {
        SidebarMenuButtonVariant::Default => {
            "hover:bg-sidebar-accent hover:text-sidebar-accent-foreground"
        }
        SidebarMenuButtonVariant::Outline => {
            "bg-background shadow-[0_0_0_1px_hsl(var(--sidebar-border))] hover:bg-sidebar-accent hover:text-sidebar-accent-foreground hover:shadow-[0_0_0_1px_hsl(var(--sidebar-accent))]"
        }
    };
    let size_classes = match size {
        SidebarMenuButtonSize::Default => "h-8 text-sm",
        SidebarMenuButtonSize::Sm => "h-7 text-xs",
        SidebarMenuButtonSize::Lg => "h-12 text-sm group-data-[collapsible=icon]:p-0!",
    };

    tw_merge!(base_classes, variant_classes, size_classes)
}

#[cfg(not(feature = "ssr"))]
fn initial_state() -> bool {
    use wasm_bindgen::JsCast;
    let doc = document().unchecked_into::<web_sys::HtmlDocument>();
    let cookie = doc.cookie().unwrap_or_default();
    cookie.contains(&format!("{SIDEBAR_COOKIE_NAME}=expanded"))
}

// Server-side implementation for reading the sidebar state from cookies
#[cfg(feature = "ssr")]
fn initial_state() -> bool {
    use axum_extra::extract::cookie::CookieJar;
    use_context::<http::request::Parts>()
        .and_then(|req| {
            let cookies = CookieJar::from_headers(&req.headers);
            cookies
                .get(SIDEBAR_COOKIE_NAME)
                .and_then(|v| match v.value() {
                    "expanded" => Some(true),
                    "collapsed" => Some(false),
                    _ => None,
                })
        })
        .unwrap_or(false) // Default to collapsed if cookie is not present or invalid
}

#[component]
pub fn SidebarProvider(
    #[prop(optional, into, default = RwSignal::new(false))] open: RwSignal<bool>,
    #[prop(optional, into)] on_open_change: Option<Callback<(bool,), ()>>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] style: Option<String>,
    #[prop(optional, into, default = true)] main: bool,
    #[prop(optional, into)] shortcut: Option<String>,
    children: Children,
) -> impl IntoView {
    let state = Memo::new(move |_| {
        if open.get() {
            SideBarState::Expanded
        } else {
            SideBarState::Collapsed
        }
    });

    if main {
        open.set(initial_state());
        let toggle_sidebar_action = ServerAction::<ToggleSideBar>::new();
        Effect::new(move |_| toggle_sidebar_action.dispatch(ToggleSideBar { state: state.get() }));
    }

    let is_mobile = is_mobile();
    let open_mobile = RwSignal::new(false);

    let set_open_callback = Callback::new(move |value| {
        if let Some(cb) = &on_open_change {
            cb.run(value);
        } else {
            open.set(value.0);
        }
    });

    let toggle_sidebar = Callback::new(move |_| {
        if is_mobile.get_untracked() {
            open_mobile.update(|o| *o = !*o);
        } else {
            set_open_callback.run((!open.get_untracked(),));
        }
    });

    #[cfg(not(feature = "ssr"))]
    {
        use send_wrapper::SendWrapper;
        use wasm_bindgen::prelude::Closure;
        use wasm_bindgen::JsCast;
        let handle_key_down = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            if let Some(key) = shortcut.clone() {
                if event.key() == key && (event.meta_key() || event.ctrl_key()) {
                    event.prevent_default();
                    toggle_sidebar.run(());
                }
            } else if event.key() == SIDEBAR_KEYBOARD_SHORTCUT
                && (event.meta_key() || event.ctrl_key())
            {
                event.prevent_default();
                toggle_sidebar.run(());
            }
        }) as Box<dyn FnMut(web_sys::KeyboardEvent)>)
        .into_js_value();

        let closure = handle_key_down.clone();
        Effect::new(move |_| {
            window()
                .add_event_listener_with_callback(
                    "keydown",
                    handle_key_down.as_ref().unchecked_ref(),
                )
                .unwrap();
        });

        let cleanup_fn = {
            let closure_js = closure.clone();
            move || {
                let _ = window().remove_event_listener_with_callback(
                    "resize",
                    closure_js.as_ref().unchecked_ref(),
                );
            }
        };
        on_cleanup({
            let cleanup = SendWrapper::new(cleanup_fn);
            move || cleanup.take()()
        });
    }

    provide_context(SidebarContextValue {
        open,
        open_mobile,
        is_mobile,
        state,
        toggle_sidebar,
    });

    let style_str = Memo::new(move |_| {
        let mut s = format!(
            "--sidebar-width: {SIDEBAR_WIDTH}; --sidebar-width-icon: {SIDEBAR_WIDTH_ICON}; "
        );
        if let Some(user_style) = style.as_ref() {
            s.push_str(user_style);
        }
        s
    });

    view! {
        <div
            data-slot="sidebar-wrapper"
            style=style_str
            class=tw_merge!("group/sidebar-wrapper has-data-[variant=inset]:bg-sidebar flex min-h-svh w-full", class)
        >
            {children()}
        </div>
    }
}

#[derive(Debug, PartialEq, Clone, Copy, strum_macros::Display)]
pub enum SideBarVariant {
    #[strum(to_string = "floating")]
    Floating,
    #[strum(to_string = "inset")]
    Inset,
    #[strum(to_string = "sidebar")]
    Sidebar,
}

#[derive(Debug, PartialEq, Clone, Copy, strum_macros::Display)]
pub enum SideBarCollapsible {
    #[strum(to_string = "offcanvas")]
    Offcanvas,
    #[strum(to_string = "icon")]
    Icon,
    #[strum(to_string = "none")]
    None,
}
#[component]
pub fn Sidebar(
    #[prop(optional, into, default = Side::Left)] side: Side,
    #[prop(optional, into, default = SideBarVariant::Sidebar
    )]
    variant: SideBarVariant,
    #[prop(optional, into, default = SideBarCollapsible::Offcanvas)]
    collapsible: SideBarCollapsible,
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let sidebar_context = use_sidebar();
    let is_mobile = sidebar_context.is_mobile;
    let open_mobile = sidebar_context.open_mobile;
    let state = sidebar_context.state;

    if collapsible == SideBarCollapsible::None {
        return view! {
            <div
                data-slot="sidebar"
                class=move || tw_merge!("bg-sidebar text-sidebar-foreground flex h-full w-(--sidebar-width) flex-col", class.get())
            >
                {children()}
            </div>
        }
        .into_any();
    }

    let children = StoredValue::new(children);

    let sidebar_gap_class = Memo::new(move |_| {
        tw_merge!("relative w-(--sidebar-width) bg-transparent transition-[width] duration-200 ease-linear",  "group-data-[collapsible=offcanvas]:w-0", &tw_merge!("group-data-[side={side}]"), if variant == SideBarVariant::Floating|| variant == SideBarVariant::Inset{
            "group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)+(--spacing(4)))]"
        } else {
            "group-data-[collapsible=icon]:w-(--sidebar-width-icon)"
        })
    });

    let sidebar_container_class = Memo::new(move |_| {
        tw_merge!("fixed inset-y-0 z-10 hidden h-svh w-(--sidebar-width) transition-[left,right,width] duration-200 ease-linear md:flex", if side == Side::Left {
            "left-0 group-data-[collapsible=offcanvas]:left-[calc(var(--sidebar-width)*-1)]"
        } else {
            "right-0 group-data-[collapsible=offcanvas]:right-[calc(var(--sidebar-width)*-1)]"
        },
        if variant == SideBarVariant::Floating || variant == SideBarVariant::Inset {
            "p-2 group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)+(--spacing(4))+2px)]"
        } else {
            "group-data-[collapsible=icon]:w-(--sidebar-width-icon) group-data-[side=left]:border-r group-data-[side=right]:border-l"
        },
            class.get()
        )
    });
    view! {
        <Show when=move || is_mobile.get()>
            <Sheet
                open=open_mobile
            >
                <SheetPopup
                    side=side
                    class="bg-sidebar text-sidebar-foreground w-(--sidebar-width) p-0 [&>button]:hidden"
                    {..}
                    data-sidebar="sidebar"
                    data-slot="sidebar"
                    data-mobile="true"
                    style=format!("--sidebar-width: {};", SIDEBAR_WIDTH_MOBILE)
                >
                    // <SheetHeader class="sr-only".to_string()>
                    //     <SheetTitle>"Sidebar"</SheetTitle>
                    //     <SheetDescription>"Displays the mobile sidebar."</SheetDescription>
                    // </SheetHeader>
                    <div class="flex h-full w-full flex-col">{move ||children.get_value()()}</div>
                </SheetPopup>
            </Sheet>
        </Show>
        <Show when=move || !is_mobile.get()>
            <div
                class="group peer text-sidebar-foreground hidden md:block"
                data-state=move || state().to_string()
                data-collapsible=Memo::new(move |_| {
                    if state.get() == SideBarState::Collapsed {
                        collapsible.to_string()
                    } else {
                        "".to_string()
                    }
                })
                data-variant=variant.to_string()
                data-side=side.to_string()
                data-slot="sidebar"
            >
                // This is what handles the sidebar gap on desktop
                <div
                    data-slot="sidebar-gap"
                    class=sidebar_gap_class
                />
                <div
                    data-slot="sidebar-container"
                    class=sidebar_container_class
                >
                    <div
                        data-sidebar="sidebar"
                        data-slot="sidebar-inner"
                        class="bg-sidebar group-data-[variant=floating]:border-sidebar-border flex h-full w-full flex-col group-data-[variant=floating]:rounded-lg group-data-[variant=floating]:border group-data-[variant=floating]:shadow-sm"
                    >
                        {children.get_value()()}
                    </div>
                </div>
            </div>
        </Show>
    }.into_any()
}

#[component]
pub fn SidebarTrigger(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] on_click: Option<Box<dyn Fn(MouseEvent)>>,
) -> impl IntoView {
    let SidebarContextValue { toggle_sidebar, .. } = use_sidebar();
    view! {
        <Button
            variant=ButtonVariants::Ghost
            size=ButtonSizes::Icon
            class=Signal::derive(move || tw_merge!( "size-7", class.get()))
            {..}
            data-sidebar="trigger"
            data-slot="sidebar-trigger"
            on:click=move |event| {
                if let Some(cb) = &on_click {
                    cb(event);
                }
                toggle_sidebar.run(());
            }
        >
            <IconPanelLeft/>
            <span class="sr-only">"Toggle Sidebar"</span>
        </Button>
    }
}

#[component]
pub fn SidebarRail(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    let SidebarContextValue { toggle_sidebar, .. } = use_sidebar();

    view! {
        <button
            data-sidebar="rail"
            data-slot="sidebar-rail"
            aria-label="Toggle Sidebar"
            tabindex="-1"
            on:click=move |_| toggle_sidebar.run(())
            title="Toggle Sidebar"
            class=move || tw_merge!(
                "hover:after:bg-sidebar-border absolute inset-y-0 z-20 hidden w-4 -translate-x-1/2 transition-all ease-linear group-data-[side=left]:-right-4 group-data-[side=right]:left-0 after:absolute after:inset-y-0 after:left-1/2 after:w-[2px] sm:flex",
                "in-data-[side=left]:cursor-w-resize in-data-[side=right]:cursor-e-resize",
                "[[data-side=left][data-state=collapsed]_&]:cursor-e-resize [[data-side=right][data-state=collapsed]_&]:cursor-w-resize",
                "hover:group-data-[collapsible=offcanvas]:bg-sidebar group-data-[collapsible=offcanvas]:translate-x-0 group-data-[collapsible=offcanvas]:after:left-full",
                "[[data-side=left][data-collapsible=offcanvas]_&]:-right-2",
                "[[data-side=right][data-collapsible=offcanvas]_&]:-left-2",
                class.get(),
            )
        />
    }
}

#[component]
pub fn SidebarInset(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <main
            data-slot="sidebar-inset"
            class=move || tw_merge!(
                "bg-background relative flex w-full flex-1 flex-col",
                "md:peer-data-[variant=inset]:m-2 md:peer-data-[variant=inset]:ml-0 md:peer-data-[variant=inset]:rounded-xl md:peer-data-[variant=inset]:shadow-sm md:peer-data-[variant=inset]:peer-data-[state=collapsed]:ml-2",
                class.get(),
            )
        >
            {children()}
        </main>
    }
}

#[component]
pub fn SidebarInput(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <Input
            class=Signal::derive(move || tw_merge!("bg-background h-8 w-full shadow-none {}", class.get()))
            {..}
            data-slot="sidebar-input"
            data-sidebar="input"
        />
    }
}

#[component]
pub fn SidebarHeader(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="sidebar-header"
            data-sidebar="header"
            class=move || tw_merge!("flex flex-col gap-2 p-2", &class.get())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarFooter(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="sidebar-footer"
            data-sidebar="footer"
            class=move || tw_merge!["flex flex-col gap-2 p-2", class.get()]
        >
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarSeparator(#[prop(optional, into)] class: MaybeProp<String>) -> impl IntoView {
    view! {
        <Separator
            class=tw_merge!("bg-sidebar-border mx-2 w-auto", &class.get())
            {..}
            data-slot="sidebar-separator"
            data-sidebar="separator"
        />
    }
}

#[component]
pub fn SidebarContent(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="sidebar-content"
            data-sidebar="content"
            class=move || tw_merge!(
                "flex min-h-0 flex-1 flex-col gap-2 overflow-auto group-data-[collapsible=icon]:overflow-hidden",
                class.get(),
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarGroup(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="sidebar-group"
            data-sidebar="group"
            class=move || tw_merge!("relative flex w-full min-w-0 flex-col p-2", class.get())
        >
            {children()}
        </div>
    }
}
#[component]
pub fn SidebarGroupLabel(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] as_child: bool,
    children: Children,
) -> impl IntoView {
    if as_child {
        return view! {{children()}}.into_any();
    }

    view! {
        <div
            data-slot="sidebar-group-label"
            data-sidebar="group-label"
            class=move || tw_merge!(
                "text-sidebar-foreground/70 ring-sidebar-ring flex h-8 shrink-0 items-center rounded-md px-2 text-xs font-medium outline-hidden transition-[margin,opacity] duration-200 ease-linear focus-visible:ring-2 [&>svg]:size-4 [&>svg]:shrink-0",
                "group-data-[collapsible=icon]:-mt-8 group-data-[collapsible=icon]:opacity-0",
                class.get(),
            )
        >
            {children()}
        </div>
    }.into_any()
}

#[component]
pub fn SidebarGroupAction(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] as_child: bool,
    children: Children,
) -> impl IntoView {
    if as_child {
        return view! {{children()}}.into_any();
    }

    view! {
        <button
            data-slot="sidebar-group-action"
            data-sidebar="group-action"
            class=move || tw_merge!(
                "text-sidebar-foreground ring-sidebar-ring hover:bg-sidebar-accent hover:text-sidebar-accent-foreground absolute top-3.5 right-3 flex aspect-square w-5 items-center justify-center rounded-md p-0 outline-hidden transition-transform focus-visible:ring-2 [&>svg]:size-4 [&>svg]:shrink-0",
                "after:absolute after:-inset-2 md:after:hidden",
                "group-data-[collapsible=icon]:hidden",
                class.get(),
            )
        >
            {children()}
        </button>
    }.into_any()
}

#[component]
pub fn SidebarGroupContent(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="sidebar-group-content"
            data-sidebar="group-content"
            class=tw_merge!("w-full text-sm", &class.get())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarMenu(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <ul
            data-slot="sidebar-menu"
            data-sidebar="menu"
            class=move || tw_merge!("flex w-full min-w-0 flex-col gap-1", class.get())
        >
            {children()}
        </ul>
    }
}

#[component]
pub fn SidebarMenuItem(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <li
            data-slot="sidebar-menu-item"
            data-sidebar="menu-item"
            class=tw_merge!("group/menu-item relative", class.get())
        >
            {children()}
        </li>
    }
}

#[component]
pub fn SidebarMenuButton(
    #[prop(optional, into)] is_active: Signal<bool>,
    #[prop(default = SidebarMenuButtonVariant::Default)] variant: SidebarMenuButtonVariant,
    #[prop(default = SidebarMenuButtonSize::Default)] size: SidebarMenuButtonSize,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let button_class =
        Signal::derive(move || tw_merge!(sidebar_menu_button_variants(variant, size), class.get()));

    view! {
        <div
            class=button_class
            data-active=move || is_active.get().to_string()
            data-sidebar="menu-button"
            data-slot="sidebar-menu-button"
            data-size=size.to_string()
        >
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarMenuAction(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] show_on_hover: bool,
    children: Children,
) -> impl IntoView {
    let show_on_hover_class = if show_on_hover {
        "peer-data-[active=true]/menu-button:text-sidebar-accent-foreground group-focus-within/menu-item:opacity-100 group-hover/menu-item:opacity-100 data-[state=open]:opacity-100 md:opacity-0"
    } else {
        ""
    };

    view! {
        <button
            data-slot="sidebar-menu-action"
            data-sidebar="menu-action"
            class=move || tw_merge!(
                "text-sidebar-foreground ring-sidebar-ring hover:bg-sidebar-accent hover:text-sidebar-accent-foreground peer-hover/menu-button:text-sidebar-accent-foreground absolute top-1.5 right-1 flex aspect-square w-5 items-center justify-center rounded-md p-0 outline-hidden transition-transform focus-visible:ring-2 [&>svg]:size-4 [&>svg]:shrink-0",
                "after:absolute after:-inset-2 md:after:hidden",
                "peer-data-[size=sm]/menu-button:top-1",
                "peer-data-[size=default]/menu-button:top-1.5",
                "peer-data-[size=lg]/menu-button:top-2.5",
                "group-data-[collapsible=icon]:hidden",
                show_on_hover_class,
                class.get(),
            )
        >
            {children()}
        </button>
    }
}

#[component]
pub fn SidebarMenuBadge(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="sidebar-menu-badge"
            data-sidebar="menu-badge"
            class=move || tw_merge!(
                "text-sidebar-foreground pointer-events-none absolute right-1 flex h-5 min-w-5 items-center justify-center rounded-md px-1 text-xs font-medium tabular-nums select-none",
                "peer-hover/menu-button:text-sidebar-accent-foreground peer-data-[active=true]/menu-button:text-sidebar-accent-foreground",
                "peer-data-[size=sm]/menu-button:top-1",
                "peer-data-[size=default]/menu-button:top-1.5",
                "peer-data-[size=lg]/menu-button:top-2.5",
                "group-data-[collapsible=icon]:hidden",
                class.get(),
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarMenuSkeleton(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] show_icon: bool,
) -> impl IntoView {
    let width = Memo::new(move |_| {
        // randomRange
        // let mut rng = rand::rng();
        // let random_width = rng.random_range(50..=90);
        format!("{}%", 20)
    });

    view! {
        <div
            data-slot="sidebar-menu-skeleton"
            data-sidebar="menu-skeleton"
            class=move || tw_merge!( "flex h-8 items-center gap-2 rounded-md px-2", class.get())
        >
            {show_icon.then(|| view! {
                <Skeleton
                    class="size-4 rounded-md"
                    {..}
                    data-sidebar="menu-skeleton-icon"
                />
            })}
            <Skeleton
                class="h-4 flex-1"
                {..}
                data-sidebar="menu-skeleton-text"
                style=Memo::new(move |_| { format!("max-width: {};", width.get()) })
            />
        </div>
    }
}

#[component]
pub fn SidebarMenuSub(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <ul
            data-slot="sidebar-menu-sub"
            data-sidebar="menu-sub"
            class=move || tw_merge!(
                "border-sidebar-border mx-3.5 flex min-w-0 translate-x-px flex-col gap-1 border-l px-2.5 py-0.5",
                "group-data-[collapsible=icon]:hidden",
                class.get(),
            )
        >
            {children()}
        </ul>
    }
}

#[component]
pub fn SidebarMenuSubItem(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <li
            data-slot="sidebar-menu-sub-item"
            data-sidebar="menu-sub-item"
            class=move || tw_merge!( "group/menu-sub-item relative", class.get())
        >
            {children()}
        </li>
    }
}

#[component]
pub fn SidebarMenuSubButton(
    #[prop(optional)] as_child: bool,
    #[prop(optional, default = "md".to_string())] size: String,
    #[prop(optional)] is_active: bool,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    if as_child {
        return children().into_any();
    }

    let size_class = if size == "sm" {
        "text-xs"
    } else if size == "md" {
        "text-sm"
    } else {
        ""
    };

    view! {
        <a
            data-slot="sidebar-menu-sub-button"
            data-sidebar="menu-sub-button"
            data-size=size.clone()
            data-active=is_active.to_string()
            class=move || tw_merge!(
                "text-sidebar-foreground ring-sidebar-ring hover:bg-sidebar-accent hover:text-sidebar-accent-foreground active:bg-sidebar-accent active:text-sidebar-accent-foreground [&>svg]:text-sidebar-accent-foreground flex h-7 min-w-0 -translate-x-px items-center gap-2 overflow-hidden rounded-md px-2 outline-hidden focus-visible:ring-2 disabled:pointer-events-none disabled:opacity-50 aria-disabled:pointer-events-none aria-disabled:opacity-50 [&>span:last-child]:truncate [&>svg]:size-4 [&>svg]:shrink-0",
                "data-[active=true]:bg-sidebar-accent data-[active=true]:text-sidebar-accent-foreground",
                size_class,
                "group-data-[collapsible=icon]:hidden",
                class.get(),
            )
        >
            {children()}
        </a>
    }.into_any()
}
