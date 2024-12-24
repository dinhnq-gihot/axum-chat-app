use {
    super::models::{
        JoinRoom,
        MessageOut,
    },
    crate::{
        database::Database,
        enums::errors::*,
        features::chat::{
            dto::Chat,
            services::insert_chat,
        },
    },
    chrono::Utc,
    socketioxide::{
        extract::{
            Data,
            SocketRef,
        },
        socket::DisconnectReason,
    },
    std::sync::Arc,
    tracing::{
        info,
        warn,
    },
};

pub async fn handle_message(socket: SocketRef, Data(chat): Data<Chat>) {
    let db = socket
        .req_parts()
        .extensions
        .get::<Arc<Database>>()
        .ok_or(Error::DatabaseConnectionFailed)
        .map_err(|e| warn!("{}", e.to_string()))
        .unwrap();

    insert_chat(Arc::clone(db), chat.clone())
        .await
        .map_err(|e| warn!("{}", e.to_string()))
        .unwrap();

    let response = MessageOut {
        content: chat.content,
        user_id: chat.user_id,
        group_id: chat.group_id,
        created_at: Utc::now(),
    };

    socket
        .within(chat.group_id.to_string())
        .emit("message-back", &response)
        .map_err(|e| warn!("{}", e.to_string()))
        .unwrap();
}

pub fn handle_join(socket: SocketRef, Data(data): Data<JoinRoom>) {
    info!("Received join: {:?}", data);

    socket
        .leave_all()
        .map_err(|e| warn!("{}", e.to_string()))
        .unwrap();
    socket
        .join(data.room.to_string())
        .map_err(|e| warn!("{}", e.to_string()))
        .unwrap();

    socket
        .within(data.room.to_string())
        .emit("join-room-back", &data)
        .map_err(|e| warn!("{}", e.to_string()))
        .unwrap();
}

pub async fn handle_disconnect(socket: SocketRef, reason: DisconnectReason) {
    info!("Socket {} was disconnected because {}", socket.id, reason);
}
