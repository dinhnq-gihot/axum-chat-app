use {
    crate::{
        enums::routes::RoutePath,
        features::{
            auth::routes::get_routes as get_auth_router,
            users::routes::get_routes as get_user_router,
        },
    },
    axum::Router,
};

pub fn create_router() -> Router {
    let auth_routes = get_auth_router();
    let user_routes = get_user_router();

    let api_routes = Router::new()
        .nest(RoutePath::AUTH.get_path(), auth_routes)
        .nest(RoutePath::USERS.get_path(), user_routes);

    Router::new().nest("/api", api_routes)
}
