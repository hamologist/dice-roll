.PHONY: wasm

wasm:
	\
	cargo build \
		--release \
		-p dice-roll-wasm \
		--target wasm32-unknown-unknown && \
	wasm-bindgen target/wasm32-unknown-unknown/release/dice_roll_wasm.wasm \
		--out-name wasm \
		--out-dir pkg \
		--typescript \
		--target web && \
	wasm-opt pkg/wasm_bg.wasm \
		-o pkg/wasm_bg.wasm-opt.wasm \
		-O && \
	mv pkg/wasm_bg.wasm-opt.wasm pkg/wasm_bg.wasm;
