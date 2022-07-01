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
# Pangolin source code path
export PANGOLIN_SOURCE=/path/to/darwinia-common
# Pangolin source code path
export PANGORO_SOURCE=/path/to/darwinia-common

export SUBQL_S2S_PANGOLIN_PORT=13100
export SUBQL_S2S_PANGORO_PORT=13101

export PANGOLIN_PORT_WS=19101
export PANGORO_PORT_WS=191012

export BRIDGER_HOME=${BRIDGER_HOME:-${DATA_DIR}/bridger}

FORCE=$1

DOCKER_COMPOSE="docker-compose -f ${BIN_PATH}/docker-compose.yml"

${BIN_PATH}/generate.sh

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

${DOCKER_COMPOSE} up
