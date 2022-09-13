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

* The asset name for macOs is `bridger-darwin-x86_64.zip`.  
* The asset name for Linux is `bridger-linux-x86_64.zip`.  
* The asset name for Windows is `bridger-windows-x86_64.zip`.  

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

##### Set registry

You need set registry when you want to compile from source, because default
registry from download precompile binary from Github release page. So set
registry to local

```text
./bridger.sh registry set --type local
```

## Environment Vars

The bridger program will read some environments.

| Name             | Description                                                                                                                       |
|------------------|-----------------------------------------------------------------------------------------------------------------------------------|
| `BRIDGER_HOME`   | All data of bridger and bridges will be store in this path. If not set this value, the default will be store in `$HOME/.bridger`. |
| `LOG_ADAPTER`    | Custom log adapter, you can use `json` value. after do this, all logs will output use json format.                                |
| `LOG_MAX_LEVEL`  | Max log level, default is `trace`                                                                                                 |
| `RUST_BACKTRACE` | You can set  `1` or `0` to enable or disable error full error trace                                                               |
| `RUST_LOG`       | Custom log level for target or crate                                                                                              |

## Configuration

The currently darwinia-bridger supports muliti bridges. Each bridge has its own configuration file. The configuration filename pattern is `bridge-<BRIDGE_NAME>.toml`

The default configuration dir is your `BRIDGER_HOME` env var.  

If you use docker to run the bridger. You should put your configuration file to your dir that will be mapped to the container's `/root/.bridger`. For example: `/home/ubuntu/bridger_config_dir/pangolin-pangoro`.


### Mainnet
| TYPE                | BRIDGE NAME                 |                                                              |
| ------------------- | --------------------------- | ------------------------------------------------------------ |
| substrate-ethereum  | darwinia-ethereum           | [Guide](./bridges/darwinia-ethereum/docs/Guide.md)           |
| substrate-substrate | darwinia-crab               | [Guide](./bridges/darwinia-crab/docs/Guide.md)               |
| substrate-substrate | crab-crabparachain          | [Guide](./bridges/crab-crabparachain/docs/Guide.md)          |

### Testnet
| TYPE                | BRIDGE NAME                 |                                                               |
| ------------------- | --------------------------- | ------------------------------------------------------------- |
| substrate-ethereum  | pangoro-goerli              | [Guide](./bridges/pangoro-goerli/docs/Guide.md)               |
| substrate-substrate | pangolin-pangoro            | [Guide](./bridges/pangolin-pangoro/docs/Guide.md)             |
| substrate-substrate | pangolin-pangolinparachain  | [Guide](./bridges/pangolin-pangolinparachain/docs/Guide.md)   |

> Goerli is the testnet of Ethereum.  
> Pangolin is the testnet of Crab.
> Pangoro is the testnet of Darwinia.

## Usage

### Binary

```bash
bridger pangolin-pangoro start
```
The `pangolin-pangoro` here is a bridge name listed in the previous `Configuration` section.


`-h` will list all commands it supports.
```bash
bridger pangolin-pangoro -h
```

##### Update

Open your `~/.bridger/bridger.toml` and update the version to the new one.

```toml
[registry]
path = "https://github.com/darwinia-network/bridger"
type = "Github"
version = "0.6.4"
```

Run `bridger pangolin-pangoro start`, It will automatically update the new version bridge.


### Docker

```bash
docker run -d \
  --name pangolin-pangoro \
  --restart always \
  -v /home/ubuntu/bridger_config_dir/pangolin-pangoro:/root/.bridger \
  quay.io/darwinia-network/bridger:v0.6.3 \
  pangolin-pangoro start
```

Note: It is recommended that you create a directory specific to a bridge if you use docker to run bridger. Here we have a `pangolin-pangoro` dir in your local config dir.

##### Update

First delete your container, then rerun the docker run command with the new version.
```bash
docker stop pangolin-pangoro
docker rm pangolin-pangoro
docker run -d \
  --name pangolin-pangoro \
  --restart always \
  -v /home/ubuntu/bridger_config_dir/pangolin-pangoro:/root/.bridger \
  quay.io/darwinia-network/bridger:v0.6.4 \
  pangolin-pangoro start
```

### docker-compose

If you run multiple bridges, you can use docker-compose to simplify the process. What you need to do is create a `docker-compose.yml` file. Here is an example:

```yml
version: '3'
services:
  pangolin-pangoro:
    container_name: pangolin-pangoro
    image: quay.io/darwinia-network/bridger:v0.6.4
    restart: always
    volumes:
      - /etc/localtime:/etc/localtime
      - /home/ubuntu/bridger_config_dir/pangolin-pangoro:/root/.bridger
    command:
      - pangolin-pangoro
      - start

pangolin-pangolinparachain:
    container_name: pangolin-pangolinparachain
    image: quay.io/darwinia-network/bridger:v0.6.4
    restart: always
    volumes:
      - /etc/localtime:/etc/localtime
      - /home/ubuntu/bridger_config_dir/pangolin-pangolinparachain:/root/.bridger
    command:
      - pangolin-pangolinparachain
      - start

  pangoro-goerli:
    container_name: pangoro-goerli
    image: quay.io/darwinia-network/bridger:v0.6.4
    restart: always
    volumes:
      - /etc/localtime:/etc/localtime
      - /home/ubuntu/bridger_config_dir/pangoro-goerli:/root/.bridger
    command:
      - pangoro-goerli
      - start
```

Run in the directory where the `docker-compose.yml` file is located:

```bash
docker-compose up -d
```

##### Update

Update the bridger version in your `docker-compose.yml` file, then,

```bash
docker-compose down
docker-compose up -d
```

## Advanced

### Init bridge

After the blockchain is reset, the bridge needs to be initialized. The user does not need to do this, it is usually done by the developer.

```bash
bridger pangolin-pangoro init pangolin-to-pangoro
bridger pangolin-pangoro init pangoro-to-pangolin
```