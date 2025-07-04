use std::net::SocketAddr;
use chrono::{Utc, Duration};
use axum::extract::{ConnectInfo, State};
use axum::Json;
use headers::UserAgent;
use axum_extra::TypedHeader;
use sha2::{Sha256, Digest};
use crate::domain::models::token::TokenError;
use crate::handlers::tokens::{CreatTokenRequest, TokenResponse};
use crate::infra::repositories::token_repository;
use crate::state::AppState;
use crate::utils::JsonExtractor;

fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}


pub async fn create_token(
    State(state): State<AppState>,
    JsonExtractor(new_token): JsonExtractor<CreatTokenRequest>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    TypedHeader(user_agent): TypedHeader<UserAgent>
) -> Result<Json<TokenResponse>, TokenError> {


    let new_token_db = token_repository::NewTokenDb {
        token_hash: hash_token(new_token.token.as_str()),
        created_at: Utc::now(),
        expires_at: Utc::now() + Duration::hours(1),
        ip_address: addr.ip().to_string(),
        user_agent: user_agent.to_string(),
    };

    let created_token = token_repository::insert(&state.pool, new_token_db)
        .await
        .map_err(TokenError::InfraError)?;

    let token_response = TokenResponse {
        id: created_token.id,
        user_id: created_token.user_id,
        token_hash: created_token.token_hash,
        expires_at: created_token.expires_at,
    };

    Ok(Json(token_response))
}
