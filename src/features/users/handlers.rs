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
        enums::{
            errors::{
                Error,
                Result,
            },
            types::{
                DataResponse,
                GenericResponse,
            },
        },
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
    std::sync::Arc,
    uuid::Uuid,
};

pub async fn create_user(
    Extension(db): Extension<Arc<Database>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse> {
    let mut conn = db.get_connection().await;

    insert_into(users::table)
        .values(NewUser {
            name: &payload.username,
            email: &payload.email,
            password: &payload.password,
            avatar: payload.avatar.as_deref(),
        })
        .execute(&mut conn)
        .await
        .map_err(|e| Error::InsertFailed(e))?;

    Ok((
        StatusCode::OK,
        Json(GenericResponse {
            status: StatusCode::CREATED.to_string(),
            result: DataResponse::<String> {
                msg: "created user successfully".into(),
                data: None,
            },
        }),
    ))
}

pub async fn get_user_by_id(
    Extension(db): Extension<Arc<Database>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let mut conn = db.get_connection().await;
    let user = match users::table
        .find(id)
        .select(User::as_select())
        .first::<User>(&mut conn)
        .await
    {
        Ok(user) => Some(user),
        Err(e) => {
            match e {
                diesel::NotFound => None,
                e => return Err(Error::QueryFailed(e)),
            }
        }
    };

    let result = if let Some(u) = user {
        Some(UserResponse::from(u))
    } else {
        None
    };

    Ok((
        StatusCode::OK,
        Json(GenericResponse {
            status: StatusCode::OK.to_string(),
            result: DataResponse {
                msg: "success".into(),
                data: result,
            },
        }),
    ))
}

pub async fn get_all_user(Extension(db): Extension<Arc<Database>>) -> Result<impl IntoResponse> {
    let mut conn = db.get_connection().await;
    let users = users::table
        .select(User::as_select())
        .load::<User>(&mut conn)
        .await
        .map_err(|e| Error::QueryFailed(e))?
        .into_iter()
        .map(UserResponse::from)
        .collect::<Vec<_>>();

    Ok((
        StatusCode::OK,
        Json(GenericResponse {
            status: StatusCode::OK.to_string(),
            result: DataResponse {
                msg: "success".into(),
                data: Some(users),
            },
        }),
    ))
}

pub async fn update_user(
    Extension(db): Extension<Arc<Database>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse> {
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
        .map_err(|_| Error::RecordNotFound)?;

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
        .map_err(|e| Error::UpdateFailed(e))?;

    Ok((
        StatusCode::ACCEPTED,
        Json(GenericResponse {
            status: StatusCode::ACCEPTED.to_string(),
            result: DataResponse::<String> {
                msg: "success".into(),
                data: None,
            },
        }),
    ))
}

pub async fn delete_user(
    Extension(db): Extension<Arc<Database>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let mut conn = db.get_connection().await;
    delete(users::table.filter(users::id.eq(id)))
        .execute(&mut conn)
        .await
        .map_err(|e| Error::DeleteFailed(e))?;

    Ok((
        StatusCode::NO_CONTENT,
        Json(GenericResponse {
            status: StatusCode::NO_CONTENT.to_string(),
            result: DataResponse::<String> {
                msg: "success".into(),
                data: None,
            },
        }),
    ))
}
