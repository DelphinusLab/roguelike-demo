wasm:
	wasm-pack build --release crates/core --out-name gameplay --out-dir pkg
	cp crates/core/pkg/gameplay.d.ts ../src/games/roguelike/js/gameplay.d.ts
	cp crates/core/pkg/gameplay_bg.wasm ../src/games/roguelike/js/gameplay_bg.wasm
	cp crates/core/pkg/gameplay_bg.wasm.d.ts ../src/games/roguelike/js/gameplay_bg.wasm.d.ts
	cp crates/core/pkg/gameplay_bg.js ../src/games/roguelike/js/gameplay_bg.js

lib:
	cargo build --release

clean:
	rm -rf crates/core/pkg
	rm -rf ../src/games/roguelike/js/gameplay.d.ts
	rm -rf ../src/games/roguelike/js/gameplay.wasm_bg.js
