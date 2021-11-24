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

cargo clean -p task-pangolin-pangoro --release
cargo clean -p task-darwinia-crab --release
cargo clean -p component-pangolin-s2s --release
cargo clean -p component-pangoro-s2s --release
cargo clean -p component-darwinia-s2s --release
cargo clean -p component-crab-s2s --release

cargo clean -p messages-relay --release
cargo clean -p relay-utils --release
cargo clean -p finality-relay --release
cargo clean -p substrate-relay-helper --release
cargo clean -p relay-substrate-client --release


