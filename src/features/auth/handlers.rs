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
        ExpressionMethods,
        QueryDsl,
    },
    diesel_async::RunQueryDsl,
    std::sync::Arc, tracing::debug,
};

#[utoipa::path(
    post,
    context_path = "/api",
    path = "/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login Success", body = GenericResponse<String>),
        (status = 401, description = "Invalid Credentials")
    ),
    tag = "Authentication"
)]
pub async fn login(
    Extension(db): Extension<Arc<Database>>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse> {
    debug!("login: payload {payload:?}");

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
        StatusCode::OK,
        Json(GenericResponse {
            status: StatusCode::OK.to_string(),
            result: DataResponse {
                msg: "Login Success".into(),
                data: Some(token),
            },
        }),
    ))
}

#[utoipa::path(
    post,
    context_path = "/api",
    path = "/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Register Success", body = GenericResponse<String>),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Authentication"
)]
pub async fn register(
    Extension(db): Extension<Arc<Database>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse> {
    debug!("register: payload {payload:?}");

    let RegisterRequest {
        username,
        email,
        avatar,
        password,
    } = payload;

    let mut conn = db.get_connection().await;

    // Count the number of users with the given email
    let user_email_count = users::table
        .filter(users::email.eq(&email))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .map_err(Error::QueryFailed)?;

    let username_count = users::table
        .filter(users::name.eq(&username))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .map_err(Error::QueryFailed)?;

    if user_email_count > 0 || username_count > 0 {
        return Err(Error::UserAlreadyExists);
    }

    let hashed_password = hash(&password, DEFAULT_COST).map_err(|_| Error::HashingFailed)?;

    insert_into(users::table)
        .values(NewUser {
            name: &username,
            email: &email,
            password: &hashed_password,
            avatar: avatar.as_deref(),
            role: "user"
        })
        .execute(&mut conn)
        .await
        .map_err(|e| Error::InsertFailed(e))?;

    Ok((
        StatusCode::OK,
        Json(GenericResponse {
            status: StatusCode::OK.to_string(),
            result: DataResponse::<String> {
                msg: "Register successfully".into(),
                data: None,
            },
        }),
    ))
}
