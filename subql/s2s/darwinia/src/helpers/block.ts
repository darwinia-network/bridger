import { Block } from '@polkadot/types/interfaces/runtime';
import {SubstrateBlock} from "@subql/types";

export const getBlockTimestamp = (block: Block): Date => {
  const extrinsicForSetTimestamp = block.extrinsics.find((item) => {
    return item.method.method === 'set' && item.method.section === 'timestamp';
  });

  if (extrinsicForSetTimestamp) {
    return new Date(Number(extrinsicForSetTimestamp?.args?.[0].toString()));
  }

  return new Date();
};

export class FastBlock {
  private readonly block: SubstrateBlock;

  constructor(block: SubstrateBlock) {
    this.block = block;
  }

  get raw() {
    return this.block;
  }

  get number() {
    return this.block.block.header.number.toNumber() || Number(0);
  }

  get hash() {
    return this.block.block.hash.toString();
  }

  get specVersion() {
    return this.block.specVersion;
  }

  get parentHash() {
    return this.block.block.header.parentHash.toString();
  }
}
