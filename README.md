# Darwinia Bridger

[![CI](https://github.com/darwinia-network/bridger/workflows/CI/badge.svg)](https://github.com/darwinia-network/bridger/actions)
[![release](https://img.shields.io/github/v/release/darwinia-network/bridger)](https://github.com/darwinia-network/bridger/releases/latest)
[![doc](https://img.shields.io/badge/current-docs-brightgreen.svg)](https://rust-docs.darwinia.network/bridger)
[![downloads](https://img.shields.io/crates/d/darwinia-bridger.svg)](https://github.com/darwinia-network/bridger/releases/latest)
[![license](https://img.shields.io/github/license/darwinia-network/bridger)](https://choosealicense.com/licenses/gpl)

Relayers (aka. Bridgers) in Darwinia Network are offchain worker clients which help relay the headers and messages between source chains and target chains, they work between two chains and require RPC access of two chains.

Darwinia Bridger (this repo) is an implementation of a relayer client written in Rust.

## Installation

### Download from GitHub releases

Download the binary from [latest release](https://github.com/darwinia-network/bridger/releases/latest).

### Pull the Docker image

> The latest version you can find from [latest release](https://github.com/darwinia-network/bridger/releases/latest)

```bash
docker pull quay.io/darwinia-network/bridger:<VERSION>
```

### Build from source

> Please install rust toolchain first

```bash
git clone https://github.com/darwinia-network/bridger.git
cd bridger/
./bridger.sh -h
```

## Configuration

The currently darwinia-bridger supports these bridges, the config you can click docs link

| type                | bridge            | doc                                                |
| ------------------- | ----------------- | -------------------------------------------------- |
| substrate-ethereum  | darwinia-ethereum | [Guide](./bridges/darwinia-ethereum/docs/Guide.md) |
| substrate-ethereum  | pangolin-ropsten  | [Guide](./bridges/pangolin-ropsten/docs/Guide.md)  |
| substrate-substrate | pangolin-pangoro  | [Guide](./bridges/pangolin-pangoro/docs/Guide.md)  |
| substrate-substrate | darwinia-crab     | [Guide](./bridges/darwinia-crab/docs/Guide.md)     |

## Usage

The more usage you can click [Usage](./docs/Usage.md)
