#!/bin/sh
#
set -xe

BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)
WORK_PATH=${BIN_PATH}/../

PACKAGE=$1

_clippy() {
  MANIFEST=$1
  LOCKED=$2
  cargo clippy \
    --manifest-path ${MANIFEST} ${LOCKED} \
    --all -- -D warnings
}

if [ "${PACKAGE}" == "bridger" ]; then
  _clippy ${WORK_PATH}/frame/Cargo.toml
  exit 0
fi

BRIDGE_CARGO_TOML=${WORK_PATH}/bridges/${PACKAGE}/Cargo.toml

if [ -f ${BRIDGE_CARGO_TOML} ]; then
  _clippy ${BRIDGE_CARGO_TOML} --locked
fi


