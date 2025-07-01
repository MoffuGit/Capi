mod components;
mod hooks;
mod routes;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    ParamSegment, StaticSegment,
};
use routes::Home;

use self::{
    components::{auth::AuthProvider, ui::theme::ThemeProvider},
    routes::{Auth, GoogleAuth, Servers},
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
            <AuthProvider>
                <Router>
                    <main>
                        <Routes fallback=|| "Page not found.".into_view()>
                            <Route path=StaticSegment("") view=Home/>
                            <ParentRoute path=StaticSegment("auth") view=|| view!{<Outlet/>}>
                                <Route path=StaticSegment("") view=Auth />
                                <Route path=StaticSegment("google")  view=GoogleAuth/>
                            </ParentRoute>
                            <ParentRoute
                                path=StaticSegment("servers")
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
                            </ParentRoute>
                        </Routes>
                    </main>
                </Router>
            </AuthProvider>
        </ThemeProvider>
    }
}
