import {SubstrateExtrinsic, SubstrateEvent, SubstrateBlock} from '@subql/types';
import {FastEvent} from '../helpers';
import {EventHandler} from "../handler/event";
import {BlockHandler} from "../handler/block";


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
}


