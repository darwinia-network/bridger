import {FastEvent} from '../helpers';
import {NeedRelayBlock} from '../types';
import {Justifications} from "@polkadot/types/interfaces/runtime/types";

export enum RelayBlockType {
  Mandatory = 'mandatory',
  OnDemand = 'on-demand',
}

export enum RelayBlockOrigin {
  Mandatory = 'mandatory',
  BridgePangolin = 'bridge-pangolin',
}

export async function storeNeedRelayBlock(
  event: FastEvent,
  origin: RelayBlockOrigin
) {
  const _event = new NeedRelayBlock(event.id);
  _event.atBlock = event.blockNumber;
  _event.hash = event.blockHash;
  _event.type = origin == RelayBlockOrigin.Mandatory ? RelayBlockType.Mandatory : RelayBlockType.OnDemand;
  _event.origin = origin;

  if (_event.type == RelayBlockType.OnDemand) {
    const data = event.data;
    const [laneId, messageNonce] = data as unknown as [string, number];
    _event.laneId = laneId;
    _event.messageNonce = messageNonce;
  }

  const block = event.block;
  if (block.justifications.isSome) {
    const justifications = block.justifications.value as unknown as Justifications;
    for (const justification of justifications) {
      const [consensusEngineId, encodedJustification] = justification;
      if (!consensusEngineId.isGrandpa) continue;
      const engineId = consensusEngineId.toString();
      if (engineId == 'FRNK') {
        _event.justification = encodedJustification.toString();
        break;
      }
    }
  }

  _event.timestamp = event.timestamp;
  await _event.save();
}
