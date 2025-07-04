use axum::extract::{State};
use axum::Json;

use crate::domain::models::user::{UserError, UserModel};
use crate::handlers::users::{ListUsersResponse, UserResponse};
use crate::infra::repositories::user_repository::{get_all, UsersFilter};
use crate::AppState;
use crate::utils::JsonExtractor;

pub async fn list_users(
    State(state): State<AppState>,
    JsonExtractor(params): JsonExtractor<UsersFilter>,
) -> Result<Json<ListUsersResponse>, UserError> {
    let users = get_all(&state.pool, params)
        .await
        .map_err(|_| UserError::InternalServerError)?;

    Ok(Json(adapt_users_to_list_users_response(users)))
}

fn adapt_user_to_user_response(user: UserModel) -> UserResponse {
    UserResponse {
        id: user.id,
        email: user.email,
        username: user.username,
        created_at: user.created_at
    }
}

fn adapt_users_to_list_users_response(users: Vec<UserModel>) -> ListUsersResponse {
    let users_response: Vec<UserResponse> =
        users.into_iter().map(adapt_user_to_user_response).collect();

    ListUsersResponse {
        users: users_response,
    }
}
