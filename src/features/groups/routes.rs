use {
    super::handlers::{
        create_group,
        get_all_groups_of_user,
        get_group_by_id,
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
        .route("/", post(create_group))
        .route("/user/:id", get(get_all_groups_of_user))
        .layer(middleware::from_fn(check_jwt))
        .route("/:id", get(get_group_by_id))
}
