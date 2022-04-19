import {SubstrateEvent} from "@subql/types";

export class FastEvent {
  private readonly event: SubstrateEvent;

  constructor(event: SubstrateEvent) {
    this.event = event;
  }

  get raw() {
    return this.event;
  }

  get index() {
    return this.event.idx;
  }

  get block() {
    return this.event.block
  }

  get blockNumber() {
    return this.event.block.block.header.number.toNumber();
  }

  get blockHash() {
    return this.event.block.block.hash.toString();
  }

  get events() {
    return this.event.block.events;
  }

  get section() {
    return this.event.event.section;
  }

  get method() {
    return this.event.event.method;
  }

  get data() {
    return this.event.event.data;
  }

  get extrinsicHash() {
    const i = this.event?.extrinsic?.extrinsic?.hash?.toString();

    return i === 'null' ? undefined : i;
  }

  get id() {
    return `${this.blockNumber}-${this.index}`;
  }

  get timestamp() {
    return this.event.block.timestamp;
  }

}
