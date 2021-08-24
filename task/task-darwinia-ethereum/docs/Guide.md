darwinia-ethereum
===

## Prepare

1. `cargo build --release`
2. `cp .maintain/config/task-darwinia-ethereum.toml ~/.bridger`
   The dir can be any path you like, and fill the content.
3. `cp .maintain/config/linked-darwinia.toml ~/.bridger`

## Run

1. `./target/release/bridger server`
   Start bridge server

2. Open another shell

3. Set darwinia and ethereum scan block

   ```bash
   # set darwinia
   ./target/release/bridger kv -n task-darwinia-ethereum put scan.darwinia.next 123456
   # set ethereum
   ./target/release/bridger kv -n task-darwinia-ethereum put scan.ethereum.next 123456
   ```

4. Set darwinia and ethereum scan to running

   ```bash
   # set darwinia
   ./target/release/bridger kv -n task-darwinia-ethereum put scan.darwinia.running true
   # set ethereum
   ./target/release/bridger kv -n task-darwinia-ethereum put scan.ethereum.running true
   ```


### Query state


**darwinia scan state**

```bash
./target/release/bridger kv \
  -n task-darwinia-ethereum \
  get \
  scan.darwinia.current \
  scan.darwinia.next \
  scan.darwinia.finish \
  scan.darwinia.running \
  scan.darwinia.skipped \
  -o table --include-key

+-----------------------+----------------------------------------+
| scan.darwinia.current | 150                                    |
+-----------------------+----------------------------------------+
| scan.darwinia.next    | null                                   |
+-----------------------+----------------------------------------+
| scan.darwinia.finish  | 149                                    |
+-----------------------+----------------------------------------+
| scan.darwinia.running | false                                  |
+-----------------------+----------------------------------------+
| scan.darwinia.skipped | 1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16 |
|                       | ,17,18,19,20,21,22,23,24,25,26,27,28,2 |
|                       | 9,30,31,32,33,34,35,36,37,38,39,40,41  |
+-----------------------+----------------------------------------+
```

**ethereum scan state**

```bash
./target/release/bridger kv \
  -n task-darwinia-ethereum \
  get \
  scan.ethereum.current \
  scan.ethereum.next \
  scan.ethereum.finish \
  scan.ethereum.running \
  scan.ethereum.skipped \
  -o table --include-key

+----------------------+-------+
| scan.ethereum.current | 3     |
+----------------------+-------+
| scan.ethereum.next    | null  |
+----------------------+-------+
| scan.ethereum.finish  | 2     |
+----------------------+-------+
| scan.ethereum.running | false |
+----------------------+-------+
| scan.ethereum.skipped | null  |
+----------------------+-------+
```

explain:

- `current`
  Currently scan block number
- `next`
  Planned block number, if there is a value, the next time will start from this block. (support array, use `,` to split)
  ```bash
  ./target/release/bridger kv -n task-darwinia-ethereum \
    put \
    scan.ethereum.next 1,425,36987
  ```
  The order of scanning will be: `1` `425` `36987` `36988` `36989` ...
- `finish`
  Currently finished block number
- `running`
  The scan is running
- `skipped`
  Skipped block, maybe happened some error, these blocks need to be taken seriously



## Migrate

### Migrate 0.3.x to <=0.4.5

To keep the progress, bridger(0.3.x) will save two files.

- ~/.bridger/last-redeemed
- ~/.bridger/last-tracked-darwinia-block

Bridger(0.4.x) does not need these two files, so if you want to continue from the previous progress after the upgrade, you need to migrate these two values.

