Bridge of darwinia-crab
===

Create the configuration file in your config dir. You can create it by coping from a template file. 

  ```bash
  cp .maintain/config/bridge-darwinia-crab.toml ~/.bridger
  ``` 

  If you use docker, you should copy it to your local mapped config dir.

## Edit

You only need to modify two parameters:

* darwinia.signer
  The private key of darwinia account to sign tx.

* crab.signer
  The private key of crab account to sign tx.

## Bridge start command

```bash
bridger darwinia-crab start
```