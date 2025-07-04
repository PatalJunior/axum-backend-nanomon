use axum::extract::State;
use axum::Json;

use uuid::Uuid;

use crate::domain::models::user::{UserError, UserModel};
use crate::handlers::users::{PatchUserRequest, UserResponse};
use crate::infra::errors::InfraError;
use crate::infra::repositories::user_repository;
use crate::utils::{JsonExtractor, PathExtractor};
use crate::AppState;
use crate::infra::repositories::user_repository::UpdateUserDb;


use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher,
        SaltString
    },
    Argon2
};


pub async fn patch_user(
    State(state): State<AppState>,
    PathExtractor(user_id): PathExtractor<Uuid>,
    JsonExtractor(patch_user): JsonExtractor<PatchUserRequest>,
) -> Result<Json<UserResponse>, UserError> {
    let mut user = user_repository::get(&state.pool, user_id)
        .await
        .map_err(|db_error| match db_error {
            InfraError::InternalServerError => UserError::InternalServerError,
            InfraError::NotFound => UserError::NotFound(user_id),
        })?;



    if let Some(username) = patch_user.username {
        user.username = username
    }

    if let Some(email) = patch_user.email {
        user.email = email
    }

    if let Some(password) = patch_user.password {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hashed_password = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
        user.password_hash = hashed_password;
    }
    let update_user = adapt_user_to_user_patch(user);
    let updated_user = user_repository::update(&state.pool, user_id, update_user)
        .await
        .map_err(|_| UserError::InternalServerError)?;

    Ok(Json(adapt_user_to_user_response(updated_user)))
}

fn adapt_user_to_user_response(user: UserModel) -> UserResponse {
    UserResponse {
        id: user.id,
        email: user.email,
        username: user.username,
        created_at: user.created_at,
    }
}

fn adapt_user_to_user_patch(user: UserModel) -> UpdateUserDb {
    UpdateUserDb {
        email: user.email,
        username: user.username,
        password_hash: user.password_hash,
        is_admin: user.is_admin,
    }
}
