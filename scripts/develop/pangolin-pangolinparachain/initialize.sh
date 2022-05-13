#!/bin/bash
#

set -xe

BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)
WORKSPACE=${BIN_PATH}/../../../

CONTAINER_POSTGRES=pp-postgres
CONTAINER_SUBQL_BUILD=subql-build

EXEC_PG="docker exec -it ${CONTAINER_POSTGRES}"
EXEC_SUBQL_BUILD="docker exec -it ${CONTAINER_SUBQL_BUILD}"


${EXEC_PG} psql -U postgres -c 'create database subql_s2s_pangolin'
${EXEC_PG} psql -U postgres -c 'create database subql_s2s_pangolin_parachain'
${EXEC_PG} psql -U postgres -c 'create database subql_s2s_rococo'
${EXEC_PG} psql -U postgres -c 'create database subql_parachain_rococo'

docker stop ${CONTAINER_SUBQL_BUILD} || true
docker rm ${CONTAINER_SUBQL_BUILD} || true

docker run -dit \
  --rm \
  --name ${CONTAINER_SUBQL_BUILD} \
  -v ${WORKSPACE}/subql:/data \
  node:14

docker run -dit \
  --rm \
  --name ${CONTAINER_SUBQL_BUILD} \
  -v ${PWD}/subql:/data \
  node:14

${EXEC_SUBQL_BUILD} sh -c "cd /data/s2s/pangolin && npm i && npm run codegen && npm run build"
${EXEC_SUBQL_BUILD} sh -c "cd /data/s2s/pangolin-parachain && npm i && npm run codegen && npm run build"
${EXEC_SUBQL_BUILD} sh -c "cd /data/s2s/rococo && npm i && npm run codegen && npm run build"
${EXEC_SUBQL_BUILD} sh -c "cd /data/parachain/rococo && npm i && npm run codegen && npm run build"

docker stop ${CONTAINER_SUBQL_BUILD}
docker rm ${CONTAINER_SUBQL_BUILD}
