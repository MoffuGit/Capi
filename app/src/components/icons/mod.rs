#![allow(dead_code)]

use leptos::prelude::*;

#[component]
pub fn Icon(children: Children, #[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class=class
        >
            {children()}
        </svg>
    }
}

#[component]
pub fn IconPencil(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z"/><path d="m15 5 4 4"/>
        </Icon>
    }
}

#[component]
pub fn IconCompass(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="m16.24 7.76-1.804 5.411a2 2 0 0 1-1.265 1.265L7.76 16.24l1.804-5.411a2 2 0 0 1 1.265-1.265z"/>
            <circle cx="12" cy="12" r="10"/>
        </Icon>
    }
}

#[component]
pub fn IconGlobe(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <circle cx="12" cy="12" r="10"/>
            <path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"/>
            <path d="M2 12h20"/>
        </Icon>
    }
}

#[component]
pub fn IconX(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="M18 6 6 18"/>
            <path d="m6 6 12 12"/>
        </Icon>
    }
}

#[component]
pub fn IconSticker(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="M15.5 3H5a2 2 0 0 0-2 2v14c0 1.1.9 2 2 2h14a2 2 0 0 0 2-2V8.5L15.5 3Z"/>
            <path d="M14 3v4a2 2 0 0 0 2 2h4"/>
            <path d="M8 13h.01"/>
            <path d="M16 13h.01"/>
            <path d="M10 16s.8 1 2 1c1.3 0 2-1 2-1"/>
        </Icon>
    }
}

#[component]
pub fn IconChevronDown(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="m6 9 6 6 6-6"/>
        </Icon>
    }
}

#[component]
pub fn IconChevronLeft(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="m15 18-6-6 6-6"/>
        </Icon>
    }
}

#[component]
pub fn IconChevronRight(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="m9 18 6-6-6-6"/>
        </Icon>
    }
}

#[component]
pub fn IconChevronTop(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="m18 15-6-6-6 6"/>
        </Icon>
    }
}

#[component]
pub fn IconPaperClip(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="M13.234 20.252 21 12.3"/>
            <path d="m16 6-8.414 8.586a2 2 0 0 0 0 2.828 2 2 0 0 0 2.828 0l8.414-8.586a4 4 0 0 0 0-5.656 4 4 0 0 0-5.656 0l-8.415 8.585a6 6 0 1 0 8.486 8.486"/>
        </Icon>
    }
}

#[component]
pub fn IconTrash(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="M3 6h18"/>
            <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/>
            <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/>
            <line x1="10" x2="10" y1="11" y2="17"/>
            <line x1="14" x2="14" y1="11" y2="17"/>
        </Icon>
    }
}

#[component]
pub fn IconCirclePlus(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <circle cx="12" cy="12" r="10"/>
            <path d="M8 12h8"/>
            <path d="M12 8v8"/>
        </Icon>
    }
}

#[component]
pub fn IconPlus(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="M5 12h14"/>
            <path d="M12 5v14"/>
        </Icon>
    }
}

#[component]
pub fn IconSearch(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <circle cx="11" cy="11" r="8"/>
            <path d="m21 21-4.3-4.3"/>
        </Icon>
    }
}

#[component]
pub fn IconCommand(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="M15 6v12a3 3 0 1 0 3-3H6a3 3 0 1 0 3 3V6a3 3 0 1 0-3 3h12a3 3 0 1 0-3-3"/>
        </Icon>
    }
}

#[component]
pub fn IconMessageCircle(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="M7.9 20A9 9 0 1 0 4 16.1L2 22Z"/>
        </Icon>
    }
}

#[component]
pub fn IconInbox(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <polyline points="22 12 16 12 14 15 10 15 8 12 2 12"/>
            <path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"/>
        </Icon>
    }
}

#[component]
pub fn IconEllipsis(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <circle cx="12" cy="12" r="1"/>
            <circle cx="19" cy="12" r="1"/>
            <circle cx="5" cy="12" r="1"/>
        </Icon>
    }
}

#[component]
pub fn IconLoaderCircle(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="M21 12a9 9 0 1 1-6.219-8.56">
            </path>
        </Icon>
    }
}

#[component]
pub fn IconPanelLeft(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <rect width="18" height="18" x="3" y="3" rx="2" />
            <path d="M9 3v18" />
        </Icon>
    }
}

#[component]
pub fn IconPin(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="M12 17v5"/>
            <path d="M9 10.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24V16a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-.76a2 2 0 0 0-1.11-1.79l-1.78-.9A2 2 0 0 1 15 10.76V7a1 1 0 0 1 1-1 2 2 0 0 0 0-4H8a2 2 0 0 0 0 4 1 1 0 0 1 1 1z"/>
        </Icon>
    }
}

#[component]
pub fn IconUsers(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/>
            <circle cx="9" cy="7" r="4"/>
            <path d="M22 21v-2a4 4 0 0 0-3-3.87"/>
            <path d="M16 3.13a4 4 0 0 1 0 7.75"/>
        </Icon>
    }
}

#[component]
pub fn IconListTree(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="M21 12h-8" />
            <path d="M21 6H8" />
            <path d="M21 18h-8" />
            <path d="M3 6v4c0 1.1.9 2 2 2h3" />
            <path d="M3 10v6c0 1.1.9 2 2 2h3" />
        </Icon>
    }
}
