VERBOSE := $(if ${CI},--verbose,)
CARGO := cargo

run-dev:
	RUST_BACKTRACE=full RUST_LOG=trace,hyper=trace,axum-chat-app=trace${RUST_LOG} ${CARGO} run --bin axum-chat-app --release