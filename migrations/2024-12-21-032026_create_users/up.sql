-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    avatar VARCHAR(255),
    is_online BOOLEAN DEFAULT FALSE
);

CREATE INDEX idx_user_email ON users (email);

INSERT INTO users (name, email, password) VALUES
('dinh1', 'dinh1@gmail.com', '$2b$12$KVhV/jjY8PcpGHA88fSVsen4cvZq/LKT5Vg2h8eVM.XYXtGnq2RX6'),
('dinh2', 'dinh2@gmail.com', '$2b$12$KVhV/jjY8PcpGHA88fSVsen4cvZq/LKT5Vg2h8eVM.XYXtGnq2RX6'),
('dinh3', 'dinh3@gmail.com', '$2b$12$KVhV/jjY8PcpGHA88fSVsen4cvZq/LKT5Vg2h8eVM.XYXtGnq2RX6');