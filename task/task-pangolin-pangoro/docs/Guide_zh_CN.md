Bridge of pangolin-pangoro
===

## Prepare

1. `cargo build --release`
2. `cp .maintain/config/task-pangolin-pangoro.toml ~/.bridger`

   复制配置文件到默认目录 `~/.bridger`, 并且修改内容.


## Run

1. `bridger server`
   启动 bridger server

2. 开启另外一个 Shell 窗口

3. 初始化桥 (仅需要在双方链之间由一个人执行一次, 桥矿工无需执行)

   ```bash
   bridger task exec -n task-pangolin-pangoro --api init-bridge --param bridge=pangolin-to-pangoro
   bridger task exec -n task-pangolin-pangoro --api init-bridge --param bridge=pangoro-to-pangolin
   ```

4. 开始消息 relay

   ```bash
   bridger task exec -n task-pangolin-pangoro --api start-relay
   ```

   当执行此指令后, 将会自动将配置文件中的 `auto_start` 设定为 true, 以后将会自动启动.

