## Bridger

[![bridger](https://github.com/darwinia-network/bridger/workflows/bridger/badge.svg)](https://github.com/darwinia-network/bridger)
[![crate](https://img.shields.io/crates/v/darwinia-bridger.svg)](https://crates.io/crates/darwinia-bridger)
[![doc](https://img.shields.io/badge/current-docs-brightgreen.svg)](https://docs.rs/darwinia-bridger/)
[![downloads](https://img.shields.io/crates/d/darwinia-bridger.svg)](https://crates.io/crates/darwinia-bridger)
[![LICENSE](https://img.shields.io/crates/l/darwinia-bridger.svg)](https://choosealicense.com/licenses/gpl/)

The Darwinia Bridger

## Installation

### Build from source

```
git clone https://github.com/darwinia-network/bridger.git
cd bridger
cargo build --release
cd ./target/release/
./bridger run -v
```

### [Outdated]Install from https://crates.io/

```
$ cargo install darwinia-bridger
```

## Usage

```
$ bridger
darwinia-bridger 0.0.4

USAGE:
    bridger <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    affirm     Affirm target block
    confirm    Set Confirmed block with sudo privilege
    help       Prints this message or the help of the given subcommand(s)
    keys       Show technical committee members
    run        Run the bridger
```

### Run the bridger

#### Configuration

When you first run ```bridger run```, it will auto generate an ```config.toml``` in ```~/.bridger```, you may need to config it to the right settings.

Here is an [sample](./.maintain/ropsten_crab_config.toml.sample) for bridgers which want to work on ropsten-crab bridge.
You can also read the more docs about the settings in the sample.

You will need to prepare several services before using bridger:
- Darwinia node websocket json-rpc endpoint
- Ethereum json-rpc endpoint, e.g. [Infura](https://infura.io/)
- Darwinia Shadow http restful endpoint, for more details: https://github.com/darwinia-network/shadow
- Account Seed for signing extrinsics, you can also using a proxy seed to signing extrinsic for your real account. To use proxy, you need to
    + Using [Extrinsic](https://apps.darwinia.network/#/extrinsics) to setting up the proxy using your real account. Calling ```proxy.addProxy(proxy_account, ProxyType.EthereumBridge, 0)```
    + Changing the seed settings to the proxy account's seed.
    + Make sure the proxy.real key in config.toml is set to the real account's address to enable proxy feature. (Currently must be hex format without 0x prefix, Here is an [address conversion tool](https://crab.subscan.io/tools/ss58_transform))

#### Run

You can also using ```briger run -v``` to enter verbose model which can help print more trace logs.    

## LICENSE

GPL-3.0
