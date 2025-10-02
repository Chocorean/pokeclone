#!/bin/bash
#
# usage: bin/wasm
#
# note that it needs an additional index.html file

rm -rf ./out
cargo build --target wasm32-unknown-unknown;
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/debug/pokeclone.wasm;
cp assets/wasm/index.html out/
ln -s ../assets ./out/assets
python3 -m http.server -d ./out