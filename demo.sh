#!/bin/sh
cd "$(dirname "$0")"
mkdir -p ./target

cargo component build --release

MSG="$1"
[ -z "$MSG" ] && MSG='Example `personal_sign` message'

[ -z "$MNEMONIC" ] && MNEMONIC="zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote"

wac plug \
    --plug ./target/wasm32-wasip1/release/isolated_crypto_provider.wasm \
    ./target/wasm32-wasip1/release/isolated_crypto_demo.wasm \
    --output ./target/wasm32-wasip1/release/demo-no-engine.wasm

wac plug \
    --plug ./target/wasm32-wasip1/release/isolated_crypto_engine.wasm \
    ./target/wasm32-wasip1/release/demo-no-engine.wasm \
    --output ./target/wasm32-wasip1/release/demo.wasm

avs-toolkit-cli wasmatic run --wasm-source ./target/wasm32-wasip1/release/demo.wasm \
    --envs "MNEMONIC=$MNEMONIC" \
    --input "$MSG"
