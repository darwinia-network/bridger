#!/bin/sh
#
set -xe

BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)
WORK_PATH=${BIN_PATH}/../

_ENDPOINT=$1
ENDPOINT=${_ENDPOINT:-'https://pangolin-rpc.darwinia.network'}

SUBXT=$(which subxt)

if [ -z "${SUBXT}" ]; then
#  cargo install subxt-cli
  echo 'The subxt command not found.'
  exit 1
fi

OUTPUT_PATH=${WORK_PATH}/src
OUTPUT_FILE=${OUTPUT_PATH}/runtime.rs

mkdir -p ${OUTPUT_PATH}

${SUBXT} codegen --url ${ENDPOINT} --derive Clone > ${OUTPUT_FILE}

cargo fmt -- ${OUTPUT_FILE} || true

