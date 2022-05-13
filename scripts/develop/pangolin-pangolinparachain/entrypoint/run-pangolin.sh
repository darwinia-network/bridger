#!/bin/sh
#


BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)
SOURCE_PATH=/data/source
DATA_PATH=/data/node-data/pangolin
mkdir -p ${DATA_PATH}
DRML=${SOURCE_PATH}/target/release/drml

if [ ! -f ${DRML} ]; then
  apt update -y
  apt install -y libclang-dev
fi

cargo build --release --manifest-path ${SOURCE_PATH}/Cargo.toml

${DRML} \
   --chain pangolin-dev \
   --base-path ${DATA_PATH} \
   --rpc-cors all \
   --unsafe-ws-external \
   --alice

