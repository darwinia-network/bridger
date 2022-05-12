#!/bin/sh
#


BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)
WORKSPACE=${BIN_PATH}/../../../

COMPOSE_FILE_TEMPLATE=${BIN_PATH}/docker-compose.template.yml
COMPOSE_FILE_RUN=${BIN_PATH}/docker-compose.yml


#DATA_DIR=${DATA_DIR:-/tmp/bridger/pangolin-pangolinparachain}
DATA_DIR=${DATA_DIR:-D:/dev/darwinia-network/_data/bridger/pangolin-pangolinparachain}

SUBQL_NODE_VERSION=v0.28.2
SUBQL_QUERY_VERSION=v0.12.0

SUBQL_PANGOLIN_DIR=${WORKSPACE}/subql/s2s/pangolin
SUBQL_PANGOLIN_PORT=13000

SUBQL_PANGOLIN_PARACHAIN_DIR=${WORKSPACE}/subql/s2s/pangolin-parachain
SUBQL_PANGOLIN_PARACHAIN_PORT=13001

SUBQL_ROCOCO_DIR=${WORKSPACE}/subql/s2s/rococo
SUBQL_ROCOCO_PORT=12002

ROCOCO_VERSION=v0.9.16
ROCOCO_ALICE_PORT_WS=19901
ROCOCO_BOB_PORT_WS=19902

PANGOLIN_PARACHAIN_SOURCE=D:/dev/darwinia-network/darwinia-common
PANGOLIN_PARACHAIN_PORT_WS=19701

PANGOLIN_SOURCE=D:/dev/darwinia-network/darwinia-common
PANGOLIN_PORT_WS=19801

_generate_docker_compose() {
  COMPOSE_CONTENT="$(_gen_compose_content)"
  echo "${COMPOSE_CONTENT}" > ${COMPOSE_FILE_RUN}
}

_gen_compose_content() {
  CONTENT=$(cat $COMPOSE_FILE_TEMPLATE)
  REGEX_SLASH='s/\//\\\//g'
  _BIN_PATH=$(echo ${BIN_PATH} | sed "${REGEX_SLASH}")
  _DATA_DIR=$(echo ${DATA_DIR} | sed "${REGEX_SLASH}")
  _SUBQL_PANGOLIN_DIR=$(echo ${SUBQL_PANGOLIN_DIR} | sed "${REGEX_SLASH}")
  _SUBQL_PANGOLIN_PARACHAIN_DIR=$(echo ${SUBQL_PANGOLIN_PARACHAIN_DIR} | sed "${REGEX_SLASH}")
  _SUBQL_ROCOCO_DIR=$(echo ${SUBQL_ROCOCO_DIR} | sed "${REGEX_SLASH}")
  _PANGOLIN_PARACHAIN_SOURCE=$(echo ${PANGOLIN_PARACHAIN_SOURCE} | sed "${REGEX_SLASH}")
  _PANGOLIN_SOURCE=$(echo ${PANGOLIN_SOURCE} | sed "${REGEX_SLASH}")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${BIN_PATH}/${_BIN_PATH}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${DATA_DIR}/${_DATA_DIR}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${SUBQL_NODE_VERSION}/${SUBQL_NODE_VERSION}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${SUBQL_QUERY_VERSION}/${SUBQL_QUERY_VERSION}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${SUBQL_PANGOLIN_DIR}/${_SUBQL_PANGOLIN_DIR}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${SUBQL_PANGOLIN_PORT}/${SUBQL_PANGOLIN_PORT}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${SUBQL_PANGOLIN_PARACHAIN_DIR}/${_SUBQL_PANGOLIN_PARACHAIN_DIR}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${SUBQL_PANGOLIN_PARACHAIN_PORT}/${SUBQL_PANGOLIN_PARACHAIN_PORT}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${SUBQL_ROCOCO_DIR}/${_SUBQL_ROCOCO_DIR}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${SUBQL_ROCOCO_PORT}/${SUBQL_ROCOCO_PORT}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${ROCOCO_VERSION}/${ROCOCO_VERSION}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${ROCOCO_ALICE_PORT_WS}/${ROCOCO_ALICE_PORT_WS}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${ROCOCO_BOB_PORT_WS}/${ROCOCO_BOB_PORT_WS}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${PANGOLIN_PARACHAIN_SOURCE}/${_PANGOLIN_PARACHAIN_SOURCE}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${PANGOLIN_PARACHAIN_PORT_WS}/${PANGOLIN_PARACHAIN_PORT_WS}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${PANGOLIN_SOURCE}/${_PANGOLIN_SOURCE}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${PANGOLIN_PORT_WS}/${PANGOLIN_PORT_WS}/g")
  echo "${CONTENT}"
}

main() {
  _generate_docker_compose
}

main
