all: debug release

debug:
	cargo build

release:
	cargo build --release

clean:
	cargo clean
