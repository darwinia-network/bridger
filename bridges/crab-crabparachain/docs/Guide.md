Bridge of crab-crabparachain
===

Create the configuration file in your config dir. You can create it by coping from a template file.

  ```bash
  cp .maintain/config/bridge-crab-crabparachain.toml ~/.bridger
  ``` 

  If you use docker, you should copy it to your local mapped config dir.

## Edit

You need to modify two parameters:

* crab.signer
  The private key of crab account to sign tx.

* crab_parachain.signer
  The private key of crab_parachain account to sign tx.

## Bridge start command

```bash
bridger crab-crabparachain start
```