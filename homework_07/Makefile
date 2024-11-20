path = ./smart_home

fmt:
	cd $(path) && cargo fmt --check --all

clippy:
	cd $(path) && cargo clippy --all --all-targets

test:
	cd $(path) && cargo test

run:
	cd $(path) && cargo run --example usage

run_server:
	cd $(path)/net/server && cargo run server

run_client:
	cd $(path)/net/client && cargo run client