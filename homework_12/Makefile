path = .

fmt_strategy:
	cd $(path)/strategy && cargo fmt --all

fmt_builder:
	cd $(path)/builder && cargo fmt --all

clippy_strategy:
	cd $(path)/strategy && cargo clippy --all --all-targets

clippy_builder:
	cd $(path)/builder && cargo clippy --all --all-targets

test_strategy:
	cd $(path)/strategy && cargo test

run_strategy:
	cd $(path)/strategy && cargo run

test_builder:
	cd $(path)/builder && cargo test

run_builder:
	cd $(path)/builder && cargo run
#run_decorator_dynamic:
	#cd $(path)/dynamic && cargo run

