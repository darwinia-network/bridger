pangolin-millau
===

## init-bridge

pangolin -> millau

```bash
substrate-relay init-bridge PangolinToMillau \
  --source wss://pangolin-rpc.darwinia.network \
  --target ws://104.155.228.145:9145 \
  --target-signer //Alice
```

millau -> pangolin

```bash
substrate-relay init-bridge MillauToPangolin \
  --source ws://104.155.228.145:9145 \
  --target wss://pangolin-rpc.darwinia.network \
  --target-signer //Alice
```

## relay

```bash
substrate-relay relay PangolinToMillau \
  --source wss://pangolin-rpc.darwinia.network \
  --target ws://104.155.228.145:9145 \
  --lanes 00000000 \
  --source-signer //Alice \
  --target-signer //Alice
```
