pangolin-ropsten
===

## Prepare

1. `cargo build --release`
2. `cp .maintain/config/task-pangolin-ropsten.toml ~/.bridger`
   The dir can be any path you like, and fill the content.

## Run

1. `bridger server`
   Start bridge server

2. Open another shell

3. Set pangolin and ropsten scan block

   ```bash
   # set pangolin
   bridger kv -n task-pangolin-ropsten put scan.pangolin.planned 123456
   # set ropsten
   bridger kv -n task-pangolin-ropsten put scan.ropsten.check.planned 123456 \
     scan.ropsten.redeem.planned 123456 \
     scan.ropsten.affirm.planned 123456
   ```

4. Set pangolin and ropsten scan to running

   ```bash
   # set pangolin
   bridger kv -n task-pangolin-ropsten put scan.pangolin.running true
   # set ropsten check/redeem/affirm scan
   bridger kv -n task-pangolin-ropsten put scan.ropsten.check.running true \
     scan.ropsten.redeem.running true \
     scan.ropsten.check.running true
   ```

## More

### Query state

Deprecated

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

**ropsten affirm scan state**

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
  bridger kv -n task-pangolin-ropsten \
    put \
    scan.ropsten.check.planned 36987
  ```
  WARNING: if you set planned, the next times start block is `planned + RANGE_LIMIT + 1`
- `running`
  The scan is running
