mod components;
mod routes;

use capi_primitives::common::floating_tree::provide_floating_tree;
use convex_client::leptos::ConvexProvider;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, ProtectedParentRoute, Route, Router, Routes},
    ParamSegment, StaticSegment,
};
use routes::Landing;

use self::{
    components::{
        auth::{use_auth, AuthProvider},
        ui::theme::ThemeProvider,
    },
    routes::{
        server::{channel::Channel, Server},
        servers::Servers,
        GoogleAuth, Home, Login, SignUp,
    },
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
                <link rel="preconnect" href="https://fonts.googleapis.com" />
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
                <link
                    href="https://fonts.googleapis.com/css2?family=Geist:wght@100..900&display=swap"
                    rel="stylesheet"
                />
                <link href="https://fonts.googleapis.com/css2?family=Inter:ital,opsz,wght@0,14..32,100..900;1,14..32,100..900&display=swap" rel="stylesheet"/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_floating_tree();

    view! {
        <Stylesheet id="leptos" href="/pkg/capi.css"/>

        <Title text="Capi"/>

        <ThemeProvider>
            <AuthProvider>
                <ConvexProvider>
                    <Router>
                        <main id="app">
                            <Routes fallback=|| "Page not found.".into_any()>
                                <Route path=StaticSegment("") view=Landing/>
                                <ParentRoute path=StaticSegment("auth") view=|| view!{<Outlet/>}>
                                    <Route path=StaticSegment("login") view=Login />
                                    <Route path=StaticSegment("signup") view=SignUp />
                                    <Route path=StaticSegment("google")  view=GoogleAuth/>
                                </ParentRoute>
                                <ProtectedParentRoute
                                    condition=move || use_auth().auth().get().and_then(|res| res.ok()).map(|res| res.is_some())
                                    path=StaticSegment("servers")
                                    redirect_path= || "/"
                                    view=Home
                                >
                                    <Route
                                        path=StaticSegment("")
                                        view=Servers
                                    />
                                    <Route
                                        path=StaticSegment("me")
                                        view=move || view!{<div>"private"</div>}
                                    />
                                    <ParentRoute
                                        path=ParamSegment("server")
                                        view=Empty
                                    >
                                        <Route
                                            path=StaticSegment("")
                                            view=Server
                                        />
                                        <Route
                                            path=ParamSegment("channel")
                                            view=Channel
                                        />
                                    </ParentRoute>
                                </ProtectedParentRoute>
                            </Routes>
                        </main>
                    </Router>
                </ConvexProvider>
            </AuthProvider>
        </ThemeProvider>
    }
}

#[component]
pub fn Empty() -> impl IntoView {
    view! {
        <Outlet/>
    }
}

//TOOD:
//  download for attachments
//  add animations for icons
//  add embeds
//  add pinned messages
//  add private conversations and friends
//  improve the markdown
//  add mentions for users, roles and everyone
//  add notifications
//  add the toasts
//  add inbox
//  add live calls
