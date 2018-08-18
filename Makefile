.PHONY: name all compile run clean test

all: clean compile

compile:
	@cargo build

run:
	@cargo run --release --quiet

clean:
	@cargo clean

test:
	cargo test
	@(echo "[1,1,2,3,5,8]"; echo 42; cat) | \
	cargo run --release --quiet

name:
	@echo 'Rusty'
