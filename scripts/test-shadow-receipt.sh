#!/bin/sh
#


set -e

BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)

mkdir -p ${BIN_PATH}/logs

TX='0xe565921b5193261af60800d9d9a641467590e101b3c8dc23f7160a52296a52b8'
LAST_CONFIRMED='12244903'

curl https://ropsten.shadow.darwinia.network/ethereum/receipt/${TX}/${LAST_CONFIRMED} \
 | jq '.' | tee ${BIN_PATH}/logs/shadow-receipt.json

