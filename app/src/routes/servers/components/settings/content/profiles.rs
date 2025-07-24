use crate::components::ui::tabs::{Tab, TabIndicator, TabPanel, Tabs, TabsList};

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
                "Profile"
            </TabPanel>
            <TabPanel value="member">
                "Member's Profile"
            </TabPanel>
        </Tabs>
    }
}
