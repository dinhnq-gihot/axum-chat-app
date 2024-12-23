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
    serde_json::json,
    uuid::Uuid,
};

pub async fn chat(
    Extension(db): Extension<Database>,
    Json(payload): Json<Chat>,
) -> impl IntoResponse {
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
        .unwrap();

    (
        StatusCode::CREATED,
        Json(json!({"result": "created message successfully"})),
    )
}

pub async fn get_group_messages(
    Extension(db): Extension<Database>,
    Path(group_id): Path<Uuid>,
) -> impl IntoResponse {
    let mut conn = db.get_connection().await;

    let messages = messages::table
        .filter(messages::group_id.eq(group_id))
        .select(Message::as_select())
        .load::<Message>(&mut conn)
        .await
        .unwrap();

    let result = messages
        .into_iter()
        .map(|m| m.into())
        .collect::<Vec<MessageResponse>>();

    (StatusCode::OK, Json(json!({ "result": result })))
}
