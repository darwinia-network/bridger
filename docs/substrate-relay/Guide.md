substrate-relay guide
===

## Compile

Currently `substrate-relay` is independent of `bridge`, need compile in [`substrate-relay`](../) folder.

```bash
git clone https://github.com/darwinia-network/bridger
cd bridger
cargo build --release
```

When compiled success, will have `substrate-relay` executable file in `bridger/target/release` folder.

## Steps

Start a Substrate To Substrate bridge, have two steps, the first, init this bridge, this step will record Source Chain best finalized block to Target Chain, the relay step read this value (in target chain), then read source chain from this position and write to Target Chain; The second step is relay headers and message.

## Bridge

In `substrate-relay`, only pre-defined bridges can be used, currently the `substrate-relay` have these bridges:

- PangolinToMillau

### init-bridge

```bash
$ substrate-relay init-bridge --help
substrate-relay-init-bridge 0.3.3
Init bridge

USAGE:
    substrate-relay init-bridge [OPTIONS] <bridge> --source <source> --target <target>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --source <source>
        --source-signer <source-signer>
        --source-signer-password <source-signer-password>
    -t, --target <target>
        --target-signer <target-signer>
        --target-signer-password <target-signer-password>

ARGS:
    <bridge>    The bridge name
```


```bash
substrate-relay init-bridge PangolinToMillau \
  --source wss://pangolin-rpc.darwinia.network \
  --target ws://104.155.228.145:9145 \
  --target-signer //Alice
```

> Attention: the `target-signer` parameters is required for `init-bridge`, the value is account of Target Chain, the purpose is to authorize when writing data, if this account have password use `target-signer-password` to set it.

### relay

```bash
$ substrate-relay relay --help
substrate-relay-relay 0.3.3
Relay headers and messages

USAGE:
    substrate-relay relay [FLAGS] [OPTIONS] <bridge> --source <source> --target <target>

FLAGS:
    -h, --help             Prints help information
        --no-prometheus    Do not expose a Prometheus metric endpoint
    -V, --version          Prints version information

OPTIONS:
        --lanes <lanes>...                                    [default: 00000000]
        --prometheus-host <prometheus-host>
            Expose Prometheus endpoint at given interface [default: 127.0.0.1]

        --prometheus-port <prometheus-port>                  Expose Prometheus endpoint at given port [default: 9616]
    -s, --source <source>
        --source-signer <source-signer>
        --source-signer-password <source-signer-password>
    -t, --target <target>
        --target-signer <target-signer>
        --target-signer-password <target-signer-password>

ARGS:
    <bridge>    The bridge name
```

```bash
substrate-relay relay PangolinToMillau \
  --source wss://pangolin-rpc.darwinia.network \
  --target ws://104.155.228.145:9145 \
  --lanes 00000000 \
  --source-signer //Alice \
  --target-signer //Alice
```

> Attention: the `source-signer` and `target-signer` are required for `relay`, the purpose is to authorize when writing data, if this account have password use `target-signer-password` to set it. if the account have password, use `source-signer-password` and `target-signer-password` to set it.

