use {
    super::handlers::{
        create_group,
        get_group_by_id,
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
        .route("/", post(create_group))
        .route("/:id", get(get_group_by_id))
}
