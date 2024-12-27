use {
    super::{
        dto::{
            CreateUserRequest,
            UpdateUserRequest,
            UserResponse,
        },
        models::{
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
        extract::{
            Multipart,
            Path,
        },
        http::StatusCode,
        response::IntoResponse,
        Extension,
        Json,
    },
    axum_chat_app::only_role,
    diesel::{
        delete,
        insert_into,
        prelude::*,
        update,
    },
    diesel_async::RunQueryDsl,
    regex::Regex,
    std::sync::Arc,
    tokio::{
        fs::File,
        io::AsyncWriteExt,
    },
    tracing::debug,
    uuid::Uuid,
};

#[only_role("admin")]
pub async fn create_user(
    Extension(db): Extension<Arc<Database>>,
    Extension(sender): Extension<User>,
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

#[utoipa::path(
    get,
    context_path = "/api",
    path = "/users/{id}",
    params(
        ("id" = Uuid, description = "User ID")
    ),
    operation_id = "get_user_by_id",
    responses(
        (status = 200, description = "User found", body = GenericResponse<UserResponse>),
        (status = 404, description = "User not found")
    ),
    security(("bearerAuth" = [])), // Apply JWT security only here
    tag = "Users"
)]
#[only_role("user", "admin")]
pub async fn get_user_by_id(
    Extension(db): Extension<Arc<Database>>,
    Extension(sender): Extension<User>,
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

#[utoipa::path(
    get,
    context_path = "/api",
    path = "/users",
    responses(
        (status = 200, description = "List of users", body = GenericResponse<Vec<UserResponse>>),
        (status = 500, description = "Internal Server Error"),
    ),
    operation_id = "get_all_user",
    security(("bearerAuth" = [])), // Apply JWT security only here
    tag = "Users"
)]
#[only_role("admin")]
pub async fn get_all_user(
    Extension(db): Extension<Arc<Database>>,
    Extension(sender): Extension<User>,
) -> Result<impl IntoResponse> {
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

#[utoipa::path(
    patch,
    context_path = "/api",
    path = "/users",
    request_body = UpdateUserRequest,
    responses(
        (status = 202, description = "User updated successfully", body = GenericResponse<String>),
        (status = 500, description = "Internal Server Error"),
    ),
    security(("bearerAuth" = [])), // Apply JWT security only here
    tag = "Users"
)]
#[only_role("user")]
pub async fn update_user(
    Extension(db): Extension<Arc<Database>>,
    Extension(sender): Extension<User>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse> {
    let UpdateUserRequest {
        name,
        email,
        avatar,
    } = payload;

    let mut conn = db.get_connection().await;
    let mut existed_user: User = users::table
        .find(sender.id)
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

    update(users::table.filter(users::id.eq(sender.id)))
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
                msg: "User updated successfully".into(),
                data: None,
            },
        }),
    ))
}

#[utoipa::path(
    delete,
    context_path = "/api",
    path = "/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 204, description = "User deleted successfully", body = GenericResponse<String>),
        (status = 500, description = "Internal Server Error"),
    ),
    security(("bearerAuth" = [])), // Apply JWT security only here
    tag = "Users"
)]
#[only_role("admin")]
pub async fn delete_user(
    Extension(db): Extension<Arc<Database>>,
    Extension(sender): Extension<User>,
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

#[utoipa::path(
    post,
    context_path = "/api",
    path = "/users/avatar",
    request_body(content_type = "multipart/form-data"),
    responses(
        (status = 202, description = "Avatar updated successfully", body = GenericResponse<String>),
        (status = 400, description = "Invalid file type or field not found"),
        (status = 500, description = "Internal Server Error"),
    ),
    security(("bearerAuth" = [])), // Apply JWT security only here
    tag = "Users"
)]
#[only_role("user")]
pub async fn update_avatar(
    Extension(db): Extension<Arc<Database>>,
    Extension(sender): Extension<User>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse> {
    let mut updated = false;
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| Error::Anyhow(e.into()))?
    {
        if let Some("avatar") = field.name() {
            let parts: Vec<_> = field.file_name().unwrap_or_default().split(".").collect();
            let (filename, extension) = match parts.as_slice() {
                [filename, extension, ..] => (filename, extension),
                _ => return Err(Error::FileTypeInvalid),
            };
            let content_type = field.content_type().unwrap_or_default();

            debug!("filename:{filename} - extension:{extension} - content_type:{content_type}");
            let regex =
                Regex::new(mime::IMAGE_STAR.as_ref()).map_err(|e| Error::Anyhow(e.into()))?;

            if regex.is_match(&content_type) {
                let mut conn = db.get_connection().await;
                let mut existed_user: User = users::table
                    .find(sender.id)
                    .select(User::as_select())
                    .first(&mut conn)
                    .await
                    .map_err(|_| Error::RecordNotFound)?;

                let new_filename = format!("{filename}-{}.{extension}", Uuid::new_v4().to_string());
                existed_user.avatar = Some(new_filename.to_string());

                let mut file = File::create(format!("public/uploads/{new_filename}"))
                    .await
                    .map_err(|_| Error::CreateFileFailed)?;
                let data = field.bytes().await.map_err(|e| Error::Anyhow(e.into()))?;
                file.write(&data)
                    .await
                    .map_err(|e| Error::Anyhow(e.into()))?;

                update(users::table)
                    .set(existed_user)
                    .execute(&mut conn)
                    .await
                    .map_err(|e| Error::UpdateFailed(e))?;

                updated = true;
            } else {
                return Err(Error::FileTypeInvalid);
            }
        }
    }

    if updated {
        return Ok((
            StatusCode::ACCEPTED,
            Json(GenericResponse {
                status: StatusCode::ACCEPTED.to_string(),
                result: DataResponse::<String> {
                    msg: "Avatar updated successfully".into(),
                    data: None,
                },
            }),
        ));
    } else {
        return Err(Error::FieldNotFound("avatar".into()));
    }
}
