#!/bin/bash

set -euxo pipefail

target="$1"

tempdir="$(mktemp -d)"
builtin pushd "${tempdir}"

url="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"
curl --retry 3 --proto '=https' --tlsv1.2 -sSf "$url" > rustup-init
chmod +x rustup-init

./rustup-init --no-modify-path --default-toolchain stable --profile minimal -y

chmod -R a+w "$RUSTUP_HOME" "$CARGO_HOME"
rustup target add "$target"

builtin popd
rm -rf tempdir
