#!/bin/sh
#
set -xe

BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)
WORK_PATH=${BIN_PATH}/../

_ENDPOINT=$1
ENDPOINT=${_ENDPOINT:-'https://pangolin-rpc.darwinia.network'}

SUBXT=$(which subxt)

if [ -z "${SUBXT}" ]; then
  cargo install subxt-cli
fi

METADATA_SCALA=${WORK_PATH}/metadata.scale
OUTPUT_PATH=${WORK_PATH}/src/codegen
OUTPUT_FILE=${OUTPUT_PATH}/runtime.rs

mkdir -p ${OUTPUT_PATH}

${SUBXT} metadata --url ${ENDPOINT} -f bytes > ${METADATA_SCALA}

${SUBXT} codegen -f ${METADATA_SCALA} > ${OUTPUT_FILE}

cargo fmt -- ${OUTPUT_FILE} || true

rm -rf ${METADATA_SCALA}

