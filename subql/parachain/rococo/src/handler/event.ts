import {FastEvent} from '../helpers';
import * as storage from '../storage';

export class EventHandler {
  private readonly event: FastEvent;

  constructor(event: FastEvent) {
    this.event = event;
  }

  public async save() {
    // await BlockHandler.ensureBlock(this.event.blockHash);

    const eventSection = this.event.section;
    const eventMethod = this.event.method;
    const eventKey = `${eventSection}:${eventMethod}`;
    switch (eventKey) {
      case 'paraInclusion:CandidateIncluded': {
        await storage.storeCandidateIncluded(this.event);
        return;
      }
    }

  }
}
