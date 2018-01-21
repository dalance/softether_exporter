VERSION = $(patsubst "%",%, $(word 3, $(shell grep version Cargo.toml)))
BUILD_TIME = $(shell date +"%Y/%m/%d %H:%M:%S")
GIT_REVISION = $(shell git log -1 --format="%h")

export BUILD_TIME
export GIT_REVISION

.PHONY: all test clean release_lnx release_win release_osx

all: test

test:
	cargo run

clean:
	cargo clean

release_lnx:
	cargo build --release --target=x86_64-unknown-linux-musl
	zip -j softether-exporter-v${VERSION}-x86_64-lnx.zip target/x86_64-unknown-linux-musl/release/softether-exporter

release_win:
	cargo build --release --target=x86_64-pc-windows-gnu
	zip -j softether-exporter-v${VERSION}-x86_64-win.zip target/x86_64-pc-windows-gnu/release/softether-exporter.exe

release_osx:
	cargo build --release --target=x86_64-apple-darwin
	zip -j softether-exporter-v${VERSION}-x86_64-osx.zip target/x86_64-apple-darwin/release/softether-exporter
