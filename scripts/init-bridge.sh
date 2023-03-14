#!/bin/sh
#
set -xe

BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)
WORK_PATH=${BIN_PATH}/../
BRIDGER=${WORK_PATH}/bridger.sh

LOG_PATH=${BIN_PATH}/logs
BRIDGE=$1

if [ -z "${BRIDGE}" ]; then
  echo 'Missing bridge. e.g. ./start-bridge.sh pangolin-ropsten'
  exit 1
fi

mkdir -p ${LOG_PATH}


LOG_FILE=${LOG_PATH}/${BRIDGE}.log

cargo build \
  --manifest-path ${WORK_PATH}/bridges/${BRIDGE}/Cargo.toml

rm -rf ${LOG_FILE} || true
${WORK_PATH}/bridges/${BRIDGE}/target/debug/bridge-${BRIDGE} init $2

