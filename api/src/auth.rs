#[cfg(feature = "ssr")]
use auth::auth;
use leptos::server;

use common::user::User;
use leptos::prelude::ServerFnError;

#[server]
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    Ok(auth().await?.current_user.map(|auth| auth.user().clone()))
}
