use {
    axum::{
        http::StatusCode,
        response::{
            IntoResponse,
            Response,
        },
        Json,
    },
    diesel_async::pooled_connection::bb8::RunError,
    serde::Serialize,
    serde_json::{
        json,
        Value,
    },
    thiserror::Error,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    // Server errors
    #[error("Server served failed")]
    ServerServedFailed,

    // Environment variable errors
    #[error("Environment variable {0} not found")]
    EnvVarNotFound(String),

    // Database errors
    #[error("Database connection failed")]
    DatabaseConnectionFailed,
    #[error("Database migration failed")]
    DatabaseMigrationFailed,
    #[error("Pool connection failed: {0}")]
    PoolConnectionFailed(#[source] RunError),
    #[error("Insert failed: {0}")]
    InsertFailed(#[source] diesel::result::Error),
    #[error("Query failed {0}")]
    QueryFailed(#[source] diesel::result::Error),
    #[error("Update failed: {0}")]
    UpdateFailed(#[source] diesel::result::Error),
    #[error("Record not found")]
    RecordNotFound,
    #[error("Delete failed: {0}")]
    DeleteFailed(#[source] diesel::result::Error),

    // File errors
    #[error("Create file failed")]
    CreateFileFailed,

    #[error("File type invalid")]
    FileTypeInvalid,

    // JWT errors
    #[error("JWT decode failed: {0}")]
    DecodeJwtFailed(String),

    // Auth errors
    #[error("Please login first")]
    TokenNotFound,

    // anyhow error
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResp {
            status: String,
            result: Value,
        }

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResp {
                status: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                result: json!({
                    "msg": self.to_string(),
                    "data": ""
                }),
            }),
        )
            .into_response()
    }
}
