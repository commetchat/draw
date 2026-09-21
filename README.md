
# Draw

<p align="left">
    <a href="https://commet.chat/donate"><img alt="Donate" src="https://img.shields.io/badge/donate-534cdd?style=for-the-badge"></a>
    <a href="https://matrix.to/#/#commet:matrix.org"><img alt="Matrix" src="https://img.shields.io/matrix/commet%3Amatrix.org?logo=matrix&style=for-the-badge&color=534cdd"></a>
    <a href="https://fosstodon.org/@commetchat"><img alt="Mastodon" src="https://img.shields.io/mastodon/follow/109894490854601533?domain=https%3A%2F%2Ffosstodon.org&style=for-the-badge&logo=mastodon&color=534cdd&logoColor=white"></a>
    <a href="https://bsky.app/profile/commet.chat"><img alt="Bluesky" src="https://img.shields.io/badge/follow-@commet.chat-whitesmoke?style=for-the-badge&logo=bluesky&logoColor=white&color=534cdd"></a>
</p>

### Draw together on Matrix
A collaborative drawing game built on top of MatrixRTC, draw with your friends on a persistent infinite canvas and create something beautiful!

<img width="1368" height="826" alt="Image" src="https://github.com/user-attachments/assets/e8a67e0d-db2b-4020-a91a-6e8df07221ca" />

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
