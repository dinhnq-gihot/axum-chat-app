use {
    crate::{
        enums::routes::RoutePath,
        features::{
            auth::routes::get_routes as get_auth_router,
            chat::routes::get_routes as get_chat_router,
            groups::routes::get_routes as get_group_router,
            users::routes::get_routes as get_user_router,
        },
    },
    axum::{
        routing::get,
        Router,
    },
};

async fn root() -> &'static str {
    "Hello Rustacean 🦀"
}

pub fn create_router() -> Router {
    let auth_routes = get_auth_router();
    let user_routes = get_user_router();
    let group_routes = get_group_router();
    let chat_routes = get_chat_router();

    let api_routes = Router::new()
        .nest(RoutePath::AUTH.get_path(), auth_routes)
        .nest(RoutePath::USERS.get_path(), user_routes)
        .nest(RoutePath::GROUPS.get_path(), group_routes)
        .nest(RoutePath::CHAT.get_path(), chat_routes);

    Router::new().route("/", get(root)).nest("/api", api_routes)
}
