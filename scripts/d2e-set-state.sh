#!/bin/sh
#
set -xe

BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)
WORK_PATH=${BIN_PATH}/../
BRIDGER=${WORK_PATH}/bridger.sh

NAMESPACE=$1

KV="${BRIDGER} ${NAMESPACE} kv"

${KV} keys

BLOCK_REDEEM=11933702
BLOCK_AFFIRM=11933702
BLOCK_CHECK=11933702
BLOCK_DARWINIA=1040573

if [ "${NAMESPACE}" == "pangolin-ropsten" ]; then

  ${KV} put \
    scan.ropsten.redeem.planned ${BLOCK_REDEEM} \
    scan.ropsten.affirm.planned ${BLOCK_AFFIRM} \
    scan.ropsten.check.planned ${BLOCK_CHECK} \
    scan.pangolin.planned ${BLOCK_DARWINIA} \

  ${KV} put \
    scan.pangolin.running true \
    scan.ropsten.check.running true \
    scan.ropsten.redeem.running true \
    scan.ropsten.affirm.running true

fi

sh -f ${BIN_PATH}/d2e-scan-state.sh ${NAMESPACE}


