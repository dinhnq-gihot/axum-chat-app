use {
    serde::{
        Deserialize,
        Serialize,
    },
    utoipa::ToSchema,
};

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub msg: String,
    pub token: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub avatar: Option<String>,
    pub password: String,
}
