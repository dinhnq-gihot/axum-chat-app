use {
    super::handlers::{
        create_user,
        delete_user,
        get_all_user,
        get_user_by_id,
        update_user,
    },
    axum::{
        routing::get,
        Router,
    },
};

pub fn get_routes() -> Router {
    Router::new()
        .route("/", get(get_all_user).post(create_user))
        .route(
            "/:id",
            get(get_user_by_id).delete(delete_user).patch(update_user),
        )
}
