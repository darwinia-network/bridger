import { BigInt } from "@graphprotocol/graph-ts"
import {
  relay,
  OwnershipTransferred,
  Paused,
  ResetRootEvent,
  SetAuthritiesEvent,
  SetRootEvent,
  Unpaused
} from "../generated/relay/relay"
import {RelayEntity, TransactionEntity} from "../generated/schema"

export function handleOwnershipTransferred(event: OwnershipTransferred): void {
  // Entities can be loaded from the store using a string ID; this ID
  // needs to be unique across all entities of the same type
  let entity = RelayEntity.load(event.transaction.from.toHex())

  // Entities only exist after they have been saved to the store;
  // `null` checks allow to create entities on demand
  if (!entity) {
    entity = new RelayEntity(event.transaction.from.toHex())

    // Entity fields can be set using simple assignments
    entity.count = BigInt.fromI32(0)
  }

  // BigInt and BigDecimal math are supported
  entity.count = entity.count + BigInt.fromI32(1)

  // Entity fields can be set based on event parameters
  entity.previousOwner = event.params.previousOwner
  entity.newOwner = event.params.newOwner

  // Entities can be written to the store with `.save()`
  entity.save()

  // Note: If a handler doesn't require existing field values, it is faster
  // _not_ to load the entity from the store. Instead, create it fresh with
  // `new Entity(...)`, set the fields that should be updated and save the
  // entity back to the store. Fields that were not set or unset remain
  // unchanged, allowing for partial updates to be applied.

  // It is also possible to access smart contracts from mappings. For
  // example, the contract that has emitted the event can be connected to
  // with:
  //
  // let contract = Contract.bind(event.address)
  //
  // The following functions can then be called on this contract to access
  // state variables and other data:
  //
  // - contract.checkNetworkPrefix(...)
  // - contract.checkRelayerNonce(...)
  // - contract.getLockTokenReceipt(...)
  // - contract.getMMRRoot(...)
  // - contract.getNetworkPrefix(...)
  // - contract.getRelayer(...)
  // - contract.getRelayerCount(...)
  // - contract.getRelayerNonce(...)
  // - contract.getRelayerThreshold(...)
  // - contract.isRelayer(...)
  // - contract.mmrRootPool(...)
  // - contract.owner(...)
  // - contract.paused(...)
  // - contract.verifyBlockProof(...)
  // - contract.verifyRootAndDecodeReceipt(...)
}

export function handlePaused(event: Paused): void {}

export function handleResetRootEvent(event: ResetRootEvent): void {}

export function handleSetAuthritiesEvent(event: SetAuthritiesEvent): void {
  let tx = new TransactionEntity(event.transaction.hash.toHex());
  tx.origin = 'Relay';
  tx.blockNumber = event.block.number;
  tx.blockHash = event.block.hash;
  tx.txHash = event.transaction.hash;
  tx.txIndex = event.transaction.index;
  tx.txType = 'SetAuthorities';
  tx.save()
}

export function handleSetRootEvent(event: SetRootEvent): void {}

export function handleUnpaused(event: Unpaused): void {}
