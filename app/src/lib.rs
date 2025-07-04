mod components;
mod hooks;
mod routes;
mod sync;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, ProtectedParentRoute, Route, Router, Routes},
    ParamSegment, StaticSegment,
};
use routes::Home;

use self::{
    components::{
        auth::{use_auth, AuthProvider},
        ui::theme::ThemeProvider,
    },
    routes::{GoogleAuth, Login, Servers, SignUp},
    sync::SyncProvider,
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

    view! {
        <Stylesheet id="leptos" href="/pkg/capi.css"/>

        <Title text="Capi"/>

        <ThemeProvider>
            <SyncProvider>
                <AuthProvider>
                    <Router>
                        <main id="app">
                            <Routes fallback=|| "Page not found.".into_view()>
                                <Route path=StaticSegment("") view=Home/>
                                <ParentRoute path=StaticSegment("auth") view=|| view!{<Outlet/>}>
                                    <Route path=StaticSegment("login") view=Login />
                                    <Route path=StaticSegment("signup") view=SignUp />
                                    <Route path=StaticSegment("google")  view=GoogleAuth/>
                                </ParentRoute>
                                <ProtectedParentRoute
                                    condition=move || use_auth().auth.get().map(|res| res.ok().flatten().is_some())
                                    path=StaticSegment("servers")
                                    redirect_path= || "/"
                                    view=Servers
                                >
                                    <Route
                                        path=StaticSegment("")
                                        view=move || view! { <div>"servers"</div> }
                                    />
                                    <Route
                                        path=StaticSegment("discover")
                                        view=move || view! { <div>"discover servers"</div> }
                                    />
                                    <Route
                                        path=StaticSegment("me")
                                        view=move || view! { <div>"Private conversations"</div> }
                                    />
                                    <Route
                                        path=(ParamSegment("server"), ParamSegment("channel"))
                                        view=move || view! { <div>"server channel"</div> }
                                    />
                                </ProtectedParentRoute>
                            </Routes>
                        </main>
                    </Router>
                </AuthProvider>
            </SyncProvider>
        </ThemeProvider>
    }
}
