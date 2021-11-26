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

cargo clean --release -p darwinia-bridger

cargo clean --release -p crab-runtime
cargo clean --release -p darwinia-runtime
cargo clean --release -p pangolin-runtime
cargo clean --release -p pangoro-runtime

cargo clean --release -p task-pangolin-pangoro
cargo clean --release -p task-darwinia-crab
cargo clean --release -p component-pangolin-s2s
cargo clean --release -p component-pangoro-s2s
cargo clean --release -p component-darwinia-s2s
cargo clean --release -p component-crab-s2s

cargo clean --release -p messages-relay
cargo clean --release -p relay-utils
cargo clean --release -p finality-relay
cargo clean --release -p substrate-relay-helper
cargo clean --release -p relay-substrate-client


