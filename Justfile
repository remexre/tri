all: build
build:
	cargo build --all
watch:
	cargo watch -x check -x doc -x test -x run
