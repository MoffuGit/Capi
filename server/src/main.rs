use app::*;
use auth::AuthUser;
use axum::Router;
use axum_session::{SessionConfig, SessionLayer, SessionStore};
use axum_session_auth::{AuthConfig, AuthSessionLayer};
use axum_session_sqlx::SessionPgPool;
use common::state::AppState;
use convex::ConvexClient;
use dotenv::dotenv;
use dotenv_codegen::dotenv;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use uploadthing::server::UploadThing;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let neon_url = dotenv!("NEON_URL");
    let convex_url = dotenv!("CONVEX_URL");

    let pool = PgPoolOptions::new()
        .connect(neon_url)
        .await
        .expect("should make a PG pool.");

    let convex_client = ConvexClient::new(convex_url)
        .await
        .expect("should make a Convec client");

    let session_config = SessionConfig::default().with_table_name("axum_sessions");
    let auth_config = AuthConfig::<i64>::default();
    let session_store =
        SessionStore::<SessionPgPool>::new(Some(SessionPgPool::from(pool.clone())), session_config)
            .await
            .unwrap();

    if let Err(e) = sqlx::migrate!().run(&pool).await {
        eprintln!("{e:?}");
    }

    let uploadthing = UploadThing::default();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app_state = AppState {
        leptos_options,
        pool: pool.clone(),
        routes: routes.clone(),
        convex: convex_client,
        uploadthing,
    };

    let app = Router::new()
        .leptos_routes(&app_state, routes, {
            let options = app_state.leptos_options.clone();
            move || shell(options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .layer(
            AuthSessionLayer::<AuthUser, i64, SessionPgPool, PgPool>::new(Some(pool.clone()))
                .with_config(auth_config),
        )
        .layer(SessionLayer::new(session_store))
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
