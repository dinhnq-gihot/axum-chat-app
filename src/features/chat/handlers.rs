use {
    super::{
        dto::{
            Chat,
            MessageResponse,
        },
        model::{
            Message,
            MessageType,
            NewMessage,
        },
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
        insert_into,
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
    let Chat {
        user_id,
        group_id,
        content,
        name_file,
        message_type,
    } = payload;
    let message_type = MessageType::from(message_type);

    let new_message = NewMessage {
        user_id: &user_id,
        group_id: &group_id,
        content: Some(&content),
        name_file: name_file.as_deref(),
        r#type: &message_type,
    };

    let mut conn = db.get_connection().await;
    insert_into(messages::table)
        .values(new_message)
        .execute(&mut conn)
        .await
        .map_err(|e| Error::InsertFailed(e))?;

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
