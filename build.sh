#!/usr/bin/env bash

set -ex

cargo update --aggressive
cargo build

./target/debug/oddjobs ./oddjobs.json > ./out/README.md
