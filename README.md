# Darwinia Bridger

[![CI](https://github.com/darwinia-network/bridger/workflows/CI/badge.svg)](https://github.com/darwinia-network/bridger/actions)
[![release](https://img.shields.io/github/v/release/darwinia-network/bridger)](https://github.com/darwinia-network/bridger/releases/latest)
[![doc](https://img.shields.io/badge/current-docs-brightgreen.svg)](https://docs.rs/darwinia-bridger/)
[![downloads](https://img.shields.io/crates/d/darwinia-bridger.svg)](https://crates.io/crates/darwinia-bridger)
[![license](https://img.shields.io/github/license/darwinia-network/bridger)](https://choosealicense.com/licenses/gpl/)

Relayers (aka. Bridgers) in Darwinia Network are offchain worker clients which help relay the headers and messages between source chains and target chains, they works between two chains and requires RPC access of two chains.

Darwinia Bridger (this repo) is an implementation of relayer client written in Rust.

## Installation

### Download from GitHub releases

Download the binary from [latest release](https://github.com/darwinia-network/bridger/releases/latest).

### Pull the Docker image

```bash
docker pull quay.io/darwinia-network/bridger:<VERSION>
```

### ~Install using cargo~

> ⚠️ This method is temporarily unavailable due to #76.

```bash
cargo install darwinia-bridger
```

### Build from source

> Note: rustc toolchain `nightly-2020-10-06` will be installed along with this package, see <https://github.com/paritytech/substrate/issues/7282>. You might want to change the default toolchain using `rustup default nightly-2020-10-06`.

```bash
git clone https://github.com/darwinia-network/bridger.git
cd bridger/
cargo build --release
```

## Configuration

Darwinia Bridger depends on a TOML config file, it is located in `~/.bridger/config.toml` by default.

Sample configs:

- For mainnet: [.maintain/config/mainnet.toml.sample](.maintain/config/mainnet.toml.sample)
- For testnet (Ropsten and Crab): [.maintain/config/ropsten_crab.toml.sample](.maintain/config/ropsten_crab.toml.sample)

The config file can be generated by executing any subcommand, such as `bridger run` and `bridger keys`. Thus, you don't need to manually create the file, but it is required to update a few fields:

#### `node`

The WebSocket endpoint of Darwinia official node or any archive node (`darwinia --pruning=archive`), supports `ws://` and `wss://`. For example:

```toml
node = "wss://cc1.darwinia.network"
```

#### `shadow`

The endpoint of [Darwinia Shadow service](https://github.com/darwinia-network/shadow), supports HTTP and HTTPS. For example using the official service:

```toml
shadow = "https://shadow.darwinia.network"
```

#### `eth.rpc`

The endpoint of Ethereum RPC. You can use the public geth hosted by Darwinia, your self-hosted node, or any third-party Ethereum RPC service, such as Infura and Alchemy. For example:

```toml
[eth]
rpc = "https://mainnet.geth.darwinia.network"
```

#### `seed`

Private key in hex of your account or proxy account to sign relay extrinsics. It is highly recommended to use a proxy account to prevent exposing your main account's private key. Proxy account is an account that can "perform" on behalf of your "real" account. **To allow an account proxying extrinsics, you need to open [Extrinsics in apps.darwinia.network](https://apps.darwinia.network/#/extrinsics), switch to your "real" account, and submit the `proxy.addProxy(proxy_account, ProxyType.EthereumBridge, 0)` extrinsic to delegate it.** For example:

```toml
seed = "0x0000000000000000000000000000000000000000000000000000000000000000"
```

#### `proxy.real`

**If you set `seed` to the private key of your proxy account, you have to set this to the public key of your "real" account as well.** For example:

```toml
[proxy]
real = "0x0000000000000000000000000000000000000000000000000000000000000000"
```

Here is a [tool](https://polkadot.subscan.io/tools/ss58_transform) provided by Subscan that helps convert SS58 addresses to public keys. Comment out this field if you don't want to use proxy account.

#### `darwinia_to_ethereum.seed`

Private key in hex of your **Ethereum** account. It's similar to `seed`, but it's for signing on Ethereum network. For example:

```toml
[darwinia_to_ethereum]
seed = "0x0000000000000000000000000000000000000000000000000000000000000000"
```

**For users who want to relay messages from Darwinia to Ethereum, you must request to become a member of the authority set first**:

1. Open [Extrinsics in apps.darwinia.network](https://apps.darwinia.network/#/extrinsics)
2. If you're not using a proxy account or you have the permission to sign extrinsics using the "real" account, switch to the "real" account and submit `ethereumRelayAuthorities.requestAuthority(stake_amount, signer)`:
   - `stake_amount` is the amount of RINGs to stake.
   - `signer` is the public key of your **Ethereum** account.
    Otherwise, you must switch to your proxy account and submit `proxy.proxy(real_account, ethereumRelayAuthorities.requestAuthority(stake_amount, signer))`. The `stake_amount` will be deducted from your "real" account.
3. Notify council members to submit `ethereumRelayAuthorities.addAuthority(your_account)`.

> How it works: authorities are the validators/nodes in the source chain consensus system to resolve Byzantine Generals' Problem and finalize the blocks. Grandpa authorities are BFT alike authorities, our authority concept comes from the similar meaning, is to be used as a replacement for grandpa authorites.
>
> Updating the authority set involves 2 times of cross-chain: 1) relay the new authority set from Darwinia to Ethereum; 2) relay from Ethereum to Darwinia to send rewards to `darwinia_to_ethereum.beneficiary` (see below).

#### `darwinia_to_ethereum.beneficiary`

Public key in hex of your **Darwinia** account which receives the rewards of relaying new authorities. For example:

```toml
[darwinia_to_ethereum]
beneficiary = "0x0000000000000000000000000000000000000000000000000000000000000000"
```

Comment out if you don't want to relay authorities change messages to Ethereum (which sends transactions on Ethereum and consumes gas fee), and of course, you will not get rewards.

## Usage

The latest help texts are also available in `bridger --help`.

```
$ bridger
darwinia-bridger 0.3.3

USAGE:
    bridger <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    affirm          Affirm one target block
    affirm-raw      Affirm a raw parcel from json str
    affirmations    List affirmations from chain
    confirm         Set Confirmed block with sudo privilege
    guard           Run `guard` service standalone
    help            Prints this message or the help of the given subcommand(s)
    keys            Show sudo and technical committee members' public key
    run             Run the bridger, this will start `ethereum`, `relay`, `redeem` and `guard` services
    set-start       Set where to start the ethereum scan
    show-parcel     Show a parcel from ethereum
```

Typically, `bridger run` is the only command that you need to know to launch bridger and start all internal services. You can also use `bridger run -v` to enter the verbose mode which prints more trace logs.

If you hit the error `No darwinia scan start, run 'bridger set-darwinia-start --block <scan_start> [--data-dir <data_dir>]' to set one`, please run the following command, `1671723` is the block number that the Darwinia to Ethereum bridge launched at:

```bash
bridger set-darwinia-start --block 1671723
```
