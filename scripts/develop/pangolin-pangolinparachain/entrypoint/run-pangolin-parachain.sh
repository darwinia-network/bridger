#!/bin/sh
#


BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)
SOURCE_PATH=/data/source
DATA_PATH=/data/node-data/pangolin-parachain

cargo build --release --manifest-path ${SOURCE_PATH}/Cargo.toml

CHAIN=pangolin-parachain-dev
DARWINIA_COLLATOR=${SOURCE_PATH}/target/release/darwinia-collator

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

