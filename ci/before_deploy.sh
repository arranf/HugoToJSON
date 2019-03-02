#!/bin/bash
set -euo pipefail

# Compile the binary for the current target
cargo build --release

# Package up the release binary
tar -C target/release -czf hugo_to_json-$TRAVIS_TAG.tar.gz hugo_to_json