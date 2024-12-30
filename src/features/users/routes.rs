use {
    super::handlers::{
        create_user,
        delete_user,
        get_all_user,
        get_user_by_id,
        update_avatar,
        update_user,
    },
    crate::features::auth::middleware::check_jwt,
    axum::{
        extract::DefaultBodyLimit,
        middleware,
        routing::{
            get,
            patch,
        },
        Router,
    },
    tower_http::limit::RequestBodyLimitLayer,
};

pub fn get_routes() -> Router {
    let router1 = Router::new()
        .route("/", get(get_all_user).post(create_user))
        .route(
            "/:id",
            get(get_user_by_id).delete(delete_user).patch(update_user),
        )
        .layer(middleware::from_fn(check_jwt));
    let router2 = Router::new()
        .route("/avatar/:id", patch(update_avatar))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024));

    Router::new().merge(router1).merge(router2)
}
