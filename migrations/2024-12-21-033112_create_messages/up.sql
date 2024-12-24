-- Your SQL goes here
CREATE TABLE messages (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    group_id UUID NOT NULL REFERENCES groups(id),
    content TEXT,
    type message_types DEFAULT 'text',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    edited_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);