use {
    super::model::Claims,
    crate::{
        database::Database,
        enums::errors::*,
        features::users::{
            dto::UserResponse,
            models::User,
        },
        schema::users,
        utils::jwt::decode_jwt,
        warn,
    },
    axum::{
        extract::Request,
        http::{
            header::AUTHORIZATION,
            HeaderMap,
        },
        middleware::Next,
        response::IntoResponse,
    },
    diesel::{
        query_dsl::methods::FilterDsl,
        ExpressionMethods,
    },
    diesel_async::RunQueryDsl,
    std::sync::Arc,
};

pub async fn check_jwt(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse> {
    let token = headers
        .get(AUTHORIZATION)
        .ok_or_else(|| {
            warn!("check_jwt: TokenNotFound");
            Error::TokenNotFound
        })?
        .to_str()
        .map_err(|e| {
            warn!("check_jwt: {}", e.to_string());
            Error::Anyhow(e.into())
        })?;

    let token = token.replace("Bearer ", "");

    let Claims {
        sub: email,
        exp: _,
        user_id,
    } = decode_jwt(token)?;
    let db = request.extensions().get::<Arc<Database>>().ok_or_else(|| {
        warn!("check_jwt: DatabaseConnectionFailed");
        Error::DatabaseConnectionFailed
    })?;
    let mut conn = db.get_connection().await;

    let user: UserResponse = users::table
        .filter(users::email.eq(email))
        .filter(users::id.eq(user_id))
        .first::<User>(&mut conn)
        .await
        .map_err(|_| Error::InvalidCredentials)?
        .into();

    drop(conn);

    request.extensions_mut().insert(user);

    let res = next.run(request).await;

    Ok(res)
}
