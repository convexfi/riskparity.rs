default: all

all: format check test

clean:
	@cargo clean

build:
	@cargo build

test:
	@cargo test --all

format:
	@rustup component add rustfmt 2> /dev/null
	@cargo fmt --all

check:
	@rustup component add rustfmt 2> /dev/null
	@rustup component add clippy 2> /dev/null
	@cargo fmt --all -- --check
	@cargo clippy -- -Dclippy::all -Dclippy::pedantic
	@cargo check

.PHONY: all clean build test format check
