#!/bin/bash
#
#

set -xe

BIN_PATH=$(dirname $(readlink -f $0))
WORK_PATH=${BIN_PATH}/../

cd ${WORK_PATH}

yum -y update && yum -y upgrade && yum -y install \
  git make \
  clang gcc gcc-c++ llvm

RUST_TOOLCHAIN=nightly-2021-04-15

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
  sh -s -- -y --profile minimal --default-toolchain ${RUST_TOOLCHAIN}

source ~/.cargo/env

rustup target add wasm32-unknown-unknown

cargo doc --all --no-deps --release
