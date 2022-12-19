#!/usr/bin/env bash
rustup install nightly
rustup component add clippy rustfmt --toolchain nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
