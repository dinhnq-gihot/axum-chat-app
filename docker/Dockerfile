# Build stage
FROM rust:1.77-bookworm as builder

WORKDIR /app

COPY . .

RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y protobuf-compiler libprotobuf-dev libpq-dev

RUN cargo build --release --bin axum-chat-app

# Production stage
FROM debian:bookworm

WORKDIR /

RUN apt-get update && apt-get -y install libpq5 ca-certificates

COPY --from=builder /app/target/release/axum-chat-app /usr/local/sbin/

EXPOSE 3000

CMD ["/usr/local/sbin/axum-chat-app"]