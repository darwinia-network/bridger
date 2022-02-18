pangolin-ropsten
===

## Prepare

1. Read [Usage](../../../docs/Usage.md)
2. `cp .maintain/config/bridge-pangolin-ropsten.toml ~/.bridger`
   The dir can be any path you like, and fill the content.

## Run

1. Set pangolin and ropsten scan block

   ```bash
   # set pangolin
   bridger pangolin-ropsten kv put scan.pangolin.planned 123456
   # set ropsten
   bridger pangolin-ropsten kv put scan.ropsten.check.planned 123456 \
     scan.ropsten.redeem.planned 123456 \
     scan.ropsten.affirm.planned 123456
   ```

2. Set pangolin and ropsten scan to running

   ```bash
   # set pangolin
   bridger pangolin-ropsten kv put scan.pangolin.running true
   # set ropsten check/redeem/affirm scan
   bridger pangolin-ropsten kv put scan.ropsten.check.running true \
     scan.ropsten.redeem.running true \
     scan.ropsten.affirm.running true
   ```

## More

### Query state

**pangolin scan state**

```bash
bridger kv \
  -n task-pangolin-ropsten \
  get \
  scan.pangolin.current \
  scan.pangolin.planned \
  scan.pangolin.running \
  -o table --include-key

+-----------------------+--------+
| scan.pangolin.current | 502434 |
+-----------------------+--------+
| scan.pangolin.planned | null   |
+-----------------------+--------+
| scan.pangolin.running | true   |
+-----------------------+--------+
```

**ropsten affirm scan state**

```bash
bridger kv \
  -n task-pangolin-ropsten \
  get \
  scan.ropsten.affirm.current \
  scan.ropsten.affirm.planned \
  scan.ropsten.affirm.running \
  -o table --include-key

+-----------------------------+----------+
| scan.ropsten.affirm.current | 10730129 |
+-----------------------------+----------+
| scan.ropsten.affirm.planned | null     |
+-----------------------------+----------+
| scan.ropsten.affirm.running | true     |
+-----------------------------+----------+
```

**ropsten redeem scan state**

```bash
bridger kv \
  -n task-pangolin-ropsten \
  get \
  scan.ropsten.redeem.current \
  scan.ropsten.redeem.planned \
  scan.ropsten.redeem.running \
  -o table --include-key

+-----------------------------+----------+
| scan.ropsten.redeem.current | 10495250 |
+-----------------------------+----------+
| scan.ropsten.redeem.planned | null     |
+-----------------------------+----------+
| scan.ropsten.redeem.running | true     |
+-----------------------------+----------+
```

**ropsten check scan state**

```bash
bridger kv \
  -n task-pangolin-ropsten \
  get \
  scan.ropsten.check.current \
  scan.ropsten.check.planned \
  scan.ropsten.check.running \
  -o table --include-key

+----------------------------+----------+
| scan.ropsten.check.current | 10861015 |
+----------------------------+----------+
| scan.ropsten.check.planned | null     |
+----------------------------+----------+
| scan.ropsten.check.running | true     |
+----------------------------+----------+
```

explain:

- `current`
  Currently scan block number
- `planned`
  Planned block number, If the value is set, the next times will start from this block.
  ```bash
  bridger pangolin-ropsten kv \
    put \
    scan.ropsten.check.planned 36987
  ```
  WARNING: if you set planned, the next times start block is `planned + RANGE_LIMIT + 1`
- `running`
  The scan is running


## Migrate

### Migrate

- [Upgrade guide for version v0.4.8 or later](https://github.com/darwinia-network/bridger/issues/315)
- [Upgrade guide for version v0.5.x](https://github.com/darwinia-network/bridger/issues/390)


## Custom command

Please read [darwinia-ethereum](../../darwinia-ethereum/docs/Guide.md#custom-command)
