Bridge of pangolin-pangolinparachain
===

Create the configuration file in your config dir. You can create it by coping from a template file.

  ```bash
  cp .maintain/config/bridge-pangolin-pangolinparachainalpha.toml ~/.bridger
  ```

  If you use docker, you should copy it to your local mapped config dir.

## Edit

You need to modify two parameters:

* pangolin.signer
  The private key of pangolin account to sign tx.

* pangolin_parachainalpha.signer
  The private key of pangolin_parachainalpha account to sign tx.

## Bridge start command

```bash
bridger pangolin-pangolinparachainalpha start
```
