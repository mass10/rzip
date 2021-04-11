# Make debug build from source. (default)
build:
	cargo fmt
	cargo build

# Install release build from source.
install:
	cargo fmt
	cargo build --release --quiet
	cargo install --path .

# Uninstall.
uninstall:
	cargo uninstall rzip
