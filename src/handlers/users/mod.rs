use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use create_user::create_user;
pub use get_user::get_user;
pub use list_users::list_users;
pub use patch_user::patch_user;
pub use login_user::login_user;


mod create_user;
mod get_user;
mod list_users;

mod patch_user;

mod login_user;

#[derive(Debug, Deserialize)]
pub struct CreatUserRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct PatchUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    id: Uuid,
    username: String,
    email: String,
    created_at: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListUsersResponse {
    users: Vec<UserResponse>,
}
