use {
    crate::features::chat::model::{
        Message,
        MessageType,
    },
    chrono::{
        DateTime,
        Utc,
    },
    serde::{
        Deserialize,
        Serialize,
    },
    uuid::Uuid,
};

#[derive(Debug, Serialize)]
pub struct MessageOut {
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub content: Option<String>,
    pub r#type: Option<MessageType>,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
}

impl From<Message> for MessageOut {
    fn from(value: Message) -> Self {
        let Message {
            id: _,
            user_id,
            group_id,
            content,
            r#type,
            created_at,
            edited_at,
        } = value;

        Self {
            user_id,
            group_id,
            content,
            r#type,
            created_at,
            edited_at,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JoinRoom {
    pub room: Uuid,
}
