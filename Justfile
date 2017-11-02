all: build test
build: build-debug build-release
build-debug:
	cargo build
build-release:
	cargo build --release
run-forever: build-release
	until target/release/tri |& tee -a tri.log; do true; done
test: test-debug test-release
test-debug:
	cargo test
test-release:
	cargo test --release
watch:
	cargo watch -x check -x doc -x test -x run
