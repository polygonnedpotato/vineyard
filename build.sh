#!/bin/bash

mkdir -p ./test/
touch ./test/vineyard.pid

cargo build -vv

RUST_BACKTRACE=full ./target/debug/vineyard