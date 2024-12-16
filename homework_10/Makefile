path = ./decorator

fmt:
	cd $(path)/static && cargo fmt --check --all
	cd $(path)/dynamic && cargo fmt --check --all
	cd $(path)/func && cargo fmt --check --all

clippy:
	cd $(path)/static && cargo clippy --all --all-targets
	cd $(path)/dynamic && cargo clippy --all --all-targets
	cd $(path)/func && cargo clippy --all --all-targets

test:
	cd $(path) && cargo test

run_decorator_static:
	cd $(path)/static && cargo run

run_decorator_dynamic:
	cd $(path)/dynamic && cargo run

run_decorator_func:
	cd $(path)/func && cargo run