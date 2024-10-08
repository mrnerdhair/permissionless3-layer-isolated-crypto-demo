#!/bin/sh -xe
cd "$(dirname "$0")"
mkdir -p ./target

pushd isolated-crypto
./build.sh
popd
pushd demo
./build.sh
popd

wac plug \
    --plug ./isolated-crypto/provider/target/wasm32-wasip1/debug/isolated_crypto_provider.wasm \
    ./demo/target/wasm32-wasip1/debug/isolated_crypto_demo.wasm \
    --output ./target/demo-no-engine.wasm

wac plug \
    --plug ./isolated-crypto/engine/target/wasm32-wasip1/debug/isolated_crypto_engine.wasm \
    ./target/demo-no-engine.wasm \
    --output ./target/demo.wasm

avs-toolkit-cli wasmatic run --wasm-source ./target/demo.wasm \
    --envs 'MNEMONIC=zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote' \
    --input 'Example `personal_sign` message'
