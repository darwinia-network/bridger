#!/bin/sh
#

set -e

BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)
WORK_PATH=${BIN_PATH}/../


SUBXT=$(which subxt || echo '')

if [ -z "${SUBXT}" ]; then
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
#  ENDPOINT='http://127.0.0.1:9966'
  OUTPUT_PATH=${WORK_PATH}/assistants/client-pangolin
fi

if [ "${CHAIN}" == "pangoro" ]; then
  ENDPOINT='https://pangoro-rpc.darwinia.network'
  OUTPUT_PATH=${WORK_PATH}/assistants/client-pangoro
fi

if [ "${CHAIN}" == "darwinia" ]; then
  ENDPOINT='http://g2.dev.darwinia.network:1133'
#  ENDPOINT='https://rpc.darwinia.network'
#  ENDPOINT='http://127.0.0.1:9936'
  OUTPUT_PATH=${WORK_PATH}/assistants/client-darwinia
fi

if [ "${CHAIN}" == "darwinia2" ]; then
  ENDPOINT='http://g1.dev.darwinia.network:10000'
  OUTPUT_PATH=${WORK_PATH}/assistants/client-darwinia
fi

if [ "${CHAIN}" == "pangoro2" ]; then
  ENDPOINT='http://g2.dev.darwinia.network:8888'
  OUTPUT_PATH=${WORK_PATH}/assistants/client-pangoro
fi

if [ "${CHAIN}" == "darwinia-parachain" ]; then
  ENDPOINT='http://127.0.0.1:9988'
  OUTPUT_PATH=${WORK_PATH}/assistants/client-darwinia-parachain
fi

if [ "${CHAIN}" == "rococo" ]; then
  ENDPOINT='https://rococo-rpc.polkadot.io'
  OUTPUT_PATH=${WORK_PATH}/assistants/client-rococo
fi

if [ "${CHAIN}" == "pangolin-parachain" ]; then
  ENDPOINT='https://pangolin-parachain-rpc.darwinia.network'
#  ENDPOINT='http://127.0.0.1:40338'
  OUTPUT_PATH=${WORK_PATH}/assistants/client-pangolin-parachain
fi

#if [ "${CHAIN}" == "pangolin-parachainalpha" ]; then
#  ENDPOINT='https://pangolin-parachain-alpha-rpc.darwinia.network'
#  OUTPUT_PATH=${WORK_PATH}/assistants/client-pangolin-parachainalpha
#fi

if [ "${CHAIN}" == "crab" ]; then
  ENDPOINT='http://g2.dev.darwinia.network:2233'
#  ENDPOINT='https://crab-rpc.darwinia.network'
  OUTPUT_PATH=${WORK_PATH}/assistants/client-crab
fi

if [ "${CHAIN}" == "crab-parachain" ]; then
  ENDPOINT='https://crab-parachain-rpc.darwinia.network'
  OUTPUT_PATH=${WORK_PATH}/assistants/client-crab-parachain
fi

if [ "${CHAIN}" == "kusama" ]; then
  ENDPOINT='http://g2.dev.darwinia.network:5533'
#  ENDPOINT='https://kusama-rpc.dwellir.com'
  OUTPUT_PATH=${WORK_PATH}/assistants/client-kusama
fi

if [ "${CHAIN}" == "moonbase" ]; then
  ENDPOINT='https://frag-moonbase-relay-rpc.g.moonbase.moonbeam.network'
  OUTPUT_PATH=${WORK_PATH}/assistants/client-moonbase
fi

if [ "${CHAIN}" == "polkadot" ]; then
  ENDPOINT='http://g2.dev.darwinia.network:3333'
#  ENDPOINT='https://rpc.polkadot.io'
  OUTPUT_PATH=${WORK_PATH}/assistants/client-polkadot
fi

if [ -z "${ENDPOINT}" ]; then
  echo 'Not support chain:' ${CHAIN}
  exit 1
fi



mkdir -p ${OUTPUT_PATH}

OUTPUT_FILE=${OUTPUT_PATH}/src/subxt_runtime/runtime.rs

${SUBXT} codegen --url ${ENDPOINT} ${DERIVE} > ${OUTPUT_FILE}

cargo fmt --manifest-path ${OUTPUT_PATH}/Cargo.toml -- ${OUTPUT_FILE} || true
