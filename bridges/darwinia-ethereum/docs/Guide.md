darwinia-ethereum
===

## Prepare

1. Read [Usage](../../../docs/Useage.md)
2. `cp .maintain/config/bridge-darwinia-ethereum.toml ~/.bridger`
   The dir can be any path you like, and fill the content.

## Run

1. Set darwinia and ethereum scan block

   ```bash
   # set darwinia
   bridger kv -n task-darwinia-ethereum put scan.darwinia.planned 123456
   # set ethereum
   bridger kv -n task-darwinia-ethereum put scan.ethereum.check.planned 123456 \
     scan.ethereum.redeem.planned 123456 \
     scan.ethereum.affirm.planned 123456
   ```

2. Set darwinia and ethereum scan to running

   ```bash
   # set darwinia
   bridger kv -n task-darwinia-ethereum put scan.darwinia.running true
   # set ethereum check/redeem/affirm scan
   bridger kv -n task-darwinia-ethereum put scan.ethereum.check.running true \
     scan.ethereum.redeem.running true \
     scan.ethereum.affirm.running true
   ```


### Query state


**darwinia scan state**

```bash
bridger kv \
  -n task-darwinia-ethereum \
  get \
  scan.darwinia.current \
  scan.darwinia.planned \
  scan.darwinia.running \
  -o table --include-key

+-----------------------+--------+
| scan.darwinia.current | 502434 |
+-----------------------+--------+
| scan.darwinia.planned | null   |
+-----------------------+--------+
| scan.darwinia.running | true   |
+-----------------------+--------+
```

**ethereum affirm scan state**

```bash
bridger kv \
  -n task-darwinia-ethereum \
  get \
  scan.ethereum.affirm.current \
  scan.ethereum.affirm.planned \
  scan.ethereum.affirm.running \
  -o table --include-key

+------------------------------+----------+
| scan.ethereum.affirm.current | 10730129 |
+------------------------------+----------+
| scan.ethereum.affirm.planned | null     |
+------------------------------+----------+
| scan.ethereum.affirm.running | true     |
+------------------------------+----------+
```

**ethereum redeem scan state**

```bash
bridger kv \
  -n task-darwinia-ethereum \
  get \
  scan.ethereum.redeem.current \
  scan.ethereum.redeem.planned \
  scan.ethereum.redeem.running \
  -o table --include-key

+------------------------------+----------+
| scan.ethereum.redeem.current | 10495250 |
+------------------------------+----------+
| scan.ethereum.redeem.planned | null     |
+------------------------------+----------+
| scan.ethereum.redeem.running | true     |
+------------------------------+----------+
```

**ethereum check scan state**

```bash
bridger kv \
  -n task-darwinia-ethereum \
  get \
  scan.ethereum.check.current \
  scan.ethereum.check.planned \
  scan.ethereum.check.running \
  -o table --include-key

+-----------------------------+----------+
| scan.ethereum.check.current | 10861015 |
+-----------------------------+----------+
| scan.ethereum.check.planned | null     |
+-----------------------------+----------+
| scan.ethereum.check.running | true     |
+-----------------------------+----------+
```

explain:

- `current`
  Currently scan block number
- `planned`
  Planned block number, If the value is set, the next times will start from this block.
  ```bash
  bridger kv -n task-darwinia-ethereum \
    put \
    scan.ethereum.check.planned 36987
  ```
  WARNING: if you set planned, the next times start block is `planned + RANGE_LIMIT + 1`
- `running`
  The scan is running



## Migrate

### Migrate

- [Upgrade guide for version v0.4.8 or later](https://github.com/darwinia-network/bridger/issues/315)
- [Upgrade guide for version v0.5.x](https://github.com/darwinia-network/bridger/issues/390)


## Custom command

### do affirm

**do affirm with block**

```bash
bridger darwinia-ethereum affirm do --mode block --block 125468
```

**do affirm with raw**

```bash
bridger darwinia-ethereum affirm do --mode raw --raw '{"mmr_root": "", "header": {...}}'
```

### affirm state

```bash
bridger darwinia-ethereum state
```

### confirm

```bash
bridger darwinia-ethereum confirm --block 125468
```

### ecdsa

```bash
bridger darwinia-ethereum ecdsa --message Hello
```

### info

**params**

| name      | type   |
| --------- | ------ |
| network   | string |
| txblock   | u64    |
| mmrblock  | u64    |
| signblock | u64    |

**examples**

```bash
bridger darwinia-ethereum d2e \
  --network=<NETWORK> \
  --txblock=12345 \
  --mmrblock=2345346 \
  --signblock=824864
```

### keys


**examples**

```bash
bridger darwinia-ethereum keys
```

### sign-mmr-root


**params**

| name      | type   |
| --------- | ------ |
| network   | string |
| mmrblock  | u64    |


**examples**

```bash
bridger darwinia-ethereum mmr \
  --network=<NETWORK> \
  --mmrblock=2345346
```

### show-parcel


**params**

| name   | type   | allow      | default |
| ------ | ------ | ---------- | ------- |
| output | string | raw / json | raw     |
| block  | u64    |            |         |


**examples**

```bash
bridger darwinia-ethereum parcel \
  --output=json \
  --block=2345346
```

### relay


**params**

| name         | type |
| ------------ | ---- |
| block_number | u64  |

**examples**

```bash
bridger darwinia-ethereum relay \
  --block=2354684
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
bridger darwinia-ethereum guard
```
