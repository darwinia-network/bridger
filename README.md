## Bridger

[![bridger](https://github.com/darwinia-network/bridger/workflows/bridger/badge.svg)](https://github.com/darwinia-network/bridger)
[![crate](https://img.shields.io/crates/v/darwinia-bridger.svg)](https://crates.io/crates/darwinia-bridger)
[![doc](https://img.shields.io/badge/current-docs-brightgreen.svg)](https://docs.rs/darwinia-bridger/)
[![downloads](https://img.shields.io/crates/d/darwinia-bridger.svg)](https://crates.io/crates/darwinia-bridger)
[![LICENSE](https://img.shields.io/crates/l/darwinia-bridger.svg)](https://choosealicense.com/licenses/gpl/)

The Darwinia Bridger

## Installation

### Install from github releases

Download binary from [latest release](https://github.com/darwinia-network/bridger/releases/latest).

### [ Deprecated due to [#76](https://github.com/darwinia-network/bridger/issues/76) ]Install using cargo

```
$ cargo install darwinia-bridger
```

### Build from source

```
git clone https://github.com/darwinia-network/bridger.git
cd bridger
cargo build --release
cd ./target/release/
./bridger run -v
```

> Note: rustc toolchain `nightly-2020-10-06` will be installed along with this package, due to: https://github.com/paritytech/substrate/issues/7282
>
> ```rustup default nightly-2020-10-06```

## Usage

```
$ bridger
darwinia-bridger 0.1.11

USAGE:
    bridger <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    affirm          Affirm one block
    affirmations    List affirmations from chain
    confirm         Set Confirmed block with sudo privilege
    guard           Run `guard` service standalone
    help            Prints this message or the help of the given subcommand(s)
    keys            Show sudo and technical committee members' public key
    run             Run the bridger, this will start `ethereum`, `relay`, `redeem` and `guard` services
```

### Run the bridger

#### Configuration

`Bridger` depends on a configuration file, the default is `~/.bridger/config.toml`. 

You don't need to manually create this file, it will be automatically generated by running any subcommand, such as the `bridger run`, `bridger keys`.

When you first run subcommand like ```bridger run```, it will generate an ```config.toml``` in ```~/.bridger```, you may need to config it to the right settings.

Here is an [sample](./.maintain/ropsten_crab_config.toml.sample) for bridgers which want to work on ropsten-crab bridge.
You can also read the more docs about the settings in the sample.

If you running for mainet, here is the [mainet sample](./.maintain/mainet_config.toml.sample) for reference.

You will need to prepare several services before using bridger:
- Darwinia node websocket json-rpc endpoint
- Ethereum json-rpc endpoint, e.g. [Infura](https://infura.io/)
- Darwinia Shadow http restful endpoint, for more details: https://github.com/darwinia-network/shadow
- Account Seed for signing extrinsics. You can use this account for `affirm` and `redeem`
  ```toml
  ...
  seed = '<account seed>'
  ...
  ```
- you can also using a proxy seed to signing extrinsic for your real account. To use proxy, you need to
    + Using [Extrinsic](https://apps.darwinia.network/#/extrinsics) to setting up the proxy using your real account. Calling ```proxy.addProxy(proxy_account, ProxyType.EthereumBridge, 0)```
    + Changing the seed settings to the proxy account's seed.
    + Make sure the proxy.real key in config.toml is set to the real account's address to enable proxy feature. (Currently must be public key hex format, Here is an [tool](https://crab.subscan.io/tools/ss58_transform) which can help address conversion.)    
  
  ```toml
  ...
  seed = '<proxy account seed>' 
  ...
  [proxy]
  real = '<real account public key>'
  ``` 
      
  If you are a member of the technical committee, This is the recommended. The proxy account can do `affirm`, `guard` and `redeem` for the real account.
      
#### Run

`bridger run` will run the bridger fully. You can also using ```bridger run -v``` to enter verbose model which can help print more trace logs.    

`bridger guard` will run the guard service standalone.

## LICENSE

GPL-3.0
