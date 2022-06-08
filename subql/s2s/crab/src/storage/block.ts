import {Justifications} from "@polkadot/types/interfaces/runtime/types";
import {JustificationMapping} from "../types";
import {FastBlock} from "../helpers";



export async function storeJustification(
  block: FastBlock,
) {
  const rawBlock = block.raw;
  if (rawBlock.justifications.isNone) {
    return;
  }
  const justifications = rawBlock.justifications.value as unknown as Justifications;

  for (const justification of justifications) {
    const [consensusEngineId, encodedJustification] = justification;
    if (!consensusEngineId.isGrandpa) continue;

    const _justification = new JustificationMapping(block.number.toString());
    _justification.blockNumber = block.number;
    _justification.blockHash = block.hash;
    _justification.mandatory = false;
    _justification.justification = encodedJustification.toString();
    await _justification.save();
    break;
  }

}
