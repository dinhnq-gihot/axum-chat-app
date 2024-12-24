use {
    super::dto::LoginRequest,
    crate::{
        database::Database,
        enums::{
            errors::*,
            types::{
                DataResponse,
                GenericResponse,
            },
        },
        features::users::model::User,
        schema::users,
        utils::jwt,
    },
    axum::{
        http::StatusCode,
        response::IntoResponse,
        Extension,
        Json,
    },
    diesel::{
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
        .filter(users::password.eq(password))
        .first::<User>(&mut conn)
        .await
        .map_err(|_| Error::RecordNotFound)?;

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
