use {
    super::types::{
        DataResponse,
        GenericResponse,
    },
    axum::{
        http::StatusCode,
        response::{
            IntoResponse,
            Response,
        },
        Json,
    },
    diesel_async::pooled_connection::bb8::RunError,
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
    #[error("User already exists")]
    UserAlreadyExists,

    // File errors
    #[error("Create file failed")]
    CreateFileFailed,
    #[error("File type invalid")]
    FileTypeInvalid,
    #[error("Field not found: {0}")]
    FieldNotFound(String),

    #[error("Not self assign")]
    NotSelfAssign,

    // JWT errors
    #[error("JWT decode failed: {0}")]
    DecodeJwtFailed(#[source] jsonwebtoken::errors::Error),
    #[error("JWT encode failed: {0}")]
    EncodeJwtFailed(#[source] jsonwebtoken::errors::Error),

    // Auth errors
    #[error("Please login first")]
    TokenNotFound,
    #[error("Hash password failed")]
    HashingFailed,
    #[error("Verify password failed")]
    VerifyPasswordFailed,
    #[error("Invalid credentials")]
    InvalidCredentials,

    // anyhow error
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    // Access denied
    #[error("Access denied: {0} role required")]
    AccessDenied(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            // Handle authorization error
            Error::TokenNotFound | Error::InvalidCredentials | Error::DecodeJwtFailed(_) => {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(GenericResponse {
                        status: StatusCode::UNAUTHORIZED.to_string(),
                        result: DataResponse::<String> {
                            msg: self.to_string(),
                            data: None,
                        },
                    }),
                )
                    .into_response()
            }
            Error::AccessDenied(role) => {
                (
                    StatusCode::FORBIDDEN,
                    Json(GenericResponse {
                        status: StatusCode::FORBIDDEN.to_string(),
                        result: DataResponse::<String> {
                            msg: format!("Access denied: Role '{}' is not allowed", role),
                            data: None,
                        },
                    }),
                )
                    .into_response()
            }
            Error::RecordNotFound => {
                (
                    StatusCode::NOT_FOUND,
                    Json(GenericResponse {
                        status: StatusCode::NOT_FOUND.to_string(),
                        result: DataResponse::<String> {
                            msg: "Record Not Found".into(),
                            data: None,
                        },
                    }),
                )
                    .into_response()
            }

            // Handle other errors as internal server errors
            _ => {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(GenericResponse {
                        status: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                        result: DataResponse::<String> {
                            msg: self.to_string(),
                            data: None,
                        },
                    }),
                )
                    .into_response()
            }
        }
    }
}
