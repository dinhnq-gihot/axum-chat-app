use {
    super::dto::{
        LoginRequest,
        RegisterRequest,
    },
    crate::{
        database::Database,
        enums::{
            errors::*,
            types::{
                DataResponse,
                GenericResponse,
            },
        },
        features::users::models::{
            NewUser,
            User,
        },
        schema::users,
        utils::jwt,
    },
    axum::{
        http::StatusCode,
        response::IntoResponse,
        Extension,
        Json,
    },
    bcrypt::{
        hash,
        verify,
        DEFAULT_COST,
    },
    diesel::{
        insert_into,
        query_dsl::methods::FilterDsl,
        ExpressionMethods,
    },
    diesel_async::RunQueryDsl,
    std::sync::Arc,
};

pub async fn login(
    Extension(db): Extension<Arc<Database>>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse> {
    let LoginRequest { email, password } = payload;

    let mut conn = db.get_connection().await;
    let user = users::table
        .filter(users::email.eq(email))
        .first::<User>(&mut conn)
        .await
        .map_err(|_| Error::InvalidCredentials)?;

    // Verify password
    let is_valid = verify(password, &user.password).map_err(|_| Error::VerifyPasswordFailed)?;

    if !is_valid {
        return Err(Error::InvalidCredentials);
    }

    let token = jwt::encode_jwt(user.id, user.email)?;

    Ok((
        StatusCode::CREATED,
        Json(GenericResponse {
            status: StatusCode::CREATED.to_string(),
            result: DataResponse {
                msg: "Login Success".into(),
                data: Some(token),
            },
        }),
    ))
}

pub async fn register(
    Extension(db): Extension<Arc<Database>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse> {
    let mut conn = db.get_connection().await;

    let hashed_password =
        hash(&payload.password, DEFAULT_COST).map_err(|_| Error::HashingFailed)?;

    insert_into(users::table)
        .values(NewUser {
            name: &payload.username,
            email: &payload.email,
            password: &hashed_password,
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
