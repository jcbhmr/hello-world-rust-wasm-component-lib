build:
	cargo component build --target wasm32-unknown-unknown

setup:
	rustup target add wasm32-unknown-unknown
	command -v cargo-binstall || curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
	command -v cargo-component || cargo binstall cargo-component -y
	npm install -g @bytecodealliance/jco

bindings-js:
	rm -rf target/js
	jco transpile \
		target/wasm32-unknown-unknown/debug/hello_world_rust_wasm_component_lib.wasm \
		-o target/js \
		--map "jcbhmr:hello-world-rust-wasm-component-lib/*=../../tests/*.js" \
		$(JCOFLAGS)
	git -C target/js init
	echo '{"type":"module","exports":"./hello_world_rust_wasm_component_lib.js","dependencies":{"@bytecodealliance/preview2-shim":"latest"}}' > target/js/package.json
	(cd target/js; npm install)
	git -C target/js add -Af
	git -C target/js commit -m "Initial commit"
	(cd target/js; patch -p1 < ../../bindings-js.patch)

diff-bindings-js:
	find target/js -type f -name '*.orig' -delete
	find target/js -type f -name '*.rej' -delete
	git -C target/js diff > bindings-js.patch

test-bindings-js:
	node --experimental-default-type=module --test

test-bindings-rs:
	cargo test -- --show-output

test: test-bindings-js test-bindings-rs
