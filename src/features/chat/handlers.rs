use {
    super::{
        dto::{
            Chat,
            MessageResponse,
        },
        model::Message,
        services::insert_chat,
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
        schema::messages,
    },
    axum::{
        extract::Path,
        http::StatusCode,
        response::IntoResponse,
        Extension,
        Json,
    },
    diesel::{
        prelude::*,
        ExpressionMethods,
        SelectableHelper,
    },
    diesel_async::RunQueryDsl,
    std::sync::Arc,
    uuid::Uuid,
};

pub async fn chat(
    Extension(db): Extension<Arc<Database>>,
    Json(payload): Json<Chat>,
) -> Result<impl IntoResponse> {
    insert_chat(Arc::clone(&db), payload).await?;

    Ok((
        StatusCode::CREATED,
        Json(GenericResponse {
            status: StatusCode::CREATED.to_string(),
            result: DataResponse::<String> {
                msg: "success".into(),
                data: None,
            },
        }),
    ))
}

pub async fn get_group_messages(
    Extension(db): Extension<Arc<Database>>,
    Path(group_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let mut conn = db.get_connection().await;

    let messages = messages::table
        .filter(messages::group_id.eq(group_id))
        .select(Message::as_select())
        .load::<Message>(&mut conn)
        .await
        .map_err(|e| Error::QueryFailed(e))?;

    let result = messages
        .into_iter()
        .map(|m| m.into())
        .collect::<Vec<MessageResponse>>();

    Ok((
        StatusCode::OK,
        Json(GenericResponse {
            status: StatusCode::OK.to_string(),
            result: DataResponse {
                msg: "success".into(),
                data: Some(result),
            },
        }),
    ))
}
