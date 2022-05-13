#!/bin/bash
#

set -xe

BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)
WORKSPACE=${BIN_PATH}/../../../

CONTAINER_POSTGRES=pp-postgres
CONTAINER_SUBQL_BUILD=subql-build

EXEC_PG="docker exec -it ${CONTAINER_POSTGRES}"
EXEC_SUBQL_BUILD="docker exec -it ${CONTAINER_SUBQL_BUILD}"

DATABASES='subql_s2s_pangolin subql_s2s_pangolin_parachain subql_s2s_rococo subql_parachain_rococo'


for DB in ${DATABASES}; do
  ${EXEC_PG} bash -c "
  psql -U postgres -tc \"select 1 from pg_database where datname = '${DB}'\" | grep -q 1 || psql -U postgres -c \"create database ${DB}\"
  "
done

docker stop ${CONTAINER_SUBQL_BUILD} || true
docker rm ${CONTAINER_SUBQL_BUILD} || true

docker run -dit \
  --rm \
  --name ${CONTAINER_SUBQL_BUILD} \
  -v ${WORKSPACE}/subql:/data \
  node:14

SUBQL_PROJECTS='s2s/pangolin s2s/pangolin-parachain s2s/rococo parachain/rococo'

for PROJ in ${SUBQL_PROJECTS}; do
  ${EXEC_SUBQL_BUILD} sh -c "cd /data/${PROJ} && npm i && npm run codegen && npm run build"
done

docker stop ${CONTAINER_SUBQL_BUILD} || true
