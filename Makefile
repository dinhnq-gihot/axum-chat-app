VERBOSE := $(if ${CI},--verbose,)
CARGO := cargo

run-dev:
	RUST_BACKTRACE=full RUST_LOG=trace,hyper=trace,axum-chat-app=trace${RUST_LOG} ${CARGO} run --bin axum-chat-app --release

build-image:
	docker build . -t axum-chat-app:v0.0.1 -f ./docker/Dockerfile