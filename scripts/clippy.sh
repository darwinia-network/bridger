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
fi

if [ "${PACKAGE}" == "pangolin-ropsten" ]; then
  _clippy ${WORK_PATH}/bridges/pangolin-ropsten/Cargo.toml \
    --locked
fi

if [ "${PACKAGE}" == "darwinia-ethereum" ]; then
  _clippy ${WORK_PATH}/bridges/darwinia-ethereum/Cargo.toml \
    --locked
fi

if [ "${PACKAGE}" == "pangolin-pangoro" ]; then
  _clippy --manifest-path ${WORK_PATH}/bridges/pangolin-pangoro/Cargo.toml \
    --locked
fi

if [ "${PACKAGE}" == "darwinia-crab" ]; then
  _clippy ${WORK_PATH}/bridges/darwinia-crab/Cargo.toml \
    --locked
fi


