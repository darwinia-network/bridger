#!/bin/bash
#

set -xe

BIN_PATH=$(dirname $(readlink -f $0))
WORK_PATH=${BIN_PATH}/../

echo -e '\e[1;32m📥 Installing Cross Compile Toolchain(s)\e[0m'
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
  sh -s -- -y --profile minimal --default-toolchain ${RUST_TOOLCHAIN}
source ~/.cargo/env

echo -e '\e[1;32m🔧 Building Docker Image(s)\e[0m'
docker build -f .maintain/docker/Dockerfile.x86_64-linux-gnu -t x86_64-linux-gnu .

cargo install cross --git https://github.com/l2ust/cross

rustup target add \
  x86_64-unknown-linux-gnu \
  aarch64-unknown-linux-gnu \
  wasm32-unknown-unknown

mkdir -p ${WORK_PATH}/deploy/bin

cross build \
  --release \
  --target x86_64-unknown-linux-gnu


cd ${WORK_PATH}/deploy/bin/

cp ${WORK_PATH}/target/x86_64-unknown-linux-gnu/release/bridger ${WORK_PATH}/deploy/bin/
chmod +x bridger
tar cjSf bridger-x86_64-linux-gnu.tar.bz2 bridger
mv ${WORK_PATH}/deploy/bin/bridger ${WORK_PATH}/deploy/


echo -e '\e[1;32m🔑 Generating File(s) Hash\e[0m'

md5sum * > ../md5sums.txt
sha256sum * > ../sha256sums.txt

mv ../md5sums.txt .
mv ../sha256sums.txt .

ls
