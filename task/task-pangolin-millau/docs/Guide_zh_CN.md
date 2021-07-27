pangolin-millau 桥
===

## 编译

```bash
git clone https://github.com/darwinia-network/bridger
cd bridger
cargo build --release
```


## 启动

`pangolin-millau` 作为 bridger 其中的一个桥, 首先需要启动 `bridger`.

```bash
bridger server --base-path /path/to/bridge-config
```

接下来准备一个 `pangoli-millau` 桥的配置文件, 可以透过

```bash
bridger task config-template --name task-pangolin-millau
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
auto_start = false
signer_pangolin = "//Alice"
signer_millau = "//Millau"
prometheus_params = { no_prometheus = false, prometheus_host = "127.0.0.1", prometheus_port = 9616 }
```

将其配置修改为正确的值, 并存储为一个文件, 接下来便可以启动 `pangolin-millau` 桥

需要注意的是 `relay` 里面的 `auto_start` `signer_pangolin` `signer_millau` 这三个配置.

- auto_start
  桥启动时是否自动启动 relay, 这是由 s2s 桥的特性所增添的配置, 因为 s2s 之间 relay 消息之前需要先进行链的初始化, 未初始化的两个链不应该进行 relay, 因此, 第一次启动的时候此值应该一直是 false, 不开启 relay. 当初始化后通过 `task exec` 指令开启 relay, 此值将会自动更新.
- signer_pangolin/signer_millau
  s2s 桥之间 relay 消息所使用的签名账户, 这并不是必须的, 如果不提供这两个值, 将会默认使用 init 时也就是配置文件中的 `[pangolin]` `[millau]` 中所提供的账户

```bash
bridger task start --name task-pangolin-millau --config /path/to/pangolin-millau.toml
```

bridger 会记住第一次所给定的配置并保存在 bridge 启动时指定的 `base-path` 目录中, 以后如果需要启动则无需再指定配置文件

```bash
bridger task start --name task-pangolin-millau
```

当然, 如果希望更新配置文件, 可以通过 `task restart` 指令重新给予新的配置文件

```bash
bridger task restart --name task-pangolin-millau --config /path/to/pangolin-millau.toml
```

如果希望停止 `pangolin-millau` 桥, 执行 `task stop` 指令

```bash
bridger task stop --name task-pangolin-millau
```

当执行完上述指令后, pangolin-millau 桥就启动成功了, 最后还需要注意的是, 因为 s2s 桥是需要在对手链进行初始化的.
尚未被初始化的桥不应该启动 relay. 也就是上面配置文件中所设定的 `auto_start` 配置. 正常启动 `pangolin-millau` 应该遵循一下步骤

1. 当第一次启动时, `auto_start` 应该设置为 false, 否则, bridger 将会自动启动 `pangolin-millau` relay 服务.
2. 当第一次启动后, 执行 `task exec` 指令进行桥的初始化工作
   ```bash
   bridger task exec -n task-pangolin-millau --api init-bridge --param bridge=pangolin-to-millau
   bridger task exec -n task-pangolin-millau --api init-bridge --param bridge=millau-to-pangolin
   ```
3. 初始化后, 执行 `task exec` 指令开始 `relay` 服务
   ```bash
   bridger task exec -n task-pangolin-millau --api start-relay
   ```
   当执行后, 将会自动更新 `auto_start` 的值为 true
4. 以后就不需要做其他配置, `pangolin-millau` 将会随着 bridger 的启动而一起启动.
