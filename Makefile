VERSION = $(patsubst "%",%, $(word 3, $(shell grep version Cargo.toml)))
BUILD_TIME = $(shell date +"%Y/%m/%d %H:%M:%S")
GIT_REVISION = $(shell git log -1 --format="%h")

export BUILD_TIME
export GIT_REVISION

.PHONY: all test clean release_lnx32 release_lnx64 release_osx32 release_osx64

all: test

test:
	cargo run

clean:
	cargo clean

release_lnx32:
	cargo build --release --target=i686-unknown-linux-musl
	zip -j softether-exporter-v${VERSION}-i686-lnx.zip target/i686-unknown-linux-musl/release/softether-exporter

release_lnx64:
#	cargo build --release --target=x86_64-unknown-linux-musl
	cargo build --release --target=x86_64-pc-windows-gnu
	zip -j softether-exporter-v${VERSION}-x86_64-lnx.zip target/x86_64-unknown-linux-musl/release/softether-exporter

release_osx32:
	cargo build --release --target=i686-apple-darwin
	zip -j softether-exporter-v${VERSION}-i686-osx.zip target/i686-apple-darwin/release/softether-exporter

release_osx64:
	cargo build --release --target=x86_64-apple-darwin
	zip -j softether-exporter-v${VERSION}-x86_64-osx.zip target/x86_64-apple-darwin/release/softether-exporter
