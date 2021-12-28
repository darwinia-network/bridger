import {SubstrateBlock, SubstrateEvent, SubstrateExtrinsic} from '@subql/types';
import {BlockHandler, EventHandler} from '../handler';
import {FastEvent} from '../helpers';


export async function handleBlock(block: SubstrateBlock): Promise<void> {
  const handler = new BlockHandler(block);
  await handler.save();
}

export async function handleEvent(event: SubstrateEvent): Promise<void> {
  const fastEvent = new FastEvent(event);
  const handler = new EventHandler(fastEvent);
  await handler.save();
}

export async function handleCall(extrinsic: SubstrateExtrinsic): Promise<void> {
  // const record = await StarterEntity.get(extrinsic.block.block.header.hash.toString());
  // //Date type timestamp
  // record.field4 = extrinsic.block.timestamp;
  // //Boolean tyep
  // record.field5 = true;
  // await record.save();
}


