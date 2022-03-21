import {FastEvent, ALLOW_PARA_IDS} from '../helpers';
import {
  CandidateIncludedEvent
} from '../types';


export async function storeCandidateIncluded(event: FastEvent) {
  const data = event.data;
  const [candidateReceipt, headData, coreIndex, groupIndex] = data;
  const a = JSON.parse(JSON.stringify(candidateReceipt));
  const {descriptor, commitmentsHash} = a;
  if (ALLOW_PARA_IDS.indexOf(descriptor.paraId) < 0) {
    return;
  }

  const eventId = event.id;
  const atBlock = event.blockNumber;

  const _event = new CandidateIncludedEvent(eventId);
  _event.atBlock = atBlock;
  _event.paraId = descriptor.paraId;
  _event.signature = descriptor.signature;

  _event.timestamp = event.timestamp;
  await _event.save();
}
