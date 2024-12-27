use {
    crate::{
        database::Database,
        features::{
            chat::{
                dto::Chat,
                services::{
                    get_all_msgs_in_group,
                    insert_chat,
                },
            },
            groups::services::get_group_by_id,
        },
        warn,
    },
    socketioxide::{
        extract::{
            Data,
            SocketRef,
            State,
        },
        socket::DisconnectReason,
    },
    std::sync::Arc,
    tracing::{
        debug,
        info,
    },
    uuid::Uuid,
};

pub async fn handle_message(socket: SocketRef, Data(chat): Data<Chat>, db: State<Arc<Database>>) {
    debug!("Received message: {:?}", chat);

    let message_back = insert_chat(db.0, chat.clone())
        .await
        .map_err(|e| warn!("{}", e.to_string()))
        .unwrap();

    socket
        .within(chat.group_id.to_string())
        .emit("message-back", &message_back)
        .ok();
}

pub async fn handle_join(socket: SocketRef, Data(room): Data<Uuid>, db: State<Arc<Database>>) {
    debug!("Received join: {:?}", room);

    let messages = get_all_msgs_in_group(Arc::clone(&db.0), room)
        .await
        .unwrap();
    let room_name = get_group_by_id(Arc::clone(&db.0), room).await.unwrap().name;

    socket
        .leave_all()
        .map_err(|e| warn!("{}", e.to_string()))
        .unwrap();

    socket
        .join(room.to_string())
        .map_err(|e| warn!("{}", e.to_string()))
        .unwrap();

    socket
        .within(room.to_string())
        .emit("join-room-back", &room_name)
        .map_err(|e| warn!("{}", e.to_string()))
        .unwrap();

    socket
        .within(room.to_string())
        .emit("messages", &messages)
        .map_err(|e| warn!("{}", e.to_string()))
        .unwrap();
}

pub async fn handle_disconnect(socket: SocketRef, reason: DisconnectReason) {
    info!("Socket {} was disconnected because {}", socket.id, reason);
}
