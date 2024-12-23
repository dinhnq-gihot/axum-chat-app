use {
    super::{
        dto::{
            CreateUserRequest,
            UpdateUserRequest,
            UserResponse,
        },
        model::{
            NewUser,
            User,
        },
    },
    crate::{
        database::Database,
        schema::users,
    },
    axum::{
        extract::Path,
        http::StatusCode,
        response::IntoResponse,
        Extension,
        Json,
    },
    diesel::{
        delete,
        insert_into,
        prelude::*,
        update,
    },
    diesel_async::RunQueryDsl,
    serde_json::json,
    std::sync::Arc,
    uuid::Uuid,
};

pub async fn create_user(
    Extension(db): Extension<Arc<Database>>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let mut conn = db.get_connection().await;

    let _ = insert_into(users::table)
        .values(NewUser {
            name: &payload.username,
            email: &payload.email,
            password: &payload.password,
            avatar: payload.avatar.as_deref(),
        })
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await
        .unwrap();

    (
        StatusCode::OK,
        Json(json!({"result": "create user successfully"})),
    )
}

pub async fn get_user_by_id(
    Extension(db): Extension<Arc<Database>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let mut conn = db.get_connection().await;
    let user = users::table
        .find(id)
        .select(User::as_select())
        .first(&mut conn)
        .await
        .unwrap();

    (
        StatusCode::OK,
        Json(json!({"result": UserResponse::from(user)})),
    )
}

pub async fn get_all_user(Extension(db): Extension<Arc<Database>>) -> impl IntoResponse {
    let mut conn = db.get_connection().await;
    let users: Vec<User> = users::table
        .select(User::as_select())
        .load(&mut conn)
        .await
        .unwrap();
    let user_responses: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();

    (StatusCode::OK, Json(user_responses))
}

pub async fn update_user(
    Extension(db): Extension<Arc<Database>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> impl IntoResponse {
    let UpdateUserRequest {
        name,
        email,
        avatar,
    } = payload;

    let mut conn = db.get_connection().await;
    let mut existed_user: User = users::table
        .find(id)
        .select(User::as_select())
        .first(&mut conn)
        .await
        .unwrap();

    if name.is_some() {
        existed_user.name = name.unwrap();
    }
    if email.is_some() {
        existed_user.email = email.unwrap();
    }
    if avatar.is_some() {
        existed_user.avatar = avatar;
    }

    update(users::table.filter(users::id.eq(id)))
        .set(existed_user)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await
        .unwrap();

    (
        StatusCode::ACCEPTED,
        Json(json!(
            {
                "result": "User updated successfully"
            }
        )),
    )
}

pub async fn delete_user(
    Extension(db): Extension<Arc<Database>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let mut conn = db.get_connection().await;
    delete(users::table.filter(users::id.eq(id)))
        .execute(&mut conn)
        .await
        .unwrap();

    (
        StatusCode::ACCEPTED,
        Json(json!(
            {
                "result": "User deleted successfully"
            }
        )),
    )
}
