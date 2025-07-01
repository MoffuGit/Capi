use leptos::server;

use common::user::User;
use leptos::prelude::ServerFnError;

#[server]
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    use auth::auth;
    let user = auth().await?.current_user.map(|auth| auth.user().clone());
    Ok(user)
}

#[server]
pub async fn google_auth() -> Result<String, ServerFnError> {
    use auth::clients::GoogleAuth;
    use common::state::pool;
    let pool = pool()?;
    let (authorize_url, csrf_token, pkce_code_verifier) = GoogleAuth::auth_url().map_err(|e| {
        ServerFnError::new(format!("Failed to create Google authentication URL: {e}"))
    })?;

    let url = authorize_url.to_string();
    leptos::logging::log!("{url:?}");
    sqlx::query("INSERT INTO csrf_tokens (csrf_token, pkce_token) VALUES ($1, $2)")
        .bind(csrf_token.secret())
        .bind(pkce_code_verifier.secret())
        .execute(&pool)
        .await
        .map(|_| ())?;

    // Send the url to the client.
    Ok(url)
}

#[server]
pub async fn refresh_token(id: i64) -> Result<u64, ServerFnError> {
    use crate::auth::User;
    use auth::clients::GoogleAuth;
    use auth::clients::TokenResponse;
    use common::state::pool;
    use common::user::ssr::SqlRefreshToken;

    let pool = pool()?;
    let user = User::get(id, &pool)
        .await
        .ok_or(ServerFnError::new(format!(
            "User with email '{id}' not found"
        )))?;

    let refresh_secret = sqlx::query_as::<_, SqlRefreshToken>(
        "SELECT refresh_secret FROM google_tokens WHERE user_id = $1",
    )
    .bind(user.id)
    .fetch_one(&pool)
    .await?;

    let token_response = GoogleAuth::refresh_token(refresh_secret)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to refresh Google token: {e}")))?;

    let access_token = token_response.access_token().secret();
    let expires_in = token_response
        .expires_in()
        .map(|d| d.as_secs())
        .unwrap_or_default();
    let refresh_secret = token_response.refresh_token().map(|t| t.secret()).unwrap();

    // Use ON CONFLICT for upserting google_tokens, removing the need for a separate DELETE
    sqlx::query(
        "INSERT INTO google_tokens (user_id, access_secret, refresh_secret) \
         VALUES ($1, $2, $3) \
         ON CONFLICT (user_id) DO UPDATE SET \
            access_secret = EXCLUDED.access_secret, \
            refresh_secret = EXCLUDED.refresh_secret",
    )
    .bind(user.id)
    .bind(access_token)
    .bind(refresh_secret)
    .execute(&pool)
    .await?;
    Ok(expires_in)
}

#[server]
pub async fn handle_google_redirect(
    provided_csrf: String,
    code: String,
) -> Result<u64, ServerFnError> {
    use crate::auth::User;
    use auth::auth;
    use auth::clients::GoogleAuth;
    use auth::clients::TokenResponse;
    use common::state::pool;
    use common::user::ssr::SqlCsrfToken;

    let pool = pool()?;
    let auth_session = auth().await?;
    // If there's no match we'll return an error.
    let SqlCsrfToken { pkce_token, .. } = sqlx::query_as::<_, SqlCsrfToken>(
        // Assuming 'pkce_code_verifier' is the actual column name in the database
        // If SqlCsrfToken's field is 'pcke_token', you might need #[sqlx(rename = "pkce_code_verifier")]
        "SELECT csrf_token, pkce_token FROM csrf_tokens WHERE csrf_token = $1",
    )
    .bind(provided_csrf)
    .fetch_one(&pool)
    .await
    .map_err(|err| ServerFnError::new(format!("CSRF token verification error: {err:?}")))?;

    let token_response = GoogleAuth::auth(code, pkce_token)
        .await
        .map_err(|e| ServerFnError::new(format!("Google authentication failed: {e}")))?;
    leptos::logging::log!("{:?}", &token_response);
    let access_token = token_response.access_token().secret();
    let expires_in = token_response
        .expires_in()
        .map(|d| d.as_secs())
        .unwrap_or_default();
    let refresh_secret = token_response.refresh_token().map(|t| t.secret()).unwrap();
    let user_info_url = "https://www.googleapis.com/oauth2/v3/userinfo";

    let email = GoogleAuth::client_info(user_info_url.to_string(), access_token.to_string())
        .await
        .map_err(|e| {
            ServerFnError::new(format!("Failed to retrieve user email from Google: {e}"))
        })?;

    let user = if let Some(user) = User::get_from_email(&email, &pool).await {
        user
    } else {
        // Insert new user if not found
        sqlx::query("INSERT INTO users (email) VALUES ($1)")
            .bind(&email)
            .execute(&pool)
            .await?;
        User::get_from_email(&email, &pool).await.unwrap() // This unwrap is safe if insert was successful and email is unique
    };

    // Use ON CONFLICT for upserting google_tokens, removing the need for a separate DELETE
    sqlx::query(
        "INSERT INTO google_tokens (user_id, access_secret, refresh_secret) \
         VALUES ($1, $2, $3) \
         ON CONFLICT (user_id) DO UPDATE SET \
            access_secret = EXCLUDED.access_secret, \
            refresh_secret = EXCLUDED.refresh_secret",
    )
    .bind(user.id)
    .bind(access_token)
    .bind(refresh_secret)
    .execute(&pool)
    .await?;

    auth_session.login_user(user.id);
    auth_session.remember_user(true);

    Ok(expires_in)
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    use auth::auth;

    let auth = auth().await?;
    auth.logout_user();
    leptos_axum::redirect("/");
    Ok(())
}
