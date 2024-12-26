pub mod database;
pub mod enums;
pub mod features;
mod router;
pub mod schema;
pub mod socket;
pub mod utils;

use {
    axum::{
        http::HeaderValue,
        Extension,
    },
    database::Database,
    dotenv::dotenv,
    enums::errors::{
        Error,
        Result,
    },
    router::create_router,
    socket::{
        check_login,
        on_connect,
    },
    socketioxide::{
        handler::ConnectHandler,
        SocketIo,
    },
    std::{
        env,
        sync::Arc,
    },
    tokio::net::TcpListener,
    tower::ServiceBuilder,
    tower_http::{
        cors::CorsLayer,
        services::ServeDir,
        trace::TraceLayer,
    },
    tracing::info,
    tracing_subscriber::FmtSubscriber,
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())
        .map_err(|e| Error::Anyhow(e.into()))?;

    // load environment variables from a .env file
    dotenv().ok();

    let db_url =
        env::var("DATABASE_URL").map_err(|_| Error::EnvVarNotFound("DATABASE_URL".to_string()))?;
    let db = Arc::new(Database::try_new(db_url).await?);

    let (layer, io) = SocketIo::builder()
        .with_state(Arc::clone(&db))
        .build_layer();
    io.ns("/", on_connect.with(check_login));

    let host = env::var("HOST").map_err(|_| Error::EnvVarNotFound("HOST".to_string()))?;
    let port = env::var("PORT").map_err(|_| Error::EnvVarNotFound("PORT".to_string()))?;
    let url = format!("{host}:{port}");

    let listener = TcpListener::bind(&url)
        .await
        .map_err(|e| Error::Anyhow(e.into()))?;

    let app = create_router()
        .fallback_service(ServeDir::new("public"))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::new().allow_origin("*".parse::<HeaderValue>().unwrap()))
                .layer(Extension(db))
                .layer(layer),
        );

    info!("Starting server at: {url}");
    axum::serve(listener, app)
        .await
        .map_err(|_| Error::ServerServedFailed)?;

    Ok(())
}
