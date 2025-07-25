use crate::components::icons::{IconImage, IconX};
use crate::components::primitives::tooltip::ToolTipSide;
use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::button::{Button, ButtonSizes, ButtonVariants};
use crate::components::ui::input::Input;
use crate::components::ui::tabs::{Tab, TabIndicator, TabPanel, Tabs, TabsList};
use crate::components::ui::tooltip::{ToolTip, ToolTipContent, ToolTipTrigger};
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
                            <Avatar class="flex relative bg-muted size-auto items-center justify-center rounded-lg mt-4 w-full h-28">
                                <AvatarImage url=user.banner_url />
                                <AvatarFallback >
                                    <div/>
                                </AvatarFallback>

                                <Button class="absolute right-2 top-2" size=ButtonSizes::Sm variant=ButtonVariants::Outline>
                                    <IconImage />
                                    "Add cover image"
                                </Button>

                                //INFO: User image
                                <Avatar class="absolute shadow-xs left-2 aspect-square bg-background p-1 size-24 rounded-lg overflow-visible">
                                    <ToolTip>
                                        <ToolTipTrigger class="absolute inset-1 hover:bg-accent/50 group flex items-center justify-center rounded-lg">
                                            <Button size=ButtonSizes::Icon variant=ButtonVariants::Outline class="absolute opacity-0 size-5 group-hover:opacity-100 transition-opacity ease-out top-0 right-0 -translate-y-1/2 translate-x-1/2 rounded-md p-0 ">
                                                <IconX/>
                                            </Button >
                                        </ToolTipTrigger>
                                        <ToolTipContent side_of_set=10.0 side=ToolTipSide::Bottom>
                                            "Edit your image"
                                        </ToolTipContent>
                                    </ToolTip>
                                    <AvatarImage url=user.image_url class="rounded-md"/>
                                    <AvatarFallback class="rounded-lg">
                                        <div/>
                                    </AvatarFallback>
                                </Avatar>
                            </Avatar>
                        </div>
                        <Setting class="mt-6">
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
