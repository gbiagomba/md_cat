PROJECT_NAME := md_cat
BINARY_PATH := ./target/release/$(PROJECT_NAME)

build:
	cargo build --release

install:
	cargo install --path .

run:
	cargo run --release -- 

test:
	cargo test

clean:
	cargo clean