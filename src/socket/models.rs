use {
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
    pub content: String,
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JoinRoom {
    pub room: Uuid,
}
