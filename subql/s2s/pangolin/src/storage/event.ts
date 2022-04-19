import {FastEvent} from '../helpers';
import {
  NeedRelayBlock
} from '../types';


export enum RelayBlockOrigin {
  Mandatory = 'mandatory',
  BridgePangoro = 'bridge-pangoro',
  BridgePangolinParachain = 'bridge-pangolin-parachain',
}


export async function storeNeedRelayBlock(
  event: FastEvent,
  origin: RelayBlockOrigin
) {
  const _event = new NeedRelayBlock(event.id);
  _event.atBlock = event.blockNumber;
  _event.hash = event.blockHash;
  _event.mandatory = origin == RelayBlockOrigin.Mandatory;
  _event.origin = origin;

  if (!_event.mandatory) {
    const data = event.data;
    const [laneId, messageNonce] = data as unknown as [string, number];
    _event.laneId = laneId;
    _event.messageNonce = messageNonce;
  }

  _event.timestamp = event.timestamp;
  await _event.save();
}
