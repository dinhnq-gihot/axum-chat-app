use {
    super::handlers::{
        chat,
        get_group_messages,
    },
    crate::features::auth::middleware::check_jwt,
    axum::{
        middleware,
        routing::{
            get,
            post,
        },
        Router,
    },
};

pub fn get_routes() -> Router {
    Router::new()
        .route("/", post(chat))
        .route("/:group_id", get(get_group_messages))
        .layer(middleware::from_fn(check_jwt))
}
