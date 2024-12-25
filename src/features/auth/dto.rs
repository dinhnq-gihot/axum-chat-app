use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub msg: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub avatar: Option<String>,
    pub password: String,
}