use crate::components::icons::IconImage;
use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::button::{Button, ButtonSizes, ButtonVariants};
use crate::components::ui::input::Input;
use crate::components::ui::tabs::{Tab, TabIndicator, TabPanel, Tabs, TabsList};
use crate::routes::servers::components::settings::content::{
    Setting, SettingAction, SettingData, SettingDescription, SettingTitle,
};
use crate::routes::use_profile;

use super::Title;
use leptos::prelude::*;

#[component]
pub fn Profiles() -> impl IntoView {
    view! {
        <Title>
            "Profiles"
        </Title>
        <Tabs default_tab="main">
            <TabsList>
                <Tab value="main">
                    "Account"
                </Tab>
                <Tab value="member">
                    "Members"
                </Tab>
                <TabIndicator/>
            </TabsList>
            <TabPanel value="main">
                <Profile/>
            </TabPanel>
            <TabPanel value="member">
                "Member's Profile"
            </TabPanel>
        </Tabs>
    }
}

#[component]
pub fn Profile() -> impl IntoView {
    let user = use_profile();
    view! {
        {
            move || {
                user.get().map(|user| {
                    view!{
                        <div class="flex flex-col items-center">
                            <Avatar class="flex relative bg-muted size-auto items-center justify-center rounded-lg w-full h-28">
                                <AvatarImage url=user.banner_url />
                                <AvatarFallback >
                                    <div/>
                                </AvatarFallback>

                                <Button class="absolute right-2 top-2" size=ButtonSizes::Sm variant=ButtonVariants::Outline>
                                    <IconImage />
                                    "Add cover image"
                                </Button>

                                //INFO: User image
                                <Avatar class="absolute shadow-xs left-2 aspect-square bg-background p-1 size-24 rounded-lg">
                                    <AvatarImage url=user.image_url class="rounded-md"/>
                                    <AvatarFallback class="rounded-lg">
                                        <div/>
                                    </AvatarFallback>
                                </Avatar>
                            </Avatar>
                        </div>
                        <Setting>
                            <SettingData>
                                <SettingTitle>
                                    "Username"
                                </SettingTitle>
                                <SettingDescription>
                                    "A unique name for your profile"
                                </SettingDescription>
                            </SettingData>
                            <SettingAction>
                                <Input />
                            </SettingAction>
                        </Setting>
                        <Setting>
                            <SettingData>
                                <SettingTitle>
                                    "About you"
                                </SettingTitle>
                                <SettingDescription>
                                    "Write a description for your profile"
                                </SettingDescription>
                            </SettingData>
                            <SettingAction>
                                <Input />
                            </SettingAction>
                        </Setting>
                    }
                })
            }
        }
    }
}
