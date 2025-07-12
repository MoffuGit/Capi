use axum::extract::FromRef;
use convex::ConvexClient;
use leptos::prelude::{LeptosOptions, ServerFnError, use_context};
use leptos_axum::AxumRouteListing;
use sqlx::PgPool;

use uploadthing::server::UploadThing;
/// This takes advantage of Axum's SubStates feature by deriving FromRef. This is the only way to have more than one
/// item in Axum's State. Leptos requires you to have leptosOptions in your State struct for the leptos route handlers
#[derive(FromRef, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: PgPool,
    pub convex: ConvexClient,
    pub routes: Vec<AxumRouteListing>,
    pub uploadthing: UploadThing,
}

pub fn uploadthing() -> Result<UploadThing, ServerFnError> {
    Ok(use_context::<AppState>()
        .ok_or_else(|| ServerFnError::new("Pool missing."))?
        .uploadthing)
}

pub fn pool() -> Result<PgPool, ServerFnError> {
    Ok(use_context::<AppState>()
        .ok_or_else(|| ServerFnError::new("Pool missing."))?
        .pool)
}

pub fn convex() -> Result<ConvexClient, ServerFnError> {
    Ok(use_context::<AppState>()
        .ok_or_else(|| ServerFnError::new("Pool missing."))?
        .convex)
}
