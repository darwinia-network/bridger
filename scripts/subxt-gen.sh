#!/bin/sh
#

set -e

BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)
WORK_PATH=${BIN_PATH}/../


SUBXT=$(which subxt || echo '')

if [ -z "${SUBXT}" ]; then
#  cargo install --branch parity-master --git https://github.com/darwinia-network/subxt
  cargo install --tag v0.18.1 --git https://github.com/paritytech/subxt
fi

SUBXT=$(which subxt)


CHAIN=$1


if [ -z "${CHAIN}" ]; then
  echo 'Missing chain'
  exit 1
fi

DERIVE='--derive Clone'
ENDPOINT=
OUTPUT_FILE=



if [ "${CHAIN}" == "pangolin" ]; then
  ENDPOINT='https://pangolin-rpc.darwinia.network'
  OUTPUT_PATH=${WORK_PATH}/frame/assistants/pangolin-subxt
fi

if [ "${CHAIN}" == "pangoro" ]; then
  ENDPOINT='https://pangoro-rpc.darwinia.network'
  OUTPUT_PATH=${WORK_PATH}/frame/assistants/pangoro-subxt
fi

if [ "${CHAIN}" == "darwinia" ]; then
#  ENDPOINT='https://rpc.darwinia.network'
  ENDPOINT='http://127.0.0.1:9936'
  OUTPUT_PATH=${WORK_PATH}/frame/assistants/darwinia-subxt
fi



if [ -z "${ENDPOINT}" ]; then
  echo 'Not support chain:' ${CHAIN}
  exit 1
fi



mkdir -p ${OUTPUT_PATH}

OUTPUT_FILE=${OUTPUT_PATH}/src/runtime.rs

${SUBXT} codegen --url ${ENDPOINT} ${DERIVE} > ${OUTPUT_FILE}

cargo fmt --manifest-path ${OUTPUT_PATH}/Cargo.toml -- ${OUTPUT_FILE} || true
