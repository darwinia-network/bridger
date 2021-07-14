pangolin-millau 桥
===

## 编译

```bash
git clone https://github.com/darwinia-network/bridger
cd bridger
cargo build --release
```


## 启动

`pangolin-millau` 作为 bridger 其中的一个桥, 首先需要启动 `darwinia-bridger`.

```bash
darwinia-bridge serve --base-path /path/to/bridge-config
```

接下来准备一个 `pangoli-millau` 桥的配置文件, 可以透过

```bash
darwinia-bridge task config-template --name task-pangolin-millau
```

来获取一个配置模板

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

将其配置修改为正确的值, 并存储为一个文件, 接下来便可以启动 `pangolin-millau` 桥

```bash
darwinia-bridge task start --name task-pangolin-millau --config /path/to/pangolin-millau.toml
```

bridger 会记住第一次所给定的配置并保存在 bridge 启动时指定的 `base-path` 目录中, 以后如果需要启动则无需再指定配置文件

```bash
darwinia-bridge task start --name task-pangolin-millau
```

当然, 如果希望更新配置文件, 可以通过 `task restart` 指令重新给予新的配置文件

```bash
darwinia-bridge task restart --name task-pangolin-millau --config /path/to/pangolin-millau.toml
```

如果希望停止 `pangolin-millau` 桥, 执行 `task stop` 指令

```bash
darwinia-bridge task stop --name task-pangolin-millau
```
