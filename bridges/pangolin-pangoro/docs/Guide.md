Bridge of pangolin-pangoro
===

Create the configuration file in your config dir. You can create it by coping from a template file.

  ```bash
  cp .maintain/config/bridge-pangolin-pangoro.toml ~/.bridger
  ``` 

  If you use docker, you should copy it to your local mapped config dir.

## Edit

You only need to modify two parameters:

* pangolin.signer
  The private key of pangolin account to sign tx.

* pangoro.signer
  The private key of pangoro account to sign tx.

## Bridge start command

```bash
bridger pangolin-pangoro start
```