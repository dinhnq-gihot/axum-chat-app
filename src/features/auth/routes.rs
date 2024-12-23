use {
    super::handlers::{
        login,
        verify,
    },
    axum::{
        routing::post,
        Router,
    },
};

pub fn get_routes() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/verify", post(verify))
}
