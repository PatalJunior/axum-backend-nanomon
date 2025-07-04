use axum::extract::State;
use axum::Json;
use uuid::Uuid;

use crate::domain::models::user::{UserError, UserModel};
use crate::handlers::users::UserResponse;
use crate::infra::errors::InfraError;
use crate::infra::repositories::user_repository;
use crate::utils::PathExtractor;
use crate::AppState;

pub async fn get_user(
    State(state): State<AppState>,
    PathExtractor(post_id): PathExtractor<Uuid>,
) -> Result<Json<UserResponse>, UserError> {
    let user =
        user_repository::get(&state.pool, post_id)
            .await
            .map_err(|db_error| match db_error {
                InfraError::InternalServerError => UserError::InternalServerError,
                InfraError::NotFound => UserError::NotFound(post_id),
            })?;

    Ok(Json(adapt_user_to_user_response(user)))
}

fn adapt_user_to_user_response(user: UserModel) -> UserResponse {
    UserResponse {
        id: user.id,
        email: user.email,
        username: user.username,
        created_at: user.created_at,
    }
}
