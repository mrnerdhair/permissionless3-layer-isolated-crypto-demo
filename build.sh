#!/bin/sh -xe
cd "$(dirname "$0")"

pushd isolated-crypto
./build.sh
popd
pushd demo
./build.sh
popd

exec ./demo.sh "$@"
