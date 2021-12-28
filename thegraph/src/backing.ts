import { BigInt } from "@graphprotocol/graph-ts"
import {
  backing,
  BackingLock,
  NewTokenRegistered,
  OwnershipTransferred,
  RedeemTokenEvent,
  RegistCompleted,
  VerifyProof
} from "../generated/backing/backing"
import { TransactionEntity} from "../generated/schema"

export function handleBackingLock(event: BackingLock): void {
  let tx = new TransactionEntity(event.transaction.hash.toHex());
  tx.origin = 'Backing';
  tx.blockNumber = event.block.number;
  tx.blockHash = event.block.hash;
  tx.txHash = event.transaction.hash;
  tx.txIndex = event.transaction.index;
  tx.txType = 'RedeemErc20Token';
  tx.save()
}

export function handleNewTokenRegistered(event: NewTokenRegistered): void {
  let tx = new TransactionEntity(event.transaction.hash.toHex());
  tx.origin = 'Backing';
  tx.blockNumber = event.block.number;
  tx.blockHash = event.block.hash;
  tx.txHash = event.transaction.hash;
  tx.txIndex = event.transaction.index;
  tx.txType = 'RegisterErc20Token';
  tx.save()
}

export function handleOwnershipTransferred(event: OwnershipTransferred): void {}

export function handleRedeemTokenEvent(event: RedeemTokenEvent): void {}

export function handleRegistCompleted(event: RegistCompleted): void {}

export function handleVerifyProof(event: VerifyProof): void {}
