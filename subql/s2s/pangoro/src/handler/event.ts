import {FastEvent} from '../helpers';
import * as storage from '../storage';
import {OnDemandType, RelayBlockOrigin} from '../storage';
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
      case 'grandpa:NewAuthorities': {
        await storage.storeNeedRelayBlock(
          this.event,
          RelayBlockOrigin.Mandatory,
        );
        return;
      }
      case 'bridgePangolinMessages:MessageAccepted': {
        await storage.storeNeedRelayBlock(
          this.event,
          RelayBlockOrigin.BridgePangolin,
          OnDemandType.SendMessage,
        );
        return;
      }
    }

    // dispatch

    if (eventSection === 'bridgePangolinDispatch') {
      await storage.storeNeedRelayBlock(
        this.event,
        RelayBlockOrigin.BridgePangolin,
        OnDemandType.Dispatch,
        eventMethod,
      );
      return;
    }
  }
}
