import { Block } from '../generated/schema'
import { ethereum } from '@graphprotocol/graph-ts'

export function handleBlock(block: ethereum.Block): void {
  let id = block.hash.toHex()
  let entity = new Block(id)
  entity.author = block.author
  entity.parentHash = block.parentHash
  entity.stateRoot = block.stateRoot
  entity.transactionsRoot = block.transactionsRoot
  entity.receiptsRoot = block.receiptsRoot
  entity.number = block.number
  entity.gasUsed = block.gasUsed
  entity.gasLimit = block.gasLimit
  entity.timestamp = block.timestamp
  entity.save()
}
