use common::convex::{Role, RoleActions};
use leptos::context::Provider;
use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct UserRoles {
    pub roles: Vec<Role>,
}

impl UserRoles {
    pub fn new(roles: Vec<Role>) -> Self {
        Self { roles }
    }

    fn can_perform_action(&self, action_check: impl Fn(&RoleActions) -> bool) -> bool {
        self.roles.iter().any(|role| action_check(&role.actions))
    }

    pub fn can_manage_channels(&self) -> bool {
        self.can_perform_action(|actions| actions.can_manage_channels)
    }

    pub fn can_manage_categories(&self) -> bool {
        self.can_perform_action(|actions| actions.can_manage_categories)
    }

    pub fn can_manage_roles(&self) -> bool {
        self.can_perform_action(|actions| actions.can_manage_roles)
    }

    pub fn can_manage_members(&self) -> bool {
        self.can_perform_action(|actions| actions.can_manage_members)
    }

    pub fn can_manage_server_settings(&self) -> bool {
        self.can_perform_action(|actions| actions.can_manage_server_settings)
    }

    pub fn can_create_invitation(&self) -> bool {
        self.can_perform_action(|actions| actions.can_create_invitation)
    }
}

pub fn use_roles() -> Memo<UserRoles> {
    use_context().expect("Should have access to the RolesContext")
}

#[component]
pub fn RolesProvider(
    #[prop(into)] roles: Signal<Option<Vec<Role>>>,
    children: Children,
) -> impl IntoView {
    let user_roles = Memo::new(move |_| {
        if let Some(r) = roles.get() {
            UserRoles::new(r)
        } else {
            UserRoles::new(vec![])
        }
    });

    view! {
        <Provider value=user_roles>
            {children()}
        </Provider>
    }
}

#[component]
pub fn CanManageChannels(children: ChildrenFn) -> impl IntoView {
    let user_roles = use_roles();
    view! {
        <Transition>
            <Show when=move || user_roles.get().can_manage_channels()>
                {children()}
            </Show>
        </Transition>
    }
}

#[component]
pub fn CanManageCategories(children: ChildrenFn) -> impl IntoView {
    let user_roles = use_roles();
    view! {
        <Transition>
            <Show when=move || user_roles.get().can_manage_categories()>
                {children()}
            </Show>
        </Transition>
    }
}

#[component]
pub fn CanManageRoles(children: ChildrenFn) -> impl IntoView {
    let user_roles = use_roles();
    view! {
        <Transition>
            <Show when=move || user_roles.get().can_manage_roles()>
                {children()}
            </Show>
        </Transition>
    }
}

#[component]
pub fn CanManageMembers(children: ChildrenFn) -> impl IntoView {
    let user_roles = use_roles();
    view! {
        <Transition>
            <Show when=move || user_roles.get().can_manage_members()>
                {children()}
            </Show>
        </Transition>
    }
}

#[component]
pub fn CanManageServerSettings(children: ChildrenFn) -> impl IntoView {
    let user_roles = use_roles();
    view! {
        <Transition>
            <Show when=move || user_roles.get().can_manage_server_settings()>
                {children()}
            </Show>
        </Transition>
    }
}

#[component]
pub fn CanCreateInvitation(children: ChildrenFn) -> impl IntoView {
    let user_roles = use_roles();
    view! {
        <Transition>
            <Show when=move || user_roles.get().can_create_invitation()>
                {children()}
            </Show>
        </Transition>
    }
}
