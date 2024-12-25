use {
    super::handlers::{
        login,
        register,
    },
    axum::{
        routing::post,
        Router,
    },
};

pub fn get_routes() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}
