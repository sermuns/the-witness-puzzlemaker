build-web:
  rm -fr dist
  cargo build --release --target wasm32-unknown-unknown --target-dir target
  mkdir -p dist
  cp ./target/wasm32-unknown-unknown/release/the-witness-puzzlemaker.wasm ./index.html ./static/* ./dist/

