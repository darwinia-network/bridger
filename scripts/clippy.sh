#!/bin/sh
#
set -xe

BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)
WORK_PATH=${BIN_PATH}/../

PACKAGE=$1


if [ "${PACKAGE}" == "bridger" ]; then
  cargo clippy --manifest-path ${WORK_PATH}/frame/Cargo.toml \
    --all -- -D warnings
fi

if [ "${PACKAGE}" == "pangolin-ropsten" ]; then
  cargo clippy --manifest-path ${WORK_PATH}/bridges/pangolin-ropsten/Cargo.toml \
    --all -- -D warnings
fi

if [ "${PACKAGE}" == "darwinia-ethereum" ]; then
  cargo clippy --manifest-path ${WORK_PATH}/bridges/darwinia-ethereum/Cargo.toml \
    --all -- -D warnings
fi

if [ "${PACKAGE}" == "pangolin-pangoro" ]; then
  cargo clippy --manifest-path ${WORK_PATH}/bridges/pangolin-pangoro/Cargo.toml \
    --all -- -D warnings
fi

if [ "${PACKAGE}" == "darwinia-crab" ]; then
  cargo clippy --manifest-path ${WORK_PATH}/bridges/darwinia-crab/Cargo.toml \
    --all -- -D warnings
fi




