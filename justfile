#!/usr/bin/env -S just --justfile
# ^ A shebang isn't required, but allows a justfile to be executed
#   like a script, with `./justfile test`, for example.


test:
  cargo test

run:
  cargo run

build:
  cargo build

fmt:
  cargo fmt --all

clippy:
  cargo clippy --all --all-targets --all-features

