pangolin-ropsten
===

## Prepare

1. `cargo build --release`
2. `cp .maintain/config/task-pangolin-ropsten.toml ~/.bridger`
   The dir can be any path you like, and fill the content.

## Run

1. `./target/release/darwinia server`
   Start bridge server

2. Open another shell

3. Set pangolin and ropsten scan block

   ```bash
   # set pangolin
   ./target/release/bridger kv -n task-pangolin-ropsten put scan.pangolin.next 123456
   # set ropsten
   ./target/release/bridger kv -n task-pangolin-ropsten put scan.ropsten.next 123456
   ```

4. Set pangolin and ropsten scan to running

   ```bash
   # set pangolin
   ./target/release/bridger kv -n task-pangolin-ropsten put scan.pangolin.running true
   # set ropsten
   ./target/release/bridger kv -n task-pangolin-ropsten put scan.ropsten.running true
   ```

## More

### About start

The pangolin and ropsten scan service you can control use `bridger kv` command.

**Set is running**

```bash
# set pangolin
./target/release/bridger kv -n task-pangolin-ropsten put scan.pangolin.running true
# set ropsten
./target/release/bridger kv -n task-pangolin-ropsten put scan.ropsten.running true
```

**Set is stop**

```bash
# set pangolin
./target/release/bridger kv -n task-pangolin-ropsten put scan.pangolin.running false
# set ropsten
./target/release/bridger kv -n task-pangolin-ropsten put scan.ropsten.running false
```

The bridger will record the set value.

### Query state


**pangolin scan state**

```bash
./target/release/bridger kv \
  -n task-pangolin-ropsten \
  get \
  scan.pangolin.current \
  scan.pangolin.next \
  scan.pangolin.finish \
  scan.pangolin.running \
  scan.pangolin.skipped \
  -o table --include-key

+-----------------------+----------------------------------------+
| scan.pangolin.current | 150                                    |
+-----------------------+----------------------------------------+
| scan.pangolin.next    | null                                   |
+-----------------------+----------------------------------------+
| scan.pangolin.finish  | 149                                    |
+-----------------------+----------------------------------------+
| scan.pangolin.running | false                                  |
+-----------------------+----------------------------------------+
| scan.pangolin.skipped | 1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16 |
|                       | ,17,18,19,20,21,22,23,24,25,26,27,28,2 |
|                       | 9,30,31,32,33,34,35,36,37,38,39,40,41  |
+-----------------------+----------------------------------------+
```

**ropsten scan state**

```bash
./target/release/bridger kv \
  -n task-pangolin-ropsten \
  get \
  scan.ropsten.current \
  scan.ropsten.next \
  scan.ropsten.finish \
  scan.ropsten.running \
  scan.ropsten.skipped \
  -o table --include-key

+----------------------+-------+
| scan.ropsten.current | 3     |
+----------------------+-------+
| scan.ropsten.next    | null  |
+----------------------+-------+
| scan.ropsten.finish  | 2     |
+----------------------+-------+
| scan.ropsten.running | false |
+----------------------+-------+
| scan.ropsten.skipped | null  |
+----------------------+-------+
```

explain:

- `current`
  Currently scan block number
- `next`
  Planned block number, if there is a value, the next time will start from this block. (support array, use `,` to split)
  ```bash
  ./target/release/bridger kv -n task-pangolin-ropsten \
    put \
    scan.ropsten.next 1,425,36987
  ```
  The order of scanning will be: `1` `425` `36987` `36988` `36989` ...
- `finish`
  Currently finished block number
- `running`
  The scan is running
- `skipped`
  Skipped block, maybe happened some error, these blocks need to be taken seriously
