#!/bin/bash
#


BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)
SOURCE_PATH=/data/source
DATA_PATH=/data/node-data/pangolin-parachain
mkdir -p ${DATA_PATH}
DARWINIA_COLLATOR=${SOURCE_PATH}/target/release/darwinia-collator

if [ ! -f ${DARWINIA_COLLATOR} ]; then
  apt update -y
  apt install -y libclang-dev
fi

cd ${SOURCE_PATH}
cargo build --release

CHAIN=pangolin-parachain-dev

${DARWINIA_COLLATOR} export-genesis-wasm --chain ${CHAIN} > ${DATA_PATH}/para-2000-wasm.data
${DARWINIA_COLLATOR} export-genesis-state --chain ${CHAIN} > ${DATA_PATH}/para-2000-state.data

${DARWINIA_COLLATOR} \
  --force-authoring \
  --collator \
  --chain ${CHAIN} \
  --base-path ${DATA_PATH} \
  --rpc-cors all \
  --unsafe-ws-external \
  --alice \
  -- \
  --chain ${DATA_PATH}/rococo-alice/rococo-local-cfde.json \
  --port 30334

