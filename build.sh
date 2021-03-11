#!/usr/bin/env bash

set -ex

cargo update --aggressive
cargo build

./target/debug/odd_jobs ./odd_jobs.json > ./out/README.md
