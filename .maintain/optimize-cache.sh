#!/bin/sh
#

set -x

BIN_PATH=$(dirname $(readlink -f $0))
WORK_PATH=${BIN_PATH}/../

cd ${WORK_PATH}


rm -rf target/release/wbuild
rm -rf target/release/build
rm -rf target/release/bridger.pdb
rm -rf target/release/bridger
rm -rf target/release/incremental

cargo clean -p darwinia-bridger --release

cargo clean -p crab-runtime --release
cargo clean -p darwinia-runtime --release
cargo clean -p pangolin-runtime --release
cargo clean -p pangoro-runtime --release

