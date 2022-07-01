import {SubstrateBlock} from '@subql/types';
import * as storage from '../storage';
import {Block} from '../types';
import {FastBlock} from "../helpers";

export class BlockHandler {
  private readonly block: SubstrateBlock;

  static async ensureBlock(id: string): Promise<void> {
    const block = await Block.get(id);

    if (!block) {
      await new Block(id).save();
    }
  }

  constructor(block: SubstrateBlock) {
    this.block = block;
  }

  public async save() {
    const fastBlock = new FastBlock(this.block);
    await storage.storeJustification(fastBlock);
  }
}
