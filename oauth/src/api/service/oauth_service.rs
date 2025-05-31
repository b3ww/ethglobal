use oauth2::{AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet, PkceCodeChallenge, RedirectUrl, Scope, StandardRevocableToken, TokenResponse, TokenUrl};
use oauth2::basic::{BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse};
use reqwest::Client as HttpClient;
use serde_json::Value;
use std::env;
use thiserror::Error;

use crate::api::models::user::{GithubUser, NewUser, UpdateUser, User};
use crate::repository::{DbPool, UserRepository};
use crate::api::service::UserServiceError;

#[derive(Debug, Error)]
pub enum OAuthServiceError {
    #[error("OAuth configuration error: {0}")]
    ConfigError(String),
    #[error("Token exchange error: {0}")]
    TokenExchangeError(String),
    #[error("API request error: {0}")]
    ApiRequestError(String),
    #[error("User creation error: {0}")]
    UserCreationError(#[from] UserServiceError),
    #[error("Database error: {0}")]
    DatabaseError(#[from] diesel::result::Error),
}

pub struct OAuthService;

type MyBasicClient =  Client<BasicErrorResponse, BasicTokenResponse, BasicTokenIntrospectionResponse, StandardRevocableToken, BasicRevocationErrorResponse, EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>;

impl OAuthService {
    pub fn create_github_client() -> Result<MyBasicClient, OAuthServiceError> {
        let github_client_id = env::var("GITHUB_CLIENT_ID")
            .map_err(|_| OAuthServiceError::ConfigError("GITHUB_CLIENT_ID not set".to_string()))?;

        let github_client_secret = env::var("GITHUB_CLIENT_SECRET")
            .map_err(|_| OAuthServiceError::ConfigError("GITHUB_CLIENT_SECRET not set".to_string()))?;

        let redirect_url = env::var("OAUTH_REDIRECT_URL")
            .map_err(|_| OAuthServiceError::ConfigError("OAUTH_REDIRECT_URL not set".to_string()))?;

        let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
            .map_err(|e| OAuthServiceError::ConfigError(e.to_string()))?;

        let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
            .map_err(|e| OAuthServiceError::ConfigError(e.to_string()))?;

        let redirect_url = RedirectUrl::new(redirect_url)
            .map_err(|e| OAuthServiceError::ConfigError(e.to_string()))?;

        let client = BasicClient::new(ClientId::new(github_client_id))
            .set_client_secret(ClientSecret::new(github_client_secret))
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(redirect_url);

        Ok(client)
    }

    pub fn get_authorization_url() -> Result<(String, CsrfToken), OAuthServiceError> {
        let client = Self::create_github_client()?;

        let (pkce_challenge, _pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("user:email".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        Ok((auth_url.to_string(), csrf_token))
    }

    pub async fn exchange_code_for_token(
        code: String,
        _csrf_token: String,
    ) -> Result<String, OAuthServiceError> {
        let client = Self::create_github_client()?;

        let http_client = reqwest::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");

        let token_result = client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(&http_client)
            .await
            .map_err(|e| OAuthServiceError::TokenExchangeError(e.to_string()))?;

        Ok(token_result.access_token().secret().clone())
    }

    pub async fn get_github_user(access_token: &str) -> Result<GithubUser, OAuthServiceError> {
        let client = HttpClient::new();

        let response = client
            .get("https://api.github.com/user")
            .header("Authorization", format!("Bearer {}", access_token))
            .header("User-Agent", "ethglobal-app")
            .send()
            .await
            .map_err(|e| OAuthServiceError::ApiRequestError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(OAuthServiceError::ApiRequestError(format!(
                "GitHub API returned status: {}",
                response.status()
            )));
        }

        let user_data: Value = response
            .json()
            .await
            .map_err(|e| OAuthServiceError::ApiRequestError(e.to_string()))?;

        let github_id = user_data["id"]
            .as_i64()
            .ok_or_else(|| OAuthServiceError::ApiRequestError("Missing id field".to_string()))?;

        let login = user_data["login"]
            .as_str()
            .ok_or_else(|| OAuthServiceError::ApiRequestError("Missing login field".to_string()))?
            .to_string();

        let avatar_url = user_data["avatar_url"].as_str().map(|s| s.to_string());
        let email = user_data["email"].as_str().map(|s| s.to_string());
        let name = user_data["name"].as_str().map(|s| s.to_string());

        Ok(GithubUser {
            id: github_id,
            login,
            avatar_url,
            email,
            name,
        })
    }

    pub async fn authenticate_github_user(
        access_token: String,
        pool: &DbPool,
    ) -> Result<User, OAuthServiceError> {
        let github_user = Self::get_github_user(&access_token).await?;

        let mut conn = pool.get().expect("Failed to get connection from pool");
        use diesel::prelude::*;
        use crate::api::models::schema::users;

        let existing_user = users::table
            .filter(users::github_id.eq(github_user.id))
            .select(User::as_select())
            .first(&mut conn)
            .optional()
            .map_err(OAuthServiceError::DatabaseError)?;

        if let Some(user) = existing_user {
            if user.access_token.as_deref() != Some(&access_token) {
                let updated_user = UserRepository::update(
                    user.id,
                    UpdateUser {
                        username: None,
                        email: None,
                        github_id: None,
                        github_username: None,
                        avatar_url: github_user.avatar_url.clone(),
                        access_token: Some(access_token),
                    },
                    pool,
                )
                .map_err(OAuthServiceError::DatabaseError)?;

                return Ok(updated_user);
            }

            return Ok(user);
        }

        // Create new user
        let username = github_user.login.clone();
        let email = github_user.email.unwrap_or_else(|| format!("{}@github.com", github_user.login));

        let new_user = NewUser {
            username,
            email,
            github_id: Some(github_user.id),
            github_username: Some(github_user.login),
            avatar_url: github_user.avatar_url,
            access_token: Some(access_token),
        };

        let user = UserRepository::create(new_user, pool)
            .map_err(OAuthServiceError::DatabaseError)?;

        Ok(user)
    }
}
