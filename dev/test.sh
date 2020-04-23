#!/bin/sh

RUST_BACKTRACE=full cargo test "$1" -- --show-output
