# Draw Bevy

### Running
```
cargo build --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./web/src/bevy/ --target web ./target/wasm32-unknown-unknown/debug/draw-bevy.wasm
cd web
npm i 
npm run dev
```

### Building
```
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./web/src/bevy/ --target web ./target/wasm32-unknown-unknown/release/draw-bevy.wasm
mv ./web/src/bevy/draw-bevy_bg.wasm ./web/src/bevy/draw-bevy_bg_unoptimized.wasm
wasm-opt -Oz ./web/src/bevy/draw-bevy_bg_unoptimized.wasm -o ./web/src/bevy/draw-bevy_bg.wasm
cd web
npm i 
npm run build
```