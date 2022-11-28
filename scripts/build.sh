#!/bin/sh
#
set -xe

BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)
WORK_PATH=${BIN_PATH}/../

PACKAGE=$1
RELEASE=$2

_build() {
  MANIFEST=$1
  RELEASE=$2
  cargo build \
    --manifest-path ${MANIFEST} ${RELEASE}
}

if [ "${PACKAGE}" == "bridger" ]; then
  _build ${WORK_PATH}/frame/Cargo.toml
  exit 0
fi

if [ "${PACKAGE}" == "all" ]; then 
  BRIDGES=(darwinia-crab darwinia-ethereum pangolin-pangoro pangolin-pangolinparachain pangoro-chapel pangoro-goerli)
  for BRIDGE in "${BRIDGES[@]}"
  do
    BRIDGE_CARGO_TOML=${WORK_PATH}/bridges/${BRIDGE}/Cargo.toml
    _build ${BRIDGE_CARGO_TOML} ${RELEASE}
  done
  exit 0
fi

BRIDGE_CARGO_TOML=${WORK_PATH}/bridges/${PACKAGE}/Cargo.toml
if [ -f ${BRIDGE_CARGO_TOML} ]; then
  _build ${BRIDGE_CARGO_TOML} ${RELEASE}
fi


