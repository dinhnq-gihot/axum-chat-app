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
    pub group_name: String,
    pub user_emails: Option<Vec<String>>,
    pub user_names: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct GroupResponse {
    pub id: Uuid,
    pub users: Vec<UserResponse>,
}
