#!/bin/sh
#

set -x

BIN_PATH=$(dirname $(readlink -f $0))

cargo build \
  --manifest-path $BIN_PATH/frame/Cargo.toml \
  --package darwinia-bridger


$BIN_PATH/frame/target/debug/bridger $@
