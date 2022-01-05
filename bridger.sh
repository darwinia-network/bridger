#!/bin/sh
#

set -x

BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)

#export BRIDGER_HOME=/$HOME/.bridger

cargo build \
  --manifest-path $BIN_PATH/frame/Cargo.toml \
  --package darwinia-bridger || exit 1


$BIN_PATH/frame/target/debug/bridger $@
