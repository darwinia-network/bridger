Bridge of darwinia-crab
===

## Prepare

1. `cargo build --release`
2. `cp .maintain/config/task-darwinia-crab.toml ~/.bridger`

   Copy config to default config path `~/.bridger` and fill the content.

> About start `darwinia` and `crab` chain you can read [darwinia](https://github.com/darwinia-network/darwinia#building).
> In addition, we also deployed two public test chains
> - `darwinia`: wss://rpc.darwinia.network
> - `crab`: wss://crab-rpc.darwinia.network

## Run

1. `bridger server`
   Start bridge server

2. Open another shell

3. Init bridge (Only need once )

   ```bash
   bridger task exec -n task-darwinia-crab --api init-bridge --param bridge=darwinia-to-crab
   bridger task exec -n task-darwinia-crab --api init-bridge --param bridge=crab-to-darwinia
   ```

4. Start relay

   ```bash
   bridger task exec -n task-darwinia-crab --api start-relay
   ```

   When executed, it will automatically update the value of `auto_start` to true.

