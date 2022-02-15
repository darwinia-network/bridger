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
| Pangolin | EthereumRelayAuthorities | Instance1DarwiniaRelayAuthorities |
| Darwinia | EthereumRelay            | DarwiniaEthereumRelay             |
| Darwinia | EthereumRelayerGame      | Instance1DarwiniaRelayerGame      |
| Darwinia | EthereumRelayAuthorities | Instance1DarwiniaRelayAuthorities |



