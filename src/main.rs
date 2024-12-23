pub mod database;
pub mod enums;
pub mod features;
mod router;
pub mod schema;

use {
    axum::{
        http::StatusCode,
        routing::{
            get,
            post,
        },
        Json,
        Router,
    },
    database::Database,
    serde::{
        Deserialize,
        Serialize,
    },
    tokio::net::TcpListener,
    tracing::info,
    tracing_subscriber::FmtSubscriber,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));

    info!("Starting server");

    let db = Database::new("postgresql://chatapp:123@localhost:15432/chatapp".into()).await;

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello World"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
