#!/bin/sh
#
# Please copy this file to bootstrap.local.sh and modify your variables

BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)

# Cargo home
export CARGO_HOME=${CARGO_HOME}

# Data path, include pangolin/pangolin-parachain/rococo/bridger/subql data
export DATA_DIR=/tmp/bridger/pangolin-pangolinparachain
# Pangolin parachain source code path
export PANGOLIN_PARACHAIN_SOURCE=/path/to/darwinia-parachain
# Pangolin source code path
export PANGOLIN_SOURCE=/path/to/darwinia-common

export SUBQL_PANGOLIN_PORT=13000
export SUBQL_PANGOLIN_PARACHAIN_PORT=13001
export SUBQL_ROCOCO_PORT=12002

export ROCOCO_ALICE_PORT_WS=19901
export ROCOCO_BOB_PORT_WS=19902

export PANGOLIN_PARACHAIN_PORT_WS=19701
export PANGOLIN_PORT_WS=19801
FORCE=$1
if [ "${FORCE}" == "true" ]; then 
  rm -rf ${DATA_DIR}
fi

sh -f ${BIN_PATH}/generate.sh

docker-compose -f ${BIN_PATH}/docker-compose.yml up 
