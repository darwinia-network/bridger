import {BlockHandler} from './block';
import {FastEvent} from '../helpers';
import * as storage from '../storage';
import {storeAuthoritiesChangeSigned} from "../storage";

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
      case 'ethereumRelayAuthorities:MMRRootSigned': {
        await storage.storeMMRRootSignedEvent(this.event);
        return;
      }
      case 'ethereumRelayAuthorities:ScheduleMMRRoot': {
        await storage.storeScheduleMMRRootEvent(this.event);
        return;
      }
      case 'ethereumRelayAuthorities:ScheduleAuthoritiesChange': {
        await storage.storeScheduleAuthoritiesChange(this.event);
        return;
      }
      case 'ethereumRelayAuthorities:AuthoritiesChangeSigned': {
        await storage.storeAuthoritiesChangeSigned(this.event);
        return;
      }
      default: {
        // logger.info(`[event] Discard event: ${eventMethod} in block ${blockNumber}`);
      }
    }

  }
}
