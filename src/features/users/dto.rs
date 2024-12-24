use {
    super::model::User as UserModel,
    serde::{
        Deserialize,
        Serialize,
    },
    uuid::Uuid,
};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub avatar: Option<String>,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub avatar: Option<String>,
    pub is_online: bool,
}

impl From<UserModel> for UserResponse {
    fn from(value: UserModel) -> Self {
        Self {
            id: value.id,
            name: value.name,
            email: value.email,
            avatar: value.avatar,
            is_online: value.is_online.unwrap_or(false),
        }
    }
}