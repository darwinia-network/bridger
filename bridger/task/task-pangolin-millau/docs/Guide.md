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
darwinia-bridge serve --base-path /path/to/bridge-config
```

Next, ready a config file for `pangolin-millau`, we can use this command to generate a config template

```bash
darwinia-bridge task config-template --name task-pangolin-millau
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
prometheus_params = { no_prometheus = false, prometheus_host = "127.0.0.1", prometheus_port = 9616 }
```

Then change to right value and save to file, the last we can start `pangolin-millau` bridge

```bash
darwinia-bridge task start --name task-pangolin-millau --config /path/to/pangolin-millau.toml
```

`darwinia-bridger` will store the config when `pangolin-millau` bridge started, the storage path is the `base-path` you specify. no need to specify config file for future startup.

```bash
darwinia-bridge task start --name task-pangolin-millau
```

Of course, if you want to update your config, you can use `task restart` to update your config.

```bash
darwinia-bridge task restart --name task-pangolin-millau --config /path/to/pangolin-millau.toml
```

And if your want to stop `pangolin-millau`, run `task stop` command

```bash
darwinia-bridge task stop --name task-pangolin-millau
```
