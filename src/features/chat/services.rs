use {
    super::{
        dto::Chat,
        model::{
            Message,
            NewMessage,
        },
    },
    crate::{
        database::Database,
        enums::errors::*,
        features::users::{
            models::User,
            services::get_all_user_in_group,
        },
        schema::{
            messages,
            users,
        },
        socket::models::MessageOut,
    },
    diesel::{
        insert_into,
        prelude::*,
        ExpressionMethods,
        SelectableHelper,
    },
    diesel_async::RunQueryDsl,
    std::sync::Arc,
    tracing::debug,
    uuid::Uuid,
};

pub async fn insert_chat(db: Arc<Database>, chat: Chat) -> Result<MessageOut> {
    debug!("insert_chat");

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

    let message = insert_into(messages::table)
        .values(new_message)
        .returning(Message::as_returning())
        .get_result::<Message>(&mut conn)
        .await
        .map_err(|e| Error::InsertFailed(e))?;

    let user = users::table
        .find(message.user_id)
        .select(User::as_select())
        .first::<User>(&mut conn)
        .await
        .map_err(|_| Error::RecordNotFound)?;

    let message_out = MessageOut {
        username: user.name,
        group_id: message.group_id,
        content: message.content,
        r#type: message.r#type,
        created_at: message.created_at,
        edited_at: message.edited_at,
    };

    Ok(message_out)
}

pub async fn get_all_msgs_in_group(db: Arc<Database>, group_id: Uuid) -> Result<Vec<MessageOut>> {
    let mut conn = db.get_connection().await;

    let messages = messages::table
        .filter(messages::group_id.eq(group_id))
        .select(Message::as_select())
        .load::<Message>(&mut conn)
        .await
        .map_err(|e| Error::QueryFailed(e))?;

    let users = get_all_user_in_group(Arc::clone(&db), group_id).await?;

    let mut res = Vec::new();
    for message in messages.into_iter() {
        if let Some(user) = users.iter().find(|u| u.id == message.user_id) {
            let message_out = MessageOut {
                username: user.name.clone(),
                group_id: message.group_id,
                content: message.content,
                r#type: message.r#type,
                created_at: message.created_at,
                edited_at: message.edited_at,
            };
            res.push(message_out);
        }
    }

    Ok(res)
}
