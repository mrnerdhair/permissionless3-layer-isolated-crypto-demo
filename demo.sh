#!/bin/sh
cd "$(dirname "$0")"

MSG="$1"
[ -z "$MSG" ] && MSG='Example `personal_sign` message'

[ -z "$MNEMONIC" ] && MNEMONIC="zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote"

avs-toolkit-cli wasmatic run --wasm-source ./target/demo.wasm \
    --envs "MNEMONIC=$MNEMONIC" \
    --input "$MSG"
