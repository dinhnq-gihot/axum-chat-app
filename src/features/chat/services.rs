use {
    super::{
        dto::Chat,
        model::NewMessage,
    },
    crate::{
        database::Database,
        enums::errors::*,
        schema::messages,
    },
    diesel::insert_into,
    diesel_async::RunQueryDsl,
    std::sync::Arc,
};

pub async fn insert_chat(db: Arc<Database>, chat: Chat) -> Result<()> {
    let mut conn = db.get_connection().await;
    let Chat {
        user_id,
        group_id,
        content,
        message_type,
    } = chat;

    let new_message = NewMessage {
        user_id: &user_id,
        group_id: &group_id,
        content: Some(&content),
        r#type: &message_type.into(),
    };

    insert_into(messages::table)
        .values(new_message)
        .execute(&mut conn)
        .await
        .map_err(|e| Error::InsertFailed(e))?;

    Ok(())
}
