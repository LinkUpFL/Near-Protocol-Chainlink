export RUSTFLAGS='-C link-arg=-s'
cargo build --all --target wasm32-unknown-unknown
