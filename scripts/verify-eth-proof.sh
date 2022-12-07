#!/bin/sh
#


URL="https://eth-mainnet.g.alchemy.com/v2/YXraeqSzO1wUUOD2WC51zLUyecVFwj6h"
#URL="https://1rpc.io/eth"
#URL="https://nodes.kriptonio.com/v1/endpoints/ethereum/mainnet/GvVjSGalMxFZPXNyk7ZA3VVF"
#URL="https://rpc.flashbots.net"

response0_0=$(curl $URL -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","method":"eth_getProof","params":["0x169F28bfbfFCddFdc772A94Cf020bbB4CAdc8E01",["0x0000000000000000000000000000000000000000000000000000000000000000"],"0xf52900"],"id":1}' 2>/dev/null)
echo $response0_0
response0_1=$(curl $URL -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","method":"eth_getProof","params":["0x169F28bfbfFCddFdc772A94Cf020bbB4CAdc8E01",["0x0000000000000000000000000000000000000000000000000000000000000000"],"0xf52900"],"id":2}' 2>/dev/null)
echo $response0_1
response1_0=$(curl $URL -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","method":"eth_getProof","params":["0x169F28bfbfFCddFdc772A94Cf020bbB4CAdc8E01",["0x0000000000000000000000000000000000000000000000000000000000000001"],"0xf52900"],"id":3}' 2>/dev/null)
echo $response1_0

if [[ "$response0_0" == "$response0_1" ]]; then
  echo "reponse0_0 equals to response0_1"
fi

if [[ "$response1_0" == "$response0_1" ]]; then
  echo "reponse1_0 should not equal to response0_1"
fi


curl https://eth-mainnet.g.alchemy.com/v2/YXraeqSzO1wUUOD2WC51zLUyecVFwj6h -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","method":"eth_getProof","params":["0x169F28bfbfFCddFdc772A94Cf020bbB4CAdc8E01",["0x0000000000000000000000000000000000000000000000000000000000000000","0x0000000000000000000000000000000000000000000000000000000000000001"],"0xf52900"],"id":3}'
