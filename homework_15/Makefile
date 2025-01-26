path = ./smart_home

fmt:
	cd $(path) && cargo fmt --check --all

fmt_fix:
	cd $(path) && cargo fmt --all

clippy:
	cd $(path) && cargo clippy --all --all-targets

test:
	cd $(path) && cargo test

run:
	cd $(path) && cargo run --example usage

run_tcp_server:
	cd $(path)/tcp/server && cargo run server

run_tcp_client:
	cd $(path)/tcp/client && cargo run client

run_udp_server:
	cd $(path)/udp/server && cargo run server

run_udp_client:
	cd $(path)/udp/client && cargo run client

coverage:
	cd $(path) && cargo tarpaulin --verbose

coverage_html:
	cd $(path) && cargo tarpaulin --out Html

run_tcp_async_server:
	cd $(path)/tcp_async/server && cargo run server

run_tcp_async_client:
	cd $(path)/tcp_async/client && cargo run client

run_udp_async_server:
	cd $(path)/udp_async/server && cargo run server

run_udp_async_client:
	cd $(path)/udp_async/client && cargo run client