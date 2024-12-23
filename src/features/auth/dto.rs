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
