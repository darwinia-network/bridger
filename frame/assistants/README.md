Assistants
===

## Subxt client

Since substrate frame v2, the pallet name and storage prefix are same. so the
subxt generated code is also follow this. but currently Darwinia/Pangolin chains
also have some pallet use v1 style. It happens that pallet and prefix are not
the same. There are mapping table to describe relationship.

| Chain    | Pallet Name              | Storage Prefix                    |
| -------- | ------------------------ | --------------------------------- |
| Pangolin | EthereumRelay            | DarwiniaEthereumRelay             |
| Pangolin | EthereumRelayerGame      | Instance1DarwiniaRelayerGame      |
| Darwinia | EthereumRelay            | DarwiniaEthereumRelay             |
| Darwinia | EthereumRelayerGame      | Instance1DarwiniaRelayerGame      |
| Darwinia | EthereumRelayAuthorities | Instance1DarwiniaRelayAuthorities |

WARNING:
For [subxt@darwinia-v0.12.2](https://github.com/darwinia-network/subxt/tree/darwinia-v0.12.2)
have a bug, whe the argument is a tuple, only first parameter is used.

Please note the following storage keys

- EthereumBacking::VerifiedProof

effect chains:
- pangolin
- darwinia


- *Feemarket::Orders

effect chains:
- darwinia
- crab
- pangolin
- pangoro
- pangolin-parachain
- crab-parachain
