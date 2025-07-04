use axum::extract::State;
use axum::Json;

use crate::domain::models::user::UserError;
use crate::handlers::users::{CreatUserRequest, UserResponse};
use crate::infra::repositories::user_repository;
use crate::utils::JsonExtractor;
use crate::AppState;

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, 
        SaltString
    },
    Argon2
};



pub async fn create_user(
    State(state): State<AppState>,
    JsonExtractor(new_user): JsonExtractor<CreatUserRequest>,
) -> Result<Json<UserResponse>, UserError> {

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hashed_password = argon2.hash_password(new_user.password.as_bytes(), &salt)?.to_string();

    let new_user_db = user_repository::NewUserDb {
        email: new_user.email,
        username: new_user.username,
        password_hash: hashed_password,
        is_admin: false,
    };

    let created_user = user_repository::insert(&state.pool, new_user_db)
        .await
        .map_err(UserError::InfraError)?;

    let user_response = UserResponse {
        id: created_user.id,
        username: created_user.username,
        email: created_user.email,
        created_at: created_user.created_at,
    };

    Ok(Json(user_response))
}
