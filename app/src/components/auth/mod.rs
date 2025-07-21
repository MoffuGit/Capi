pub mod form;

use api::auth::{get_user, GoogleAuth, HandleGoogleRedirect, Logout, RefreshToken};
use common::convex::User;
use common::user::User as Auth;
use convex_client::leptos::{Query, UseQuery};
use leptos::context::Provider;
use leptos::prelude::*;
use leptos_dom::error;
use leptos_router::hooks::{use_navigate, use_query};
use leptos_router::params::Params;
use leptos_router::NavigateOptions;
use serde::Serialize;

use crate::components::ui::button::Button;

use super::ui::button::{ButtonSizes, ButtonVariants};

#[derive(Clone)]
pub struct AuthContext {
    log_out: ServerAction<Logout>,
    google_auth: ServerAction<GoogleAuth>,
    handle_google_redirect: ServerAction<HandleGoogleRedirect>,
    refresh_google_token: ServerAction<RefreshToken>,
    pub auth: Resource<Result<Option<Auth>, ServerFnError>>,
    pub user: Signal<Option<User>>,
    expires_in: RwSignal<u64>,
}

pub fn use_auth() -> AuthContext {
    use_context().expect("shoud acces to the auth context")
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetUser {
    auth: i64,
}

impl Query<Option<User>> for GetUser {
    fn name(&self) -> String {
        "user:getUser".to_string()
    }
}

#[component]
pub fn AuthProvider(children: Children) -> impl IntoView {
    let refresh_google_token = ServerAction::<RefreshToken>::new();
    let log_out = ServerAction::<Logout>::new();
    let google_auth = ServerAction::<GoogleAuth>::new();
    let handle_google_redirect = ServerAction::<HandleGoogleRedirect>::new();
    let auth = Resource::new(
        move || {
            (
                log_out.version().get(),
                refresh_google_token.version().get(),
            )
        },
        move |_| get_user(),
    );

    let user = UseQuery::new(move || {
        if let Some(Ok(Some(auth))) = auth.get() {
            Some(GetUser { auth: auth.id })
        } else {
            None
        }
    });

    let user = Signal::derive(move || user.get().and_then(|res| res.ok().flatten()));

    let expires_in: RwSignal<u64> = RwSignal::new(0);

    Effect::new(move |handle: Option<Option<TimeoutHandle>>| {
        if let Some(prev_handle) = handle.flatten() {
            prev_handle.clear();
        };
        // if expires_in isn't 0, then set a timeout that rerfresh a minute short of the refresh.
        let expires_in: u64 = expires_in.get();
        if let Some(Ok(Some(user))) = auth.get_untracked() {
            if expires_in != 0 {
                let handle = set_timeout_with_handle(
                    move || {
                        refresh_google_token.dispatch(RefreshToken { id: user.id });
                    },
                    std::time::Duration::from_secs(
                        // Google tokens last 3599 seconds, so we'll get a refresh token every 14 seconds.
                        expires_in.checked_sub(3545).unwrap_or_default(),
                    ),
                )
                .unwrap();
                return Some(handle);
            }
        }
        None
    });

    Effect::new(move |_| {
        if let Some(Ok(_expires_in)) = refresh_google_token.value().get() {
            expires_in.set(_expires_in);
        }
    });

    // // Effect to handle refresh token result
    // Effect::new(move |_| {
    //     match refresh_google_token.value().get() {
    //         Some(Ok(_expires_in)) => {
    //             leptos::logging::log!("Token refreshed successfully. New expires_in: {}", _expires_in);
    //             expires_in.set(_expires_in);
    //         },
    //         Some(Err(e)) => {
    //             leptos::logging::log!("Refresh token failed: {:?}", e);
    //             // If refresh fails, log out the user
    //             log_out.dispatch(Logout {});
    //             // Reset expires_in to indicate no active session
    //             expires_in.set(0);
    //         },
    //         _ => {
    //             // Action is pending or no value yet
    //         }
    //     }
    // });
    //
    // // Effect to handle initial auth resource failures
    // Effect::new(move |_| {
    //     if let Some(Err(e)) = auth.get() {
    //         leptos::logging::log!("Auth resource failed to fetch user: {:?}", e);
    //         expires_in.set(0);
    //     }
    // });
    //
    // // NEW: Effect to trigger token refresh and set expires_in when a user reconnects/loads with an existing session
    // Effect::new(move |_| {
    //     if let Some(Ok(Some(user))) = auth.get() {
    //         // If we have a user and expires_in hasn't been set yet (e.g., on initial load/reconnect)
    //         if expires_in.get_untracked() == 0 && !refresh_google_token.pending().get_untracked() {
    //             leptos::logging::log!("Auth resource loaded user, expires_in is 0. Dispatching refresh to get expiration.");
    //             refresh_google_token.dispatch(RefreshToken { id: user.id });
    //         }
    //     }
    // });

    view! {
        <Provider value=AuthContext { log_out, user, google_auth, handle_google_redirect, refresh_google_token, auth, expires_in }>
            {children()}
        </Provider>
    }
}

#[component]
pub fn LogOut(
    #[prop(optional, into)] variant: Signal<ButtonVariants>,
    #[prop(optional, into)] size: Signal<ButtonSizes>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into, default = Signal::from(false))] disabled: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let AuthContext { log_out, .. } = use_context().expect("shouls acces the auth context");

    view! {
        <Button on:click=move|_| { log_out.dispatch(Logout{}); } variant=variant size=size class=class disabled=Signal::derive(move || {
            disabled.get() || log_out.pending().get()
        })>
            {children.map(|children| children())}
        </Button>
    }
}

#[component]
pub fn Google(
    #[prop(optional, into)] variant: Signal<ButtonVariants>,
    #[prop(optional, into)] size: Signal<ButtonSizes>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into, default = Signal::from(false))] disabled: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let AuthContext { google_auth, .. } = use_context().expect("shouls acces the auth context");
    let navigate = use_navigate();
    Effect::new(move |_| {
        if let Some(Ok(redirect)) = google_auth.value().get() {
            navigate(&redirect, Default::default());
        }
    });
    view! {
        <Button on:click=move|_| { google_auth.dispatch(GoogleAuth{}); } variant=variant size=size class=class disabled=Signal::derive(move || {
            disabled.get() || google_auth.pending().get()
        })>
            {children.map(|children| children())}
        </Button>
    }
}

#[derive(Params, Debug, PartialEq, Clone)]
pub struct OAuthParams {
    pub code: Option<String>,
    pub state: Option<String>,
}

#[component]
pub fn HandleGAuth() -> impl IntoView {
    let AuthContext {
        handle_google_redirect,
        expires_in,
        ..
    } = use_context().expect("shouls acces the auth context");

    let query = use_query::<OAuthParams>();
    let navigate = use_navigate();
    Effect::new(move |_| {
        if let Some(Ok(_expires_in)) = handle_google_redirect.value().get() {
            expires_in.set(_expires_in);
            navigate("/", NavigateOptions::default());
        }
    });

    Effect::new(move |_| {
        if let Ok(OAuthParams {
            code: Some(code),
            state: Some(state),
        }) = query.get_untracked()
        {
            leptos::logging::log!("{code:?} {state:?}");
            handle_google_redirect.dispatch(HandleGoogleRedirect {
                provided_csrf: state,
                code,
            });
        } else {
            leptos::logging::log!("error parsing oauth params");
        }
    });
    ().into_view()
}

#[component]
pub fn SignedIn(children: ChildrenFn) -> impl IntoView {
    let AuthContext { auth, .. } = use_context().expect("shouls acces the auth context");
    view! {
        <Transition>
            <Show when=move || auth.get().is_some_and(|res| res.ok().flatten().is_some())>
                {children()}
            </Show>
        </Transition>
    }
}

#[component]
pub fn SignedOut(children: ChildrenFn) -> impl IntoView {
    let AuthContext { auth, .. } = use_context().expect("shouls acces the auth context");
    view! {
        <Transition>
            <Show when=move || auth.get().is_some_and(|res| res.ok().flatten().is_none())>
                {children()}
            </Show>
        </Transition>
    }
}
