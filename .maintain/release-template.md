## Upgrade Guide

Just upgrade version, if you run bridger use docker, maybe you should
add `--privileged` https://github.com/dani-garcia/vaultwarden/issues/2497

## Changed

- Support pangoro-goerli (#584)
- Improve latest delivered nonce (#586)

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