1. [Prepare](#Prepare)
2. Run bridger server (0.4.x)
   ```bash
   ./target/release/bridger server
   ```
4. Start the darwinia subscribe service
   ```bash
    ./target/release/bridger task exec \
      --name task-darwinia-ethereum \
      --api start-darwinia \
      --param block_number=$(cat ~/.bridger/last-tracked-darwinia-block)
   ```
4. Start the ethereum subscribe service
   ```bash
   ./target/release/bridger task exec \
     --name task-darwinia-ethereum \
     --api start-ethereum \
     --param block_number=$(cat ~/.bridger/last-redeemed)
   ```

> From 0.4.6 the bridger removed `start-darwinia` and `start-ethereum` command, so there only support migrate to <0.4.6


## Security config

if you want to encrypt your private key in a config file. first, you can use `crypto encrypt` command to get your encrypted value.

```bash
./target/release/bridger crypto encrypt --value abcdefg
```

when you got it. then update your config. change follow this.

```toml
[darwinia]
enable_crypto = true
# private key of relayer, or, private key of proxy
relayer_private_key = "<YOUR CRYPTED DATA>"
# ...
[task]
enable_crypto = true
interval_ethereum = 120
interval_relay = 60
# ...
```

Now we support these filed to encrypt and decrypt

- darwinia.relayer_private_key

> NOTE: please use the same password to encrypt your keys.

when the configuration is encrypted, the task will not start until you provide the password.

your can start with `--password`

```bash
./target/release/bridger start -n task-darwinia-ethereum --password
```

or without `--password` to start. then run the `set-password` command.

```bash
./target/release/bridger task start -n task-darwinia-ethereum
./target/release/bridger task set-password -n task-darwinia-ethereum
```

## Custom command

### affirm


**params**

| name  | type |
| ----- | ---- |
| block | u64  |

**examples**

```bash
./target/release/bridger task exec --name task-darwinia-ethereum --api affirm --param block=125468
```


### affirm-force

**params**

| name  | type |
| ----- | ---- |
| block | u64  |

**examples**

```bash
./target/release/bridger task exec --name task-darwinia-ethereum --api affirm-force --param block=125468
```


### affirm-raw

**params**

| name | type   |
| ---- | ------ |
| json | string |

**examples**

```bash
./target/release/bridger task exec --name task-darwinia-ethereum --api affirm-raw --param 'json={"block": 234557}'
```

### confirm

**params**

| name  | type |
| ----- | ---- |
| block | u64  |

**examples**

```bash
./target/release/bridger task exec --name task-darwinia-ethereum --api confirm --param block=125468
```

### ecdsa

**params**

| name    | type   |
| ------- | ------ |
| message | string |

**examples**

```bash
./target/release/bridger task exec --name task-darwinia-ethereum --api ecdsa --param "message=Hello"
```

### info-d2e

**params**

| name      | type   |
| --------- | ------ |
| network   | string |
| txblock   | u64    |
| mmrblock  | u64    |
| signblock | u64    |

**examples**

```bash
./target/release/bridger task exec \
  --name task-darwinia-ethereum \
  --api info-d2e \
  --param network=<NETWORK> \
  --param txblock=12345 \
  --param mmrblock=2345346 \
  --param signblock=824864
```

### keys


**examples**

```bash
./target/release/bridger task exec --name task-darwinia-ethereum --api keys
```

### sign-mmr-root


**params**

| name      | type   |
| --------- | ------ |
| network   | string |
| mmrblock  | u64    |


**examples**

```bash
./target/release/bridger task exec \
  --name task-darwinia-ethereum \
  --api sign-mmr-root \
  --param network=<NETWORK> \
  --param mmrblock=2345346
```

### show-parcel


**params**

| name   | type   | allow      | default |
| ------ | ------ | ---------- | ------- |
| output | string | raw / json | raw     |
| block  | u64    |            |         |


**examples**

```bash
./target/release/bridger task exec \
  --name task-darwinia-ethereum \
  --api show-parcel \
  --param output=json \
  --param block=2345346
```

### relay


**params**

| name         | type |
| ------------ | ---- |
| block_number | u64  |

**examples**

```bash
./target/release/bridger task exec \
  --name task-darwinia-ethereum \
  --api relay \
  --param block_number=2354684
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
> Updating the authority set involves 2 times of cross-chain: 1) relay the new authority set from Darwinia to Ethereum; 2) relay from Ethereum to Darwinia to send rewards to `ethereum.relayer_beneficiary_darwinia_account` (see below).

Comment out if you don't want to relay authorities change messages to Ethereum (which sends transactions on Ethereum and consumes gas fee), and of course, you will not get rewards.


### guard

**examples**

```bash
./target/release/bridger task exec --name task-darwinia-ethereum --api guard
```
