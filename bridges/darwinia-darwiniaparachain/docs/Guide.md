ridge of darwinia-darwiniaparachain
==

reate the configuration file in your config dir. You can create it by coping from a template file.

 ```bash
 cp .maintain/config/bridge-darwinia-darwiniaparachain.toml ~/.bridger
 ``` 

 If you use docker, you should copy it to your local mapped config dir.

## Edit

ou need to modify two parameters:

 darwinia.signer
 The private key of darwinia account to sign tx.

 darwinia_parachain.signer
 The private key of darwinia_parachain account to sign tx.

# Bridge start command

``bash
ridger darwinia-darwiniaparachain start
``
