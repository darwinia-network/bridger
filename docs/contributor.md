Contributor Guide
===

## How to run bridger

The first, please read [Usage](./usage.md) to known how to compile bridger and
set to local mode.

We bundle some develop tools in bridger
- [pangolin-pangolinparachain](../scripts/develop/pangolin-pangolinparachain)
- [pangolin-pangoro](../scripts/develop/pangolin-pangoro)


## How to run bridge pangolin-pangolinparachain in local

### First

Override develop subql project.yml file, You do not need to make any changes for
the first execution. Let the node start successfully first.

| template                              | path                                      |
|---------------------------------------|-------------------------------------------|
| parachain-rococo.project.yaml         | subql/parachain/rococo/project.yaml       |
| s2s-pangolin.project.yaml             | subql/s2s/pangolin/project.yaml           |
| s2s-pangolin-parachain.project.yaml   | subql/s2s/pangolin-parachain/project.yaml |
| s2s-rococo.project.yaml               | subql/s2s/rococo/project.yaml             |

### Second

Copy [bootstrap.template.sh](../scripts/develop/pangolin-pangolinparachain/bootstrap.template.sh)
to [bootstrap.local.sh](../scripts/develop/pangolin-pangolinparachain/bootstrap.template.sh)
and then change your environments

```bash
export CARGO_HOME=/tmp/cargo
export RUSTUP_HOME=/tmp/rustup
# Data path, include pangolin/pangolin-parachain/rococo/bridger/subql data
export DATA_DIR=/path/to/data-path
# Pangolin parachain source code path
export PANGOLIN_PARACHAIN_SOURCE=/path/to/darwinia-parachain
# Pangolin source code path
export PANGOLIN_SOURCE=/path/to/darwinia-common
```

Then you can run

```bash
./bootstrap.local.sh force
```

to start all nodes/subqls

> About of `force` argument, if you set this argument, will bee clean data of ${DATA_DIR}.
> But, the first time, you must use this to init all subql project.


When all nodes are started successfully. Please modify the subql's `endpoint`
/`genesisHash`/`startBlock`. and run `docker-compose restart`


## How to run bridge pangolin-pangoro in local

The steps is same with pangolin-pangolinparachain
