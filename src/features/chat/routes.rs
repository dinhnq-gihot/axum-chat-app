use {
    super::handlers::{
        chat,
        get_group_messages,
    },
    axum::{
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
}
