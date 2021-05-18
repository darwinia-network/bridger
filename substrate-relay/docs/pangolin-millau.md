pangolin-millau
===

## 启动服务

```bash
substrate-relay start --host 0.0.0.0 --port 7890
```

## 注册链

```bash
substrate-relay config chain add pangolin --host 127.0.0.1 --port 1234 --signer //Alice
substrate-relay config chain add millau --host 127.0.0.1 --port 2345 --signer //Alice
```

## init-bridge

```bash
substrate-relay init-bridge --source pangolin --target millau
```

## relay

```bash
substrate-relay relay start --source pangolin --target millau --lane 00000000
```





