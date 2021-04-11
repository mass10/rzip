build:
	cargo fmt
	cargo build

install:
	cargo fmt
	cargo build --release --quiet
	cargo install --path .

uninstall:
	cargo uninstall rcleanup

