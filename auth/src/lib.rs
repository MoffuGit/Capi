pub mod clients;

pub use axum_session_auth::{Authentication, HasPermission};
use axum_session_sqlx::SessionPgPool;
use common::user::User;
use leptos::prelude::ServerFnError;
pub use sqlx::PgPool;
pub type AuthSession = axum_session_auth::AuthSession<AuthUser, i64, SessionPgPool, PgPool>;

pub async fn auth() -> Result<AuthSession, ServerFnError> {
    let auth: AuthSession = leptos_axum::extract().await?;
    Ok(auth)
}

use async_trait::async_trait;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthUser(User);

impl AuthUser {
    pub fn user(&self) -> &User {
        &self.0
    }
}

#[async_trait]
impl Authentication<AuthUser, i64, PgPool> for AuthUser {
    async fn load_user(userid: i64, pool: Option<&PgPool>) -> Result<AuthUser, anyhow::Error> {
        let pool = pool.unwrap();

        let user = User::get(userid, pool)
            .await
            .ok_or_else(|| anyhow::anyhow!("Cannot get user for userid: {}", userid))?;
        Ok(AuthUser(user))
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn is_active(&self) -> bool {
        true
    }

    fn is_anonymous(&self) -> bool {
        false
    }
}

#[async_trait]
impl HasPermission<PgPool> for AuthUser {
    async fn has(&self, perm: &str, _pool: &Option<&PgPool>) -> bool {
        self.0.permissions.contains(perm)
    }
}
