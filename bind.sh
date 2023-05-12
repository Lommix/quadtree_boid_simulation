cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./www/out/ --target web ./target/wasm32-unknown-unknown/release/boids-quadtree.wasm
