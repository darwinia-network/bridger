pangolin-millau
===

## init-bridge

```bash
substrate-relay init-bridge PangolinToMillau \
  --source wss://pangolin-rpc.darwinia.network \
  --target ws://104.155.228.145:9145 \
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
