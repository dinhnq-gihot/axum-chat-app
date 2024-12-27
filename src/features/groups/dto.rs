use {
    super::models::Group as GroupModel,
    crate::features::users::dto::UserResponse,
    serde::{
        Deserialize,
        Serialize,
    },
    utoipa::ToSchema,
    uuid::Uuid,
};

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateGroup {
    pub group_name: String,
    pub user_emails: Option<Vec<String>>,
    pub user_names: Option<Vec<String>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GroupResponse {
    pub id: Uuid,
    pub name: String,
    pub users: Option<Vec<UserResponse>>,
}

impl From<GroupModel> for GroupResponse {
    fn from(value: GroupModel) -> Self {
        Self {
            id: value.id,
            name: value.name,
            users: None,
        }
    }
}
