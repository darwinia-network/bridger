Bridge of darwinia-crab
===

## Prepare

1. `cargo build --release`
2. `cp .maintain/config/task-darwinia-crab.toml ~/.bridger`

   复制配置文件到默认目录 `~/.bridger`, 并且修改内容.

> 关于启动 `darwinia` 和 `crab` 参见 [darwinia](https://github.com/darwinia-network/darwinia#building).
> 此外我们也部署了两个公共的测试链
> - `darwinia`: wss://rpc.darwinia.network
> - `crab`: wss://crab-rpc.darwinia.network

## Run

1. `bridger server`
   启动 bridger server

2. 开启另外一个 Shell 窗口

3. 初始化桥 (仅需要在双方链之间由一个人执行一次, 桥矿工无需执行)

   ```bash
   bridger task exec -n task-darwinia-crab --api init-bridge --param bridge=darwinia-to-crab
   bridger task exec -n task-darwinia-crab --api init-bridge --param bridge=crab-to-darwinia
   ```

4. 开始消息 relay

   ```bash
   bridger task exec -n task-darwinia-crab --api start-relay
   ```

   当执行此指令后, 将会自动将配置文件中的 `auto_start` 设定为 true, 以后将会自动启动.

