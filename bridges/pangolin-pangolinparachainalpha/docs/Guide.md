Bridge of pangolin-pangolinparachain
===

Create the configuration file in your config dir. You can create it by coping from a template file.

  ```bash
  cp .maintain/config/bridge-pangolin-pangolinparachain.toml ~/.bridger
  ``` 

  If you use docker, you should copy it to your local mapped config dir.

## Edit

You need to modify two parameters:

* pangolin.signer
  The private key of pangolin account to sign tx.

* pangolin_parachain.signer
  The private key of pangolin_parachain account to sign tx.

## Bridge start command

```bash
bridger pangolin-pangolinparachain start
```