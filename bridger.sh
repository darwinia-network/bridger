#!/bin/sh
#

set -xe

BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)

#export BRIDGER_HOME=$HOME/.bridger

export RUST_SPANTRACE=1
#export RUST_LIB_BACKTRACE=full

cargo build \
  --manifest-path $BIN_PATH/frame/Cargo.toml \
  --package darwinia-bridger


$BIN_PATH/frame/target/debug/bridger $@
