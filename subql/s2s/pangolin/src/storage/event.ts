import {FastEvent, ALLOW_PARA_IDS} from '../helpers';
import {
  // CandidateIncludedEvent
} from '../types';


export async function storeCandidateIncluded(event: FastEvent) {
  const data = event.data;
  // const [candidateReceipt, headData, coreIndex, groupIndex] = data;
  // const {descriptor} = candidateReceipt.toJSON() as unknown as CandidateReceipt;
  // if (ALLOW_PARA_IDS.indexOf(descriptor.paraId) < 0) {
  //   return;
  // }
  //
  // const eventId = event.id;
  // const blockNumber = event.blockNumber;
  //
  // const _event = new CandidateIncludedEvent(eventId);
  // _event.includedRelayBlock = blockNumber;
  // _event.paraId = descriptor.paraId;
  // _event.paraHead = descriptor.paraHead;
  // _event.relayParent = descriptor.relayParent;
  // _event.signature = descriptor.signature;
  //
  // _event.timestamp = event.timestamp;
  // await _event.save();
}
