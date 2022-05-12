#!/bin/sh
#


BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)
SOURCE_PATH=/data/source
DATA_PATH=/data/node-data/pangolin

cargo build --release --manifest-path ${SOURCE_PATH}/Cargo.toml

${SOURCE_PATH}/target/release/drml \
   --chain pangolin-dev \
   --base-path ${DATA_PATH} \
   --rpc-cors all \
   --unsafe-ws-external \
   --alice

