use icons::{IconMoon, IconSun};
use crate::components::ui::button::{Button, ButtonSizes, ButtonVariants};
use crate::components::ui::theme::use_theme;
use crate::routes::home::components::user_settings::content::SettingAction;

use super::{Setting, SettingData, SettingDescription, SettingTitle};

use super::Title;
use leptos::prelude::*;

#[component]
pub fn Preferences() -> impl IntoView {
    view! {
        <Title>
            "Preferences"
        </Title>
        <Appearance/>
    }
}

#[component]
pub fn Appearance() -> impl IntoView {
    let theme = use_theme();
    let selected = theme.prefers_theme;
    let select_theme = theme.select_theme;
    view! {
        <Setting>
            <SettingData>
                <SettingTitle>
                    "Appearance"
                </SettingTitle>
                <SettingDescription>
                    "Customize how Capi looks on you device"
                </SettingDescription>
            </SettingData>
            <SettingAction class="flex gap-2 items-center">
                <Button
                    on:click=move |_| {
                        select_theme.run(&true)
                    }
                    variant=Signal::derive(move || {
                        if selected.get() {
                            ButtonVariants::Ghost
                        } else {
                            ButtonVariants::Default
                        }
                    }) size=ButtonSizes::Icon>
                        <IconMoon/>
                </Button>
                <Button
                    on:click=move |_| {
                        select_theme.run(&false)
                    }
                    variant=Signal::derive(move || {
                        if !selected.get() {
                            ButtonVariants::Ghost
                        } else {
                            ButtonVariants::Default
                        }
                    }) size=ButtonSizes::Icon>
                        <IconSun/>
                </Button>
            </SettingAction>
        </Setting>

    }
}
