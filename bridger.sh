#!/bin/sh
#

set -x

BIN_PATH=$(dirname $(readlink -f $0))

#export BRIDGER_HOME=D:/dev/darwinia-network/_data/bridger

cargo build \
  --manifest-path $BIN_PATH/frame/Cargo.toml \
  --package darwinia-bridger || exit 1


$BIN_PATH/frame/target/debug/bridger $@
