

## Upgrade Guide

Fixed several bugs for substrate to substrate bridges.
There is a [known bug](https://github.com/darwinia-network/bridger/issues/343) in this version for ethereum bridge, please use v0.4.5 instead.

## Added

None

## Changed

- Bug Fixes

## Resources

### Docker

#### Pull with the Git Tag

```docker
docker pull quay.io/darwinia-network/bridger:{{ tag }}
```

#### Pull with the Git Commit SHA

```docker
docker pull quay.io/darwinia-network/bridger:sha-{{ sha }}
```

## Guide

| bridge            | doc                                                   |
| ----------------- | ----------------------------------------------------- |
| darwinia-ethereum | [Guide](../task/task-darwinia-ethereum/docs/Guide.md) |
| pangolin-ropsten  | [Guide](../task/task-pangolin-ropsten/docs/Guide.md)  |
| pangolin-pangoro  | [Guide](../task/task-pangolin-pangoro/docs/Guide.md)  |
| darwinia-crab     | [Guide](../task/task-darwinia-crab/docs/Guide.md)     |

***Known issue***
https://github.com/darwinia-network/bridger/issues/343
https://github.com/darwinia-network/bridger/issues/347

