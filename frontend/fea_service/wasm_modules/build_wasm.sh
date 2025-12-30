cd ./actions_router && \
cargo build --release --no-default-features --target wasm32-unknown-unknown && \
wasm-bindgen --target web --out-name actions_router --out-dir ../../wasm/actions_router --no-typescript ./target/wasm32-unknown-unknown/release/actions_router.wasm && \
cd ../preprocessor && \
cargo build --release --no-default-features --target wasm32-unknown-unknown && \
wasm-bindgen --target web --out-name preprocessor --out-dir ../../wasm/preprocessor --no-typescript ./target/wasm32-unknown-unknown/release/preprocessor.wasm && \
cd ../renderer && \
cargo build --release --no-default-features --target wasm32-unknown-unknown && \
wasm-bindgen --target web --out-name renderer --out-dir ../../wasm/renderer --no-typescript ./target/wasm32-unknown-unknown/release/renderer.wasm && \
cd ../mesher && \
cargo build --release --no-default-features --target wasm32-unknown-unknown && \
wasm-bindgen --target web --out-name mesher --out-dir ../../wasm/mesher --no-typescript ./target/wasm32-unknown-unknown/release/mesher.wasm && \
cd ../solver && \
cargo build --release --no-default-features --target wasm32-unknown-unknown && \
wasm-bindgen --target no-modules --out-name solver --out-dir ../../wasm/solver --no-typescript ./target/wasm32-unknown-unknown/release/solver.wasm
