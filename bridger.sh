#!/bin/sh
#

set -x

BIN_PATH=$(dirname $(readlink -f $0))

cargo run \
  --manifest-path $BIN_PATH/Cargo.toml \
  --package darwinia-bridger \
  --bin bridger \
  -- $@
