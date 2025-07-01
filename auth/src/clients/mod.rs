use anyhow::anyhow;
use dotenv_codegen::dotenv;
pub use oauth2::TokenResponse;
use oauth2::basic::{BasicClient, BasicTokenType};
use oauth2::url::Url;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, StandardTokenResponse, TokenUrl,
    reqwest,
};

use common::user::ssr::SqlRefreshToken;
use serde_json::Value;

pub struct GoogleAuth;

const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";

impl GoogleAuth {
    pub fn auth_url() -> anyhow::Result<(Url, CsrfToken, PkceCodeVerifier)> {
        let client_id = dotenv!("GOOGLE_CLIENT_ID");
        let client_secret = dotenv!("GOOGLE_CLIENT_SECRET");
        let redirect_url = dotenv!("GOOGLE_REDIRECTION_URL");

        let auth_url = AuthUrl::new(GOOGLE_AUTH_URL.to_string())?;
        let token_url = TokenUrl::new(GOOGLE_TOKEN_URL.to_string())?;
        let redirect_url = RedirectUrl::new(redirect_url.to_string())?;

        let client = BasicClient::new(ClientId::new(client_id.to_string()))
            .set_client_secret(ClientSecret::new(client_secret.to_string()))
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(redirect_url);

        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

        let auth_url_builder = client
            .authorize_url(CsrfToken::new_random)
            .set_pkce_challenge(pkce_code_challenge)
            .add_scope(Scope::new("openid".to_string()))
            .add_scope(Scope::new("email".to_string()))
            // required for google auth refresh token to be part of the response.
            .add_extra_param("access_type", "offline")
            .add_extra_param("prompt", "consent");

        let (authorize_url, csrf_state) = auth_url_builder.url();

        Ok((authorize_url, csrf_state, pkce_code_verifier))
    }

    pub async fn refresh_token(
        secret: SqlRefreshToken,
    ) -> anyhow::Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>> {
        let client_id = dotenv!("GOOGLE_CLIENT_ID");
        let client_secret = dotenv!("GOOGLE_CLIENT_SECRET");
        let redirect_url = dotenv!("GOOGLE_REDIRECTION_URL");

        let auth_url = AuthUrl::new(GOOGLE_AUTH_URL.to_string())?;
        let token_url = TokenUrl::new(GOOGLE_TOKEN_URL.to_string())?;
        let redirect_url = RedirectUrl::new(redirect_url.to_string())?;

        let http_client = reqwest::ClientBuilder::new()
            // Following redirects opens the client up to SSRF vulnerabilities.
            .redirect(reqwest::redirect::Policy::none())
            .build()?; // Use ? instead of .expect

        Ok(BasicClient::new(ClientId::new(client_id.to_string()))
            .set_client_secret(ClientSecret::new(client_secret.to_string()))
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(redirect_url)
            .exchange_refresh_token(&oauth2::RefreshToken::new(secret.secret))
            .request_async(&http_client)
            .await?)
    }

    pub async fn auth(
        code: String,
        pkce: String,
    ) -> anyhow::Result<
        StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>,
    > {
        let client_id = dotenv!("GOOGLE_CLIENT_ID");
        let client_secret = dotenv!("GOOGLE_CLIENT_SECRET");
        let redirect_url = dotenv!("GOOGLE_REDIRECTION_URL");

        let auth_url = AuthUrl::new(GOOGLE_AUTH_URL.to_string())?;
        let token_url = TokenUrl::new(GOOGLE_TOKEN_URL.to_string())?;
        let redirect_url = RedirectUrl::new(redirect_url.to_string())?;

        let http_client = reqwest::ClientBuilder::new()
            // Following redirects opens the client up to SSRF vulnerabilities.
            .redirect(reqwest::redirect::Policy::none())
            .build()?; // Use ? instead of .expect
        Ok(BasicClient::new(ClientId::new(client_id.to_string()))
            .set_client_secret(ClientSecret::new(client_secret.to_string()))
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(redirect_url)
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(PkceCodeVerifier::new(pkce))
            .request_async(&http_client)
            .await?)
    }

    pub async fn client_info(url: String, acces_token: String) -> anyhow::Result<String> {
        let client = reqwest::Client::new();
        let response = client.get(url).bearer_auth(acces_token).send().await?;

        if response.status().is_success() {
            let response_json: Value = response.json().await?;
            leptos::logging::log!("{response_json:?}");
            response_json["email"]
                .as_str()
                .map(|s| s.to_string())
                .ok_or_else(|| {
                    anyhow!(
                        "Email not found or not a string in user info response: {:?}",
                        response_json
                    )
                })
        } else {
            Err(anyhow!(
                "Failed to retrieve user info, status: {}",
                response.status()
            ))
        }
    }
}
