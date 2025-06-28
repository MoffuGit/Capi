mod api;
mod components;
mod hooks;
mod routes;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    StaticSegment,
};
use routes::Home;

use self::{components::ui::theme::ThemeProvider, routes::Servers};

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
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/capi.css"/>

        <Title text="Welcome to Leptos"/>

        <ThemeProvider>
            <Router>
                <main>
                    <Routes fallback=|| "Page not found.".into_view()>
                        <Route path=StaticSegment("") view=Home/>
                        <ParentRoute
                            path=StaticSegment("servers")
                            view=Servers
                        >
                            <Route
                                path=StaticSegment("")
                                view=move || view! { <div>"list of servers"</div> }
                            />
                        </ParentRoute>
                    </Routes>
                </main>
            </Router>
        </ThemeProvider>
    }
}
