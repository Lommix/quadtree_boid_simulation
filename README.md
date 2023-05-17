
# Boids with Quadtree using Bevy and Rust

![(screenshot)](./docs/screen.png)

Quadtree implementation together with a boid simulation written in rust, using the bevy framework.


## Run

```
cargo run --release
```

## Controls

```
Left click : Add 100 Boids in cursor rectangle
Right click: Remove at cursor rectangle
```


## Export to wasm

There is simple shell script to export wasm bindings to files. Node dev setup is inside the www folder.

```
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./www/out/ --target web ./target/wasm32-unknown-unknown/release/boids-quadtree.wasm

cd www
npm install
npm run dev
```
