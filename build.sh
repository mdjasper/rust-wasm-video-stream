#!/bin/sh

# For more comments about what's going on here, see the `hello_world` example

set -ex
cd "$(dirname $0)"

cargo +nightly build --target wasm32-unknown-unknown --release

wasm-bindgen \
  target/wasm32-unknown-unknown/debug/wasm_bindgen_lib.wasm --out-dir .

npm install
npm run serve
