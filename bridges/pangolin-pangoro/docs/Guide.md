Bridge of pangolin-pangoro
===

## Prepare

1. Read [Usage](../../../docs/Usage.md)
2. `cp .maintain/config/bridge-pangolin-pangoro.toml ~/.bridger`

   Copy config to default config path `~/.bridger` and fill the content.

> About start `pangolin` and `pangoro` chain you can read [darwinia-common](https://github.com/darwinia-network/darwinia-common/#development).
> In addition, we also deployed two public test chains
> - `pangolin`: wss://pangolin-rpc.darwinia.network
> - `pangoro`: wss://pangoro-rpc.darwinia.network

## Init

```bash
bridger pangolin-pangoro init pangolin-to-pangoro
bridger pangolin-pangoro init pangoro-to-pangolin
```

## Start


```bash
bridger pangolin-pangoro start
```
