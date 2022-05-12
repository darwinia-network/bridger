#!/bin/sh
#

BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)

ACCOUNT=$1
DATA_PATH=/data/node-data/rococo-${ACCOUNT}

polkadot build-spec --chain rococo-local --disable-default-bootnode --raw > ${DATA_PATH}/rococo-local-cfde.json

polkadot \
  --chain ${DATA_PATH}/rococo-local-cfde.json \
  --base-path ${DATA_PATH}/ \
  --rpc-cors all \
  --unsafe-ws-external \
  --port 30334 \
  --${ACCOUNT}


