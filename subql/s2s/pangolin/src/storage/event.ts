import {FastBlock, FastEvent} from '../helpers';
import {JustificationMapping, NeedRelayBlock} from '../types';
import {storeJustification} from "./block";

export enum RelayBlockType {
  Mandatory = 'mandatory',
  OnDemand = 'on-demand',
}

export enum OnDemandType {
  SendMessage = 'send-message',
  Dispatch = 'dispatch',
}

export enum RelayBlockOrigin {
  Mandatory = 'mandatory',
  BridgePangoro = 'bridge-pangoro',
  BridgePangolinParachain = 'bridge-pangolin-parachain',
}

export async function storeNeedRelayBlock(
  event: FastEvent,
  origin: RelayBlockOrigin,
  onDemandType?: OnDemandType,
  additional?: string,
) {
  const _event = new NeedRelayBlock(event.id);
  _event.blockNumber = event.blockNumber;
  _event.blockHash = event.blockHash;
  _event.type = origin == RelayBlockOrigin.Mandatory ? RelayBlockType.Mandatory : RelayBlockType.OnDemand;
  _event.origin = origin;
  _event.onDemandType = onDemandType;
  _event.additional = additional;

  const block = new FastBlock(event.block);
  const header = block.raw.block.header;
  _event.parentHash = header.parentHash.toString();
  _event.stateRoot = header.stateRoot.toString();
  _event.extrinsicsRoot = header.extrinsicsRoot.toString();
  _event.digest = header.digest.toHex();

  if (_event.type == RelayBlockType.OnDemand && onDemandType == OnDemandType.SendMessage) {
    const data = event.data;
    const [laneId, messageNonce] = data as unknown as [string, number];
    _event.laneId = laneId.toString().replace('0x', '');
    _event.messageNonce = messageNonce;
  }
  if (_event.type == RelayBlockType.OnDemand && onDemandType == OnDemandType.Dispatch) {
    const data = event.data;
    const [chainId, bridgeMessageIdOf] = data as unknown as [string, any];
    const [laneId, messageNonce] = bridgeMessageIdOf as unknown as [string, number];
    _event.laneId = laneId.toString().replace('0x', '');
    _event.messageNonce = messageNonce;
  }
  if (_event.type == RelayBlockType.Mandatory) {
    let justificationMapping = await JustificationMapping.get(block.number.toString());
    if (!justificationMapping) {
      await storeJustification(block);
      justificationMapping = await JustificationMapping.get(block.number.toString());
    }
    justificationMapping.mandatory = true;
    await justificationMapping.save();
  }

  _event.timestamp = event.timestamp;
  await _event.save();
}
