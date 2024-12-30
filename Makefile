VERBOSE := $(if ${CI},--verbose,)
CARGO := cargo

check:
	${CARGO} check ${VERBOSE} --all

check-fmt:
	cargo +nightly fmt ${VERBOSE} --all -- --check

fmt:
	cargo +nightly fmt ${VERBOSE} --all

clippy:
	${CARGO} clippy ${VERBOSE} --all --all-targets --all-features -- \
		-D warnings -D clippy::enum_glob_use

ci: fmt check-fmt clippy test

run-dev:
	RUST_BACKTRACE=full RUST_LOG=trace,hyper=trace,axum-chat-app=trace${RUST_LOG} ${CARGO} run --bin axum-chat-app --release

build-image:
	docker build . -t axum-chat-app:v0.0.1 -f ./docker/Dockerfile

