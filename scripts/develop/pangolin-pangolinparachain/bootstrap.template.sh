#!/bin/bash
#
# Please copy this file to bootstrap.local.sh and modify your variables

set -xe

BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)

# Cargo home
export CARGO_HOME=${CARGO_HOME}
export RUSTUP_HOME=${RUSTUP_HOME}

# Data path, include pangolin/pangolin-parachain/rococo/bridger/subql data
export DATA_DIR=/tmp/bridger/pangolin-pangolinparachain
# Pangolin parachain source code path
export PANGOLIN_PARACHAIN_SOURCE=/path/to/darwinia-parachain
# Pangolin source code path
export PANGOLIN_SOURCE=/path/to/darwinia-common

export SUBQL_S2S_PANGOLIN_PORT=13000
export SUBQL_S2S_PANGOLIN_PARACHAIN_PORT=13001
export SUBQL_S2S_ROCOCO_PORT=13002
export SUBQL_PARACHAIN_ROCOCO_PORT=13003

export ROCOCO_ALICE_PORT_WS=19901
export ROCOCO_BOB_PORT_WS=19902

export PANGOLIN_PARACHAIN_PORT_WS=19701
export PANGOLIN_PORT_WS=19801

export BRIDGER_HOME=${BRIDGER_HOME:-${DATA_DIR}/bridger}

FORCE=$1

DOCKER_COMPOSE="docker-compose -f ${BIN_PATH}/docker-compose.yml"

if [ "${FORCE}" == "force" ]; then
  for FOLDER in $(ls ${DATA_DIR}); do
    if [ "$FOLDER" == "bridger" ]; then
      continue
    fi
    rm -rf ${FOLDER}
  done

  ${DOCKER_COMPOSE} up -d postgres

  ${BIN_PATH}/initialize.sh
fi

${BIN_PATH}/generate.sh

${DOCKER_COMPOSE} up
