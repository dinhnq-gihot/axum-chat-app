use {
    crate::features::chat::model::MessageType,
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
    pub username: String,
    pub group_id: Uuid,
    pub content: Option<String>,
    pub r#type: Option<MessageType>,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JoinRoom {
    pub room: Uuid,
}
