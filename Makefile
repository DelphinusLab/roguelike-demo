wasm:
	wasm-pack build --target web --release crates/core --out-dir '../../pkg'

lib:
	cargo build --release
