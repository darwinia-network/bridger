#!/bin/bash
#


BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)
SOURCE_PATH=/data/source
DATA_PATH=/data/node-data/pangoro
mkdir -p ${DATA_PATH}
DRML=${SOURCE_PATH}/target/release/drml

if [ ! -f ${DRML} ]; then
  apt update -y
  apt install -y libclang-dev
fi

cd ${SOURCE_PATH}
cargo build --release

${DRML} \
   --chain pangoro-dev \
   --base-path ${DATA_PATH} \
   --rpc-cors all \
   --unsafe-ws-external \
   --alice

