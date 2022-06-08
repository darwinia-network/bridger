import {FastEvent} from '../helpers';
import * as storage from '../storage';
import {RelayBlockOrigin} from "../storage";
import {BlockHandler} from "./block";

export class EventHandler {
  private readonly event: FastEvent;

  constructor(event: FastEvent) {
    this.event = event;
  }

  public async save() {
    await BlockHandler.ensureBlock(this.event.blockHash);

    const eventId = this.event.id;
    const eventSection = this.event.section;
    const eventMethod = this.event.method;
    const blockNumber = this.event.blockNumber;
    const eventKey = `${eventSection}:${eventMethod}`;
    logger.info(`[event] Received event: [${eventKey}] [${eventId}] in block ${blockNumber}`);
    switch (eventKey) {
      case 'bridgeCrabMessages:MessageAccepted': {
        await storage.storeNeedRelayBlock(this.event, RelayBlockOrigin.BridgeCrab);
        return;
      }
    }

  }
}
