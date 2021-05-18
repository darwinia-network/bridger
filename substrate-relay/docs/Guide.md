substrate-relay 使用指南
===

> substrate-relay 尚未进入稳定状态, 此文档所描述的操作方式可能会发生变动

## 编译

目前 substrate-relay 是独立于 bridger 的一个项目, 需要进入到 substrate-relay 目录下进行编译.

```bash
git clone https://github.com/darwinia-network/bridger
cd bridger
git checkout -b feature/substrate-relay origin/feature/substrate-relay
cd substrate-relay
cargo build --release
```

编译完成后会在 `bridger/substrate-relay/target/release` 目录下有一个 `substrate-relay` 的可执行程序

## 使用

```text
substrate-relay 0.3.3
Substrate relay

USAGE:
    substrate-relay.exe <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    config         Substrate relay config
    help           Prints this message or the help of the given subcommand(s)
    init-bridge    Init substrate to substrate bridge
    relay          Relay headers and messages
    start          Start substrate relay
```

### start

首先通过 `start` 指令来启动一个 `substrate-relay` 服务

```text
substrate-relay.exe-start 0.3.3
Start substrate relay

USAGE:
    substrate-relay.exe start [FLAGS] [OPTIONS]

FLAGS:
        --enable-auth    Is enable authorization for request this server
        --help           Prints help information
    -V, --version        Prints version information

OPTIONS:
    -c, --config <config>    The config file path
    -h, --host <host>        Listen host, Default:  127.0.0.1
    -p, --port <port>        Listen port, Default: 7890
```

`substrate-relay` 服务会监听一个 http 端口, 默认为 7890, 所有有关 `substrate-relay` 的交互均会从这个服务中开始, 因此需要首先启动服务

```bash
substrate-relay start --host 0.0.0.0 --port 7890
```

参数中 `--enable-auth` 表明是否要开启 `substrate-relay` 的访问授权, 如果开启授权, 所有访问 `substrate-relay` 的接口都需要携带 [token](#token).

### config

```text
substrate-relay.exe-config 0.3.3
Substrate relay config

USAGE:
    substrate-relay.exe config [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
        --debug      Enable debug model, show more message
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --server <server>    The server host by substrate-relay service [default: http://127.0.0.1:7890]
    -k, --token <token>      The token of substrate-relay service

SUBCOMMANDS:
    chain    Config chain information
    help     Prints this message or the help of the given subcommand(s)
    token    Config service token
```

`config` 指令用于设定 `substrate-relay`, 包括 `chain` 以及 `token` 两个部分

#### token

`token` 指令是用来设定 `substrate-relay` 服务的授权值, 可以管理 token, 生成或者删除.

```bash
$ substrate-relay config token list
token           remark
b0dc8e810b4469daf073fe84868b6864fcf657a7                ab
```

```bash
$ substrate-relay.exe config token generate --remark "test token"
token           remark
493901a9a949c94a6e49136b7924428aa8077b1f  test token
```

```bash
$ substrate-relay.exe config token remove 91eb548c7e57f156613f4c71ac3d58858699c914
Success
```

#### chain

`chain` 用于在 `substrate-relay` 中添加链, 为了后续搭建链之间的桥做准备

> 因为目前尚未实现链的动态加载, 实际上通过这里配置的链只能是预先设定好的, 自定义链的添加也需要代码的支持.



添加链

```bash
$ substrate-relay.exe config config chain add pangolin --host 127.0.0.1 --port 1234 --signer //Alice
name            host            port            signer
pangolin        127.0.0.1               1234           //Alice
```

```bash
$ substrate-relay.exe config config chain add millau --host 127.0.0.1 --port 5678 --signer //Alice
name            host            port            signer
pangolin        127.0.0.1               1234           //Alice
millau          127.0.0.1               5678           //Alice
```

```bash
$ substrate-relay.exe config config chain add test --host 127.0.0.1 --port 7890 --signer //Alice
name            host            port            signer
pangolin        127.0.0.1               1234           //Alice
millau          127.0.0.1               5678           //Alice
test            127.0.0.1               7890           //Alice
```

删除链

```chain
$ substrate-relay config chain remove test
name            host            port            signer
pangolin        127.0.0.1               1234           //Alice
millau          127.0.0.1               5678           //Alice
```

列表

```bash
$ substrate-relay.exe config chain list
name            host            port            signer
pangolin        127.0.0.1               23044           //Alice
millau          127.0.0.1               13044           //Alice
```

### init-bridge

当链信息准备完毕后, 可以通过 `init-bridge` 指令初始化链之间的桥

```bash
$ substrate-relay init-bridge --source pangolin --target millau
Success
```

### relay

当链之间的桥初始化完毕, 可以开始进行链的消息 relay

```text
substrate-relay.exe-relay 0.3.3
Relay headers and messages

USAGE:
    substrate-relay.exe relay <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    start     Start relay
    status    Relay status
```

### 启动 relay

```bash
$ substrate-relay relay start --source pangolin --target millau --lane 00000000
```

> 注意: relay 功能已可用, 但是还有细节需要优化, 目前会提示超时, 但是不影响 relay 服务

## 配置文件

默认情况下 `substrate-relay` 会将配置以文件的形式存储在本地, 默认路径为 `substrate-relay` 可执行文件同级目录, 可以通过 `substrate-relay start -c` 指定路径


```toml
[generic]
config_file = 'D:/dev/darwinia-network/_data/bridger/config.toml'
host = '0.0.0.0'
port = 7890
enable_auth = false

[[chains]]
name = 'pangolin'
host = '127.0.0.1'
port = 23044
signer = '//Alice'
secure = false

[[chains]]
name = 'millau'
host = '127.0.0.1'
port = 13044
signer = '//Alice'
secure = false

[[tokens]]
value = 'b0dc8e810b4469daf073fe84868b6864fcf657a7'
remark = 'ab'

[[tokens]]
value = '493901a9a949c94a6e49136b7924428aa8077b1f'
remark = ''
```

