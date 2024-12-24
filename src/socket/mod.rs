use {
    axum::http::header::AUTHORIZATION,
    handlers::{
        handle_disconnect,
        handle_join,
        handle_message,
    },
    socketioxide::extract::SocketRef,
    tracing::info,
};

pub mod handlers;
pub mod models;
use crate::enums::errors::*;

pub async fn check_login(socket: SocketRef) -> Result<()> {
    let _ = socket
        .req_parts()
        .headers
        .get(AUTHORIZATION)
        .ok_or(Error::TokenNotFound)?
        .to_str()
        .or_else(|e| Err(Error::Anyhow(e.into())))?;

    Ok(())
}

pub async fn on_connect(socket: SocketRef) {
    info!("socket connected {}", socket.id);

    socket.on("message", handle_message);
    socket.on("join", handle_join);
    socket.on_disconnect(handle_disconnect);
}
