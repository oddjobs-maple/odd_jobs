#!/usr/bin/env bash

set -ex

cargo update --aggressive
cargo rustc --release -- -C target-cpu=native
strip ./target/release/oddjobs

./target/release/oddjobs ./oddjobs.json > ./out/README.md
