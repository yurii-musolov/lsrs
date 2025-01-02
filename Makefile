format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

install:
	cargo install --path .

all: format lint test install
