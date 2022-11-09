#!/usr/bin/env bash
cargo publish --all-features --color always --jobs 1 --token ${'$'}CARGO_REGISTRY_TOKEN --verbose -p aqueduct