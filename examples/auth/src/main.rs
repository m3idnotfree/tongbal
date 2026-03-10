use std::{collections::HashMap, env, str::FromStr, sync::Arc};

use anyhow::{Context, Result};
use axum::{
    Router,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use time::Duration;
use tokio::{net::TcpListener, sync::RwLock};
use tongbal_oauth::{AuthCallback, ClientId, ClientSecret, RedirectUrl, Token, TongbalOauth};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

const CSRF_COOKIE: &str = "csrf_state";
const SESSION_COOKIE: &str = "session_id";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let config = Config::from_env().context("Failed to load config")?;

    info!("Starting Tongbal OAuth server");

    let oauth = TongbalOauth::new(config.client_id, config.client_secret, config.redirect_uri);

    info!("Login  : http://localhost:{}/auth/chzzk", config.port);
    info!("Logout : http://localhost:{}/auth/logout", config.port);
    info!("Refresh: http://localhost:{}/auth/refresh", config.port);

    let state = AppState {
        oauth,
        tokens: Arc::new(RwLock::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/auth/chzzk", get(login))
        .route("/auth/callback", get(auth_callback))
        .route("/auth/refresh", get(refresh))
        .route("/auth/logout", get(logout))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", config.port);
    let listener = TcpListener::bind(addr)
        .await
        .with_context(|| format!("Failed to bind to port {}", config.port))?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn login(State(state): State<AppState>, jar: CookieJar) -> impl IntoResponse {
    use axum_extra::extract::cookie::SameSite;

    let (auth_url, csrf_state) = state.oauth.auth_url_with_state();
    info!("Generated OAuth URL");

    let cookie = Cookie::build((CSRF_COOKIE, csrf_state.to_string()))
        .same_site(SameSite::Lax)
        .http_only(true)
        .secure(true)
        .path("/")
        .max_age(Duration::minutes(30));

    (jar.add(cookie), Redirect::to(auth_url.as_str()))
}

async fn auth_callback(
    State(state): State<AppState>,
    jar: CookieJar,
    Query(callback): Query<AuthCallback>,
) -> impl IntoResponse {
    use axum_extra::extract::cookie::SameSite;

    let csrf_cookie = match jar.get(CSRF_COOKIE) {
        Some(cookie) => cookie.value().to_string(),
        None => {
            error!("Missing CSRF cookie");
            return (StatusCode::BAD_REQUEST, "missing CSRF cookie").into_response();
        }
    };

    if csrf_cookie != callback.state {
        error!("CSRF token mismatch");
        return (StatusCode::BAD_REQUEST, "CSRF token mismatch").into_response();
    }

    match state
        .oauth
        .exchange_code(callback.code, callback.state)
        .await
    {
        Ok(resp) => {
            info!("Successfully got access token");

            let session_id = Uuid::new_v4().to_string();
            state
                .tokens
                .write()
                .await
                .insert(session_id.clone(), resp.content);

            let jar = jar.remove(Cookie::from(CSRF_COOKIE)).add(
                Cookie::build((SESSION_COOKIE, session_id))
                    .same_site(SameSite::Lax)
                    .http_only(true)
                    .path("/"),
            );

            (jar, (StatusCode::OK, "Successfully authenticated")).into_response()
        }
        Err(e) => {
            error!("Failed to exchange authorization code for access token: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

async fn refresh(State(state): State<AppState>, jar: CookieJar) -> impl IntoResponse {
    let session_id = match jar.get(SESSION_COOKIE) {
        Some(cookie) => cookie.value().to_string(),
        None => {
            error!("Missing session cookie on refresh");
            return (StatusCode::UNAUTHORIZED, "missing session cookie").into_response();
        }
    };

    let refresh_token = match state.tokens.read().await.get(&session_id) {
        Some(token) => token.refresh_token.clone(),
        None => {
            error!("Session not found on refresh");
            return (StatusCode::UNAUTHORIZED, "session not found").into_response();
        }
    };

    match state.oauth.refresh_token(refresh_token).await {
        Ok(resp) => {
            info!("Successfully refreshed token");
            state.tokens.write().await.insert(session_id, resp.content);
            (StatusCode::OK, "Token refreshed").into_response()
        }
        Err(e) => {
            error!("Failed to refresh token: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

async fn logout(State(state): State<AppState>, jar: CookieJar) -> impl IntoResponse {
    let session_id = match jar.get(SESSION_COOKIE) {
        Some(cookie) => cookie.value().to_string(),
        None => {
            error!("Missing session cookie on logout");
            return (StatusCode::UNAUTHORIZED, "missing session cookie").into_response();
        }
    };

    let access_token = match state.tokens.write().await.remove(&session_id) {
        Some(token) => token.access_token,
        None => {
            error!("Session not found on logout");
            return (StatusCode::UNAUTHORIZED, "session not found").into_response();
        }
    };

    match state.oauth.revoke_token(access_token).await {
        Ok(_) => {
            info!("Successfully revoked token");
            let jar = jar.remove(Cookie::from(SESSION_COOKIE));
            (jar, (StatusCode::OK, "Logged out")).into_response()
        }
        Err(e) => {
            error!("Failed to revoke token: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

type TokenStore = Arc<RwLock<HashMap<String, Token>>>;

#[derive(Clone)]
struct AppState {
    oauth: TongbalOauth,
    tokens: TokenStore,
}

struct Config {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: RedirectUrl,
    port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv()?;
        let client_id: ClientId = env::var("CLIENT_ID")
            .context("CLIENT_ID environment variable not set")?
            .into();
        let client_secret: ClientSecret = env::var("CLIENT_SECRET")
            .context("CLIENT_SECRET environment variable not set")?
            .into();
        let redirect_uri = RedirectUrl::from_str(
            &env::var("REDIRECT_URI").context("REDIRECT_URI environment variable not set")?,
        )?;
        let port: u16 = env::var("PORT")
            .context("PORT environment variable not set")?
            .parse()?;

        Ok(Self {
            client_id,
            client_secret,
            redirect_uri,
            port,
        })
    }
}
