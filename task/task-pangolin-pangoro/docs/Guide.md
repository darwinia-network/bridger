Bridge of pangolin-pangoro
===

## Prepare

1. `cargo build --release`
2. `cp .maintain/config/task-pangolin-pangoro.toml ~/.bridger`

   Copy config to default config path `~/.bridger` and fill the content.


## Run

1. `bridger server`
   Start bridge server

2. Open another shell

3. Init bridge (Only need once )

   ```bash
   bridger task exec -n task-pangolin-pangoro --api init-bridge --param bridge=pangolin-to-pangoro
   bridger task exec -n task-pangolin-pangoro --api init-bridge --param bridge=pangoro-to-pangolin
   ```

4. Start relay

   ```bash
   bridger task exec -n task-pangolin-pangoro --api start-relay
   ```

   When executed, it will automatically update the value of `auto_start` to true.

