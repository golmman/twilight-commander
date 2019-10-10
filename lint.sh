#!/bin/sh

cargo clean --package twilight-commander
cargo clippy -- -W clippy::all
