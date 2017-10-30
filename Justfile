all: build test
build:
	cargo build
	cargo build --release
test:
	cargo test
	cargo test --release
watch:
	cargo watch -x check -x doc -x test -x run
