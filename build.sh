export PKG_CONFIG_ALLOW_CROSS=1
export OPENSSL_STATIC=true
export OPENSSL_DIR=/musl

cargo build --release --target x86_64-unknown-linux-musl
strip -s ./target/x86_64-unknown-linux-musl/release/desa
upx --best ./target/x86_64-unknown-linux-musl/release/desa
ls -hl ./target/x86_64-unknown-linux-musl/release/