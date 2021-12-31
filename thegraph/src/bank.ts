import { BigInt } from "@graphprotocol/graph-ts"
import {
  bank,
  ClaimedTokens,
  NewDeposit,
  ClaimedDeposit,
  TransferDeposit,
  BurnAndRedeem,
  LogSetAuthority,
  LogSetOwner
} from "../generated/bank/bank"
import {BankEntity, TransactionEntity} from "../generated/schema"

export function handleClaimedTokens(event: ClaimedTokens): void {
  // Entities can be loaded from the store using a string ID; this ID
  // needs to be unique across all entities of the same type
  let entity = BankEntity.load(event.transaction.from.toHex())

  // Entities only exist after they have been saved to the store;
  // `null` checks allow to create entities on demand
  if (!entity) {
    entity = new BankEntity(event.transaction.from.toHex())

    // Entity fields can be set using simple assignments
    entity.count = BigInt.fromI32(0)
  }

  // BigInt and BigDecimal math are supported
  entity.count = entity.count + BigInt.fromI32(1)

  // Entity fields can be set based on event parameters
  entity._token = event.params._token
  entity._owner = event.params._owner

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
  // - contract.userTotalDeposit(...)
  // - contract.CONTRACT_USER_POINTS(...)
  // - contract.userDeposits(...)
  // - contract.CONTRACT_WATER_ERC20_TOKEN(...)
  // - contract.CONTRACT_GOLD_ERC20_TOKEN(...)
  // - contract.depositCount(...)
  // - contract.CONTRACT_RING_ERC20_TOKEN(...)
  // - contract.UINT_AUCTION_CUT(...)
  // - contract.UINT_BANK_UNIT_INTEREST(...)
  // - contract.CONTRACT_TOKEN_LOCATION(...)
  // - contract.computePenalty(...)
  // - contract.computeInterest(...)
  // - contract.CONTRACT_KTON_ERC20_TOKEN(...)
  // - contract.CONTRACT_WOOD_ERC20_TOKEN(...)
  // - contract.CONTRACT_FIRE_ERC20_TOKEN(...)
  // - contract.UINT_BANK_PENALTY_MULTIPLIER(...)
  // - contract.isClaimRequirePenalty(...)
  // - contract.CONTRACT_LAND_BASE(...)
  // - contract.registry(...)
  // - contract.CONTRACT_INTERSTELLAR_ENCODER(...)
  // - contract.CONTRACT_PET_BASE(...)
  // - contract.CONTRACT_SOIL_ERC20_TOKEN(...)
  // - contract.owner(...)
  // - contract.getDeposit(...)
  // - contract.CONTRACT_OBJECT_OWNERSHIP(...)
  // - contract.CONTRACT_TOKEN_USE(...)
  // - contract.deposits(...)
  // - contract.CONTRACT_ERC721_BRIDGE(...)
  // - contract.CONTRACT_REVENUE_POOL(...)
  // - contract.authority(...)
  // - contract.getDepositIds(...)
  // - contract.bytesToUint256(...)
  // - contract.CONTRACT_LAND_RESOURCE(...)
  // - contract.MONTH(...)
  // - contract.UINT_REFERER_CUT(...)
  // - contract.UINT_TOKEN_OFFER_CUT(...)
  // - contract.CONTRACT_DIVIDENDS_POOL(...)
}

export function handleNewDeposit(event: NewDeposit): void {}

export function handleClaimedDeposit(event: ClaimedDeposit): void {}

export function handleTransferDeposit(event: TransferDeposit): void {}

export function handleBurnAndRedeem(event: BurnAndRedeem): void {
  let tx = new TransactionEntity(event.transaction.hash.toHex());
  tx.origin = 'Bank';
  tx.blockNumber = event.block.number;
  tx.blockHash = event.block.hash;
  tx.txHash = event.transaction.hash;
  tx.txIndex = event.transaction.index;
  tx.txType = 'Deposit';
  tx.save()
}

export function handleLogSetAuthority(event: LogSetAuthority): void {}

export function handleLogSetOwner(event: LogSetOwner): void {}
