use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum::routing::patch;

// Import handlers for user-related operations
use crate::handlers::users::{create_user, get_user, list_users, patch_user, login_user};


// Import the application state
use crate::AppState;

// Function to create the main application router
pub fn app_router(state: AppState) -> Router {
    Router::new()
        // Define the root route
        .route("/", get(root))
        .nest("/v1/users", users_routes(state.clone()))
        // Define a fallback handler for 404 errors
        .fallback(handler_404)
        // Attach the application state to the router
        .with_state(state)
}

// Handler for the root route
async fn root() -> &'static str {
    "Server is running!"
}

// Handler for 404 Not Found errors
async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}


// Function to define user-related routes
fn users_routes(state: AppState) -> Router<AppState> {
    Router::new()
        // Route for creating a new user (POST /v1/posts)
        .route("/", post(create_user))
        // Route for listing all users (GET /v1/posts)
        .route("/", get(list_users))
        // Route for getting a specific user by ID (GET /v1/posts/:id)
        .route("/{id}", get(get_user))
        // Route for patching a specific user by ID (PATCH /v1/posts/:id)
        .route("/{id}", patch(patch_user))
        // Route for handling login requests (POST /v1/posts/login)
        .route("/login", post(login_user))
        // Attach the application state to the user's router
        .with_state(state)
}
