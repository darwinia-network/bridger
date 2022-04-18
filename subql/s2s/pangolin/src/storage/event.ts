import {FastEvent} from '../helpers';
import {
  MessageAcceptedEvent
} from '../types';


export async function storeMessageAccepted(event: FastEvent) {
  const data = event.data;
  const [laneId, messageNonce] = data as unknown as [string, number];
  const _event = new MessageAcceptedEvent(event.id);
  _event.atBlock = event.blockNumber;
  _event.laneId = laneId;
  _event.messageNonce = messageNonce;

  _event.timestamp = event.timestamp;
  await _event.save();
}
