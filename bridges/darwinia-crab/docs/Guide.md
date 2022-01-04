Bridge of darwinia-crab
===

## Prepare

1. Read [Usage](../../../docs/Useage.md)
2. `cp .maintain/config/bridge-darwinia-crab.toml ~/.bridger`

   Copy config to default config path `~/.bridger` and fill the content.

> About start `darwinia` and `crab` chain you can read [darwinia](https://github.com/darwinia-network/darwinia#building).
> In addition, we also deployed two public test chains
> - `darwinia`: wss://rpc.darwinia.network
> - `crab`: wss://crab-rpc.darwinia.network

## Init

```bash
bridger darwinia-crab init darwinia-to-crab
bridger darwinia-crab init crab-to-darwinia
```

## Start


```bash
bridger darwinia-crab start
```
