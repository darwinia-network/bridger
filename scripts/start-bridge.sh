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

rm -rf ${LOG_FILE} || true
${BRIDGER} ${BRIDGE} start | tee ${LOG_FILE}

