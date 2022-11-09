#!/usr/bin/env bash

sudo apt update -y && sudo apt upgrade -y && sudo apt autoremove -y
sudo apt install -y protobuf-compiler
rustup install nightly
rustup component add clippy rustfmt --toolchain nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
