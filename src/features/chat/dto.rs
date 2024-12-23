use {
    super::model::{
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

#[derive(Debug, Deserialize)]
pub struct Chat {
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub content: String,
    pub name_file: Option<String>,
    pub message_type: String,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub content: Option<String>,
    pub name_file: Option<String>,
    pub r#type: Option<MessageType>,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
}

impl From<Message> for MessageResponse {
    fn from(value: Message) -> Self {
        let Message {
            id: _,
            user_id,
            group_id,
            content,
            name_file,
            r#type,
            created_at,
            edited_at,
        } = value;

        Self {
            user_id,
            group_id,
            content,
            name_file,
            r#type,
            created_at,
            edited_at,
        }
    }
}
