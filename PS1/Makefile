FILES=nodes.rs main.rs
CC=rustc

OUT=prob1



build:
	cargo build

run: build
	cargo run

remake:
	$(MAKE) clean
	$(MAKE) run

clean:
	cargo clean
