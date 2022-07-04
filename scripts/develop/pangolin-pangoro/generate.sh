#!/bin/bash
#


BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)
WORKSPACE=${BIN_PATH}/../../../

_abs_path() {
  INPUT=$1
  OUTPUT=$(cd ${INPUT}; pwd -P)
  echo $OUTPUT
}

CARGO_HOME=${CARGO_HOME:-/tmp/cargo}
RUSTUP_HOME=${RUSTUP_HOME:-/tmp/rustup}

COMPOSE_FILE_TEMPLATE=${BIN_PATH}/docker-compose.template.yml
COMPOSE_FILE_RUN=${BIN_PATH}/docker-compose.yml


DATA_DIR=${DATA_DIR:-/tmp/bridger/pangolin-pangoro}
BRIDGER_HOME=${BRIDGER_HOME:-${DATA_DIR}/bridger}

SUBQL_NODE_VERSION=${SUBQL_NODE_VERSION:-v0.28.2}
SUBQL_QUERY_VERSION=${SUBQL_QUERY_VERSION:-v0.12.0}

SUBQL_S2S_PANGOLIN_DIR=$(_abs_path ${WORKSPACE}/subql/s2s/pangolin)
SUBQL_S2S_PANGOLIN_PORT=${SUBQL_S2S_PANGOLIN_PORT:-13100}

SUBQL_S2S_PANGORO_DIR=$(_abs_path ${WORKSPACE}/subql/s2s/pangoro)
SUBQL_S2S_PANGORO_PORT=${SUBQL_S2S_PANGORO_PORT:-13101}

PANGOLIN_SOURCE=${PANGOLIN_SOURCE}
PANGOLIN_PORT_WS=${PANGOLIN_PORT_WS:-19101}

PANGORO_SOURCE=${PANGORO_SOURCE}
PANGORO_PORT_WS=${PANGORO_PORT_WS:-191012}

_generate_docker_compose() {
  COMPOSE_CONTENT="$(_gen_compose_content)"
  echo "${COMPOSE_CONTENT}" > ${COMPOSE_FILE_RUN}
}

_gen_compose_content() {
  CONTENT=$(cat $COMPOSE_FILE_TEMPLATE)
  REGEX_SLASH='s/\//\\\//g'
  _CARGO_HOME=$(echo ${CARGO_HOME} | sed "${REGEX_SLASH}")
  _RUSTUP_HOME=$(echo ${RUSTUP_HOME} | sed "${REGEX_SLASH}")
  _BIN_PATH=$(echo ${BIN_PATH} | sed "${REGEX_SLASH}")
  _DATA_DIR=$(echo ${DATA_DIR} | sed "${REGEX_SLASH}")
  _BRIDGER_HOME=$(echo ${BRIDGER_HOME} | sed "${REGEX_SLASH}")
  _SUBQL_S2S_PANGOLIN_DIR=$(echo ${SUBQL_S2S_PANGOLIN_DIR} | sed "${REGEX_SLASH}")
  _SUBQL_S2S_PANGORO_DIR=$(echo ${SUBQL_S2S_PANGOLIN_DIR} | sed "${REGEX_SLASH}")
  _PANGOLIN_SOURCE=$(echo ${PANGOLIN_SOURCE} | sed "${REGEX_SLASH}")
  _PANGORO_SOURCE=$(echo ${PANGORO_SOURCE} | sed "${REGEX_SLASH}")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${CARGO_HOME}/${_CARGO_HOME}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${RUSTUP_HOME}/${_RUSTUP_HOME}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${BIN_PATH}/${_BIN_PATH}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${BRIDGER_HOME}/${_BRIDGER_HOME}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${DATA_DIR}/${_DATA_DIR}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${SUBQL_NODE_VERSION}/${SUBQL_NODE_VERSION}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${SUBQL_QUERY_VERSION}/${SUBQL_QUERY_VERSION}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${SUBQL_S2S_PANGOLIN_DIR}/${_SUBQL_S2S_PANGOLIN_DIR}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${SUBQL_S2S_PANGOLIN_PORT}/${SUBQL_S2S_PANGOLIN_PORT}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${SUBQL_S2S_PANGORO_DIR}/${_SUBQL_S2S_PANGORO_DIR}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${SUBQL_S2S_PANGORO_PORT}/${SUBQL_S2S_PANGORO_PORT}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${PANGOLIN_SOURCE}/${_PANGOLIN_SOURCE}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${PANGOLIN_PORT_WS}/${PANGOLIN_PORT_WS}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${PANGORO_SOURCE}/${_PANGORO_SOURCE}/g")
  CONTENT=$(echo "${CONTENT}" | sed "s/\${PANGORO_PORT_WS}/${PANGORO_PORT_WS}/g")
  echo "${CONTENT}"
}

main() {
  _generate_docker_compose
  echo "Generated -> ${BIN_PATH}/docker-compose.yml"
}

main
