substrate-relay 使用指南
===

## 编译

```bash
git clone https://github.com/darwinia-network/bridger
cd bridger
cargo build --release
```

编译完成后会在 `bridger/target/release` 目录下有一个 `substrate-relay` 的可执行程序

## 步骤

启动一个 Substrate To Substrate 桥, 有两个步骤, 首先初始化桥, 用于在 Target Chain 中记录 Source Chain 当前同步的 finalized block 记录下来, 给 relay 做准备, 从什么位置开始同步. `init-bridge` 工作是一次性的, 后续 relay 的时候会根据同步的状态来更新这个值. 接下来就是第二布开始进行 relay 进行数据同步.

## 桥

在 `substrate-relay` 中, 可以使用的桥是预先定义好的, 因此我们只能使用已经添加的桥. 目前 `substrate-relay` 支持的桥如下:

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

> 注意: `init-bridge` 需要提供 `target-signer` 用于在目标链授权写入数据. 如果填写的目标链用户有密码请添加 `target-signer-password` 指定

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

> 注意: `relay` 需要提供 `source-signer` 以及 `target-signer` 用于在双方链同步数据的认证账户 (relay 是双向的, 前提是两边都有先初始化), 如果有密码分别添加 `source-signer-password` 以及 `target-signer-password`
 