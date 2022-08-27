#!/usr/bin/env zsh

mkdir -p mount
cargo build && sudo ./target/debug/crabcan --debug -u 0 -m ./mount/ -c "/bin/bash"
