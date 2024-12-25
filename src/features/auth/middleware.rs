use {
    super::model::Claims,
    crate::{
        database::Database,
        enums::errors::*,
        features::users::models::User,
        schema::users,
        utils::jwt::decode_jwt,
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
        .ok_or(Error::TokenNotFound)?
        .to_str()
        .or_else(|e| Err(Error::Anyhow(e.into())))?;

    let token = token.replace("Bearer ", "");

    let Claims {
        sub: email,
        exp: _,
        user_id,
    } = decode_jwt(token)?;
    let db = request
        .extensions()
        .get::<Arc<Database>>()
        .ok_or(Error::DatabaseConnectionFailed)?;
    let mut conn = db.get_connection().await;

    let user = users::table
        .filter(users::email.eq(email))
        .filter(users::id.eq(user_id))
        .first::<User>(&mut conn)
        .await
        .map_err(|_| Error::RecordNotFound)?;

    drop(conn);

    request.extensions_mut().insert(user);

    let res = next.run(request).await;

    Ok(res)
}
