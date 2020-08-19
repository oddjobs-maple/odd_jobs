#!/usr/bin/env bash

set -ex

cargo build --release
strip ./target/release/oddjobs
./target/release/oddjobs ./oddjobs.json > ./out/README.md
