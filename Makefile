build:
	cargo component build --target wasm32-unknown-unknown

setup:
	rustup target add wasm32-unknown-unknown
	command -v cargo-binstall || curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
	command -v cargo-component || cargo binstall cargo-component -y
	npm install -g @bytecodealliance/jco

bindings-js:
	rm -rf target/js/test
	jco transpile \
		target/wasm32-unknown-unknown/debug/hello_world_rust_wasm_component_lib.wasm \
		-o target/js/test \
		--map "jcbhmr:hello-world-rust-wasm-component-lib/*=../../../tests/*.js" \
		$(JCOFLAGS)
	git -C target/js/test init
	echo '{"type":"module","exports":"./hello_world_rust_wasm_component_lib.js","dependencies":{"@bytecodealliance/preview2-shim":"latest"}}' > target/js/test/package.json
	(cd target/js/test; npm install)
	git -C target/js/test add -Af
	git -C target/js/test commit -m "Initial commit"
	(cd target/js/test; patch -p1 < ../../../bindings-js.patch)

diff-bindings-js:
	find target/js/test -type f -name '*.orig' -delete
	find target/js/test -type f -name '*.rej' -delete
	git -C target/js/test diff > bindings-js.patch

test-bindings-js:
	node --experimental-default-type=module --test

test-bindings-rs:
	cargo test

test: test-bindings-js test-bindings-rs
