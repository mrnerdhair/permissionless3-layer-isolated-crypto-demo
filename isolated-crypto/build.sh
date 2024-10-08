#!/bin/sh -ex
cd "$(dirname "$0")"

mkdir -p target
wkg wit build --output ./target/wit.wasm
wkg publish ./target/wit.wasm || true

pushd engine
cargo clean
cargo component build
popd

pushd provider
cargo clean
cargo component build
popd
