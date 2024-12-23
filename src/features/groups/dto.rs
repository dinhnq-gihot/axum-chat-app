use {
    crate::features::users::dto::UserResponse,
    serde::{
        Deserialize,
        Serialize,
    },
    uuid::Uuid,
};

#[derive(Debug, Deserialize)]
pub struct CreateGroup {
    pub name: String,
    pub user_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct GroupResponse {
    pub id: Uuid,
    pub users: Vec<UserResponse>,
}
