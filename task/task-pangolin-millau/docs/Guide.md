Bridge of pangolin-millau
===

## Compile

```bash
git clone https://github.com/darwinia-network/bridger
cd bridger
cargo build --release
```

## Start

`pangolin-millau` as one of bridges of bridger, so first needs to start `darwinia-bridger`

```bash
darwinia-bridger server --base-path /path/to/bridge-config
```

Next, ready a config file for `pangolin-millau`, we can use this command to generate a config template

```bash
darwinia-bridger task config-template --name task-pangolin-millau
```

Such as

```toml
[pangolin]
endpoint = "ws://127.0.0.1:23044"
signer = "//Alice"

[millau]
endpoint = "ws://127.0.0.1:13044"
signer = "//Alice"

[relay]
lanes = [ "00000000" ]
auto_start = false
signer_pangolin = "//Alice"
signer_millau = "//Millau"
prometheus_params = { no_prometheus = false, prometheus_host = "127.0.0.1", prometheus_port = 9616 }
```

Then change to right value and save to file, the last we can start `pangolin-millau` bridge

Need to pay attention to these configurations `auto_start`, `signer_pangolin`, `signer_millau`.

- auto_start
  Whether to automatically start pangolin-pangolin when the bridger is started. the s2s bridge needs to be initialized first, so this value should be `false` the first time it is started.
- signer_pangolin/signer_millau
  The signature account used for relay messages between s2s bridges. this is not necessary. if you do not provide these two values, the init will be used by default, which is in the configuration file `[pangolin]` `[millau]` provided account

```bash
darwinia-bridger task start --name task-pangolin-millau --config /path/to/pangolin-millau.toml
```

`darwinia-bridger` will store the config when `pangolin-millau` bridge started, the storage path is the `base-path` you specify. no need to specify config file for future startup.

```bash
darwinia-bridger task start --name task-pangolin-millau
```

Of course, if you want to update your config, you can use `task restart` to update your config.

```bash
darwinia-bridger task restart --name task-pangolin-millau --config /path/to/pangolin-millau.toml
```

And if your want to stop `pangolin-millau`, run `task stop` command

```bash
darwinia-bridger task stop --name task-pangolin-millau
```

After do that, the `pangolin-millau` birdge is stated. finally, you need to pay attention to that,
because the s2s bridge needs to be initialized. the bridge that has not been initialized should not start the relay.
to start pangolin, the following steps should be followed

1. When start for the first time, `auto_start` should be set to false, otherwise, bridger will automatically start the `pangolin-millau` relay service.
2. After the first startup, execute the `task exec` command to initialize the bridge
   ```bash
   darwinia-bridger task exec -n task-pangolin-millau --api init-bridge --param bridge=pangolin-to-millau
   darwinia-bridger task exec -n task-pangolin-millau --api init-bridge --param bridge=millau-to-pangolin
   ```
3. After initialization, execute the `task exec` command to start the `relay` service
   ```bash
   darwinia-bridger task exec -n task-pangolin-millau --api start-relay
   ```
   When executed, it will automatically update the value of `auto_start` to true
4. There is no need to do other configuration in the future, `pangolin-millau` will start with the start of bridger.
