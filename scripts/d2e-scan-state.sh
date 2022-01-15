#!/bin/sh
#
set -xe

BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)
WORK_PATH=${BIN_PATH}/../
BRIDGER=${WORK_PATH}/bridger.sh

NAMESPACE=$1

KV="${BRIDGER} ${NAMESPACE} kv"

${KV} get -o table --include-key \
  scan.pangolin.running \
  scan.ropsten.check.running \
  scan.ropsten.check.current \
  scan.ropsten.check.planned \
  scan.ropsten.redeem.running \
  scan.ropsten.redeem.current \
  scan.ropsten.redeem.planned \
  scan.ropsten.affirm.running \
  scan.ropsten.affirm.current \
  scan.ropsten.affirm.planned


