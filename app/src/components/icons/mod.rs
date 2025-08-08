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
pub fn IconFile(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"/><path d="M14 2v4a2 2 0 0 0 2 2h4"/>
        </Icon>
    }
}

#[component]
pub fn IconCircle(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <circle cx="12" cy="12" r="10"/>
        </Icon>
    }
}

#[component]
pub fn IconFileText(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"/><path d="M14 2v4a2 2 0 0 0 2 2h4"/><path d="M10 9H8"/><path d="M16 13H8"/><path d="M16 17H8"/>
        </Icon>
    }
}

#[component]
pub fn IconFileAudio(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="M17.5 22h.5a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v3"/><path d="M14 2v4a2 2 0 0 0 2 2h4"/><path d="M2 19a2 2 0 1 1 4 0v1a2 2 0 1 1-4 0v-4a6 6 0 0 1 12 0v4a2 2 0 1 1-4 0v-1a2 2 0 1 1 4 0"/>
        </Icon>
    }
}

#[component]
pub fn IconFileVideo(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v4"/><path d="M14 2v4a2 2 0 0 0 2 2h4"/><rect width="8" height="6" x="2" y="12" rx="1"/><path d="m10 13.843 3.033-1.755a.645.645 0 0 1 .967.56v4.704a.645.645 0 0 1-.967.56L10 16.157"/>
        </Icon>
    }
}

#[component]
pub fn IconFileArchive(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="M10 12v-1"/><path d="M10 18v-2"/><path d="M10 7V6"/><path d="M14 2v4a2 2 0 0 0 2 2h4"/><path d="M15.5 22H18a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v16a2 2 0 0 0 .274 1.01"/><circle cx="10" cy="20" r="2"/>
        </Icon>
    }
}

#[component]
pub fn IconCornerUpLeft(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="M20 20v-7a4 4 0 0 0-4-4H4"/><path d="M9 14 4 9l5-5"/>
        </Icon>
    }
}

#[component]
pub fn IconSend(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
        <path d="M14.536 21.686a.5.5 0 0 0 .937-.024l6.5-19a.496.496 0 0 0-.635-.635l-19 6.5a.5.5 0 0 0-.024.937l7.93 3.18a2 2 0 0 1 1.112 1.11z"/><path d="m21.854 2.147-10.94 10.939"/>
        </Icon>
    }
}

#[component]
pub fn IconImage(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <rect width="18" height="18" x="3" y="3" rx="2" ry="2"/><circle cx="9" cy="9" r="2"/><path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21"/>
        </Icon>
    }
}

#[component]
pub fn IconMoon(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
        <path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"/>
        </Icon>
    }
}

#[component]
pub fn IconSun(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
        <circle cx="12" cy="12" r="4"/><path d="M12 2v2"/><path d="M12 20v2"/><path d="m4.93 4.93 1.41 1.41"/><path d="m17.66 17.66 1.41 1.41"/><path d="M2 12h2"/><path d="M20 12h2"/><path d="m6.34 17.66-1.41 1.41"/><path d="m19.07 4.93-1.41 1.41"/>
        </Icon>
    }
}

#[component]
pub fn IconMic(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
        <path d="M12 19v3"/><path d="M19 10v2a7 7 0 0 1-14 0v-2"/><rect x="9" y="2" width="6" height="13" rx="3"/>
        </Icon>
    }
}

#[component]
pub fn IconHeadphones(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="M3 14h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-7a9 9 0 0 1 18 0v7a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3"/>
        </Icon>
    }
}

#[component]
pub fn IconLink(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/><path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/>
        </Icon>

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
pub fn IconBox(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z"/><path d="m3.3 7 8.7 5 8.7-5"/><path d="M12 22V12"/>
        </Icon>
    }
}

#[component]
pub fn IconSettings(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/>
        </Icon>
    }
}

#[component]
pub fn IconSettings2(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="M14 17H5"/><path d="M19 7h-9"/><circle cx="17" cy="17" r="3"/><circle cx="7" cy="7" r="3"/>
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
pub fn IconLoader(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="M12 2v4"/><path d="m16.2 7.8 2.9-2.9"/><path d="M18 12h4"/><path d="m16.2 16.2 2.9 2.9"/><path d="M12 18v4"/><path d="m4.9 19.1 2.9-2.9"/><path d="M2 12h4"/><path d="m4.9 4.9 2.9 2.9"/>
        </Icon>
    }
}

#[component]
pub fn IconCheck(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <path d="M20 6 9 17l-5-5"/>
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
pub fn IconCircleUser(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon class=class>
            <circle cx="12" cy="12" r="10"/><circle cx="12" cy="10" r="3"/><path d="M7 20.662V19a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v1.662"/>
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
