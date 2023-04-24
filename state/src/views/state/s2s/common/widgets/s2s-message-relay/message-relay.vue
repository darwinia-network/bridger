<template>
  <v-card>
    <v-card-title>
      <h2 class="text-h5">{{ lane }}</h2>
    </v-card-title>

    <v-card-subtitle>
      <v-progress-circular v-if="loading.sourceChainOutboundLaneData" indeterminate :color="sourceChain.color"/>
      <span v-else class="subtitle-1">
        <span>[</span>
        <span>{{ source.sourceChainOutboundLaneData.latestReceivedNonce }}</span>
        <span>,</span>
        <span>{{ source.sourceChainOutboundLaneData.latestGeneratedNonce }}</span>
        <span>]</span>
      </span>
    </v-card-subtitle>

    <v-divider/>
    <v-card-text>
      <v-row>
        <v-col cols="6">
          <h3 class="subtitle-1">Delivery</h3>
          <v-divider/>
          <v-table density="compact">
            <thead>
            <tr>
              <th style="width: 70%">Title</th>
              <th>Value</th>
            </tr>
            </thead>
            <tbody>
            <tr>
              <td class="subtitle-2">Oldest unpruned nonce</td>
              <td>
                <v-progress-linear v-if="loading.sourceChainOutboundLaneData" :color="sourceChain.color"
                                   indeterminate/>
                <span v-else v-text="source.sourceChainOutboundLaneData.oldestUnprunedNonce"/>
              </td>
            </tr>
            <tr>
              <td class="subtitle-2">Latest received nonce</td>
              <td>
                <v-progress-linear v-if="loading.sourceChainOutboundLaneData" :color="sourceChain.color"
                                   indeterminate/>
                <span v-else v-text="source.sourceChainOutboundLaneData.latestReceivedNonce"/>
              </td>
            </tr>
            <tr>
              <td class="subtitle-2">Latest generated nonce</td>
              <td>
                <v-progress-linear v-if="loading.sourceChainOutboundLaneData" :color="sourceChain.color"
                                   indeterminate/>
                <span v-else v-text="source.sourceChainOutboundLaneData.latestGeneratedNonce"/>
              </td>
            </tr>
            </tbody>
          </v-table>
        </v-col>
        <v-col cols="6">
          <h3 class="subtitle-1">Receiving</h3>
          <v-divider/>
          <v-table density="compact">
            <thead>
            <tr>
              <th style="width: 45%">Title</th>
              <th>Value</th>
            </tr>
            </thead>
            <tbody>
            <tr>
              <td class="subtitle-2">Last relayed at source</td>
              <td>
                <v-progress-linear v-if="loading.targetChainInboundLaneData" :color="sourceChain.color"
                                   indeterminate/>
                <external-explorer v-else :identity="source.lastTargetChainRelayedBlockAtSource" type="block"
                                   :chain="targetChain"/>
              </td>
            </tr>
            <tr>
              <td class="subtitle-2">Last dispatched</td>
              <td>
                <v-progress-linear v-if="loading.targetChainInboundLaneData || loading.sourceChainOutboundLaneData"
                                   :color="sourceChain.color" indeterminate/>
                <span v-else :class="{
                    'text-red': source.maxConfirmEndAtTarget < source.sourceChainOutboundLaneData.latestReceivedNonce,
                    'text-green': source.maxConfirmEndAtTarget >= source.sourceChainOutboundLaneData.latestReceivedNonce,
                  }">
                    {{ source.maxConfirmEndAtTarget }}
                  </span>
              </td>
            </tr>
            </tbody>
          </v-table>
        </v-col>
        <v-expand-transition v-if="source.targetChainInboundLaneData">
          <v-col cols="12" v-if="!loading.targetChainInboundLaneData && cond.showDispatchInfo">
            <v-row>
              <v-col cols="12">
                <h3 class="subtitle-1">Dispatch info</h3>
                <v-divider class="mb-3"/>
              </v-col>
              <v-col cols="6" v-for="(item, ix) in source.targetChainInboundLaneData.relayers"
                     :key="`inbound-lane-data-${ix}`">
                <v-card :loading="loading.targetChainInboundLaneData">
                  <v-container>
                    <v-table density="compact">
                      <thead>
                      <tr>
                        <th style="width: 30%">Title</th>
                        <th>Value</th>
                      </tr>
                      </thead>
                      <tbody>
                      <tr>
                        <td class="subtitle-2">Relayer</td>
                        <td>
                          <external-explorer :identity="item.relayer" type="account"
                                             :chain="targetChain"/>
                        </td>
                      </tr>
                      <tr>
                        <td class="subtitle-2">Begin</td>
                        <td>{{ item.messages.begin }}</td>
                      </tr>
                      <tr>
                        <td class="subtitle-2">End</td>
                        <td>{{ item.messages.end }}</td>
                      </tr>
                      <tr>
                        <td class="subtitle-2">Result</td>
                        <td>{{ item.messages.dispatchResults }}</td>
                      </tr>
                      </tbody>
                    </v-table>
                  </v-container>
                </v-card>
              </v-col>
            </v-row>
          </v-col>
        </v-expand-transition>
      </v-row>
    </v-card-text>

    <v-card-actions v-if="source.targetChainInboundLaneData">
      <template v-if="!loading.targetChainInboundLaneData && source.targetChainInboundLaneData.relayers.length > 0">
        <v-tooltip  location="bottom">
          <template v-slot:activator="{ props }">
            <v-btn
              icon
              v-bind="props"
              @click="cond.showDispatchInfo = !cond.showDispatchInfo"
            >
              <v-icon>{{ cond.showDispatchInfo ? 'mdi-chevron-up' : 'mdi-chevron-down' }}</v-icon>
            </v-btn>
          </template>
          <span>Expanded more details</span>
        </v-tooltip>
      </template>
    </v-card-actions>
  </v-card>
</template>

<script lang="ts" setup>

import {defineProps, inject, onBeforeUnmount, onMounted, PropType, reactive, toRefs} from 'vue'
import {BridgeSubstrateChainInfo} from "@/types/app";
import {ApiPromise} from "@polkadot/api";
import ExternalExplorer from "@/components/widgets/external-explorer.vue";


const props = defineProps({
  lane: {
    type: String,
  },
  parachainBridge: {
    type: Boolean,
    default: false,
  },
  sourceChain: {
    type: Object as PropType<BridgeSubstrateChainInfo>,
  },
  sourceClient: {
    type: Object as PropType<ApiPromise>,
  },
  targetChain: {
    type: Object as PropType<BridgeSubstrateChainInfo>,
  },
  targetClient: {
    type: Object as PropType<ApiPromise>,
  },
});


interface _StateSource {
  sourceChainOutboundLaneData: _OutboundLaneData;
  targetChainInboundLaneData: _InboundLaneData;
  lastTargetChainRelayedBlockAtSource: string;
  maxConfirmEndAtTarget: number;
}

interface _StateLoading {
  sourceChainOutboundLaneData: boolean,
  targetChainInboundLaneData: boolean,
}

interface _InboundLaneData {

}

interface _OutboundLaneData {
  latestReceivedNonce: number;
  latestGeneratedNonce: number;
}

const state = reactive({
  source: {} as _StateSource,
  loading: {
    sourceChainOutboundLaneData: true,
    targetChainInboundLaneData: true,
  } as _StateLoading,
  subscriber: {
    sourceChainOutboundLaneData: null,
    targetRelayedBlockAtSource: null,
  },
  cond: {
    showDispatchInfo: false,
  },
});

const {loading, source, subscriber, cond} = toRefs(state);


async function initState() {
  const sourceChainBridgeTarget = props.sourceChain.bridge_target[props.targetChain.bridge_chain_name];
  subscriber.value.sourceChainOutboundLaneData = await props.sourceClient
    .query[sourceChainBridgeTarget.query_name.messages]
    .outboundLanes(props.lane, async (v: any) => {
      source.value.sourceChainOutboundLaneData = v.toJSON();
      loading.value.sourceChainOutboundLaneData = false;
    });

  if (props.parachainBridge) {
    await listenParachainInboundLaneData();
  } else {
    await listenTargetChainInboundLaneData();
  }
}

async function listenTargetChainInboundLaneData() {
  const sourceChainBridgeTarget = props.sourceChain.bridge_target[props.targetChain.bridge_chain_name];
  subscriber.value.targetRelayedBlockAtSource = await props.sourceClient
    .query[sourceChainBridgeTarget.query_name.grandpa]
    .bestFinalized(async (v: any) => {
      const [blockNumber, blockHash] = v.toJSON();
      source.value.lastTargetChainRelayedBlockAtSource = blockHash;
      await queryTargetChainInboundLaneData();
      loading.value.targetChainInboundLaneData = false;
    });
}

async function listenParachainInboundLaneData() {
  const sourceChainBridgeTarget = props.sourceChain.bridge_target[props.targetChain.bridge_chain_name];
  subscriber.value.targetRelayedBlockAtSource = await props.sourceClient
    .query[sourceChainBridgeTarget.query_name.parachains]
    .bestParaHeads(sourceChainBridgeTarget.para_id, async (v: any) => {
      // headHash
      const paraHead = v.toJSON();
      source.value.lastTargetChainRelayedBlockAtSource = paraHead.headHash;
      await queryTargetChainInboundLaneData();
      loading.value.targetChainInboundLaneData = false;
    });
}

async function queryTargetChainInboundLaneData() {
  // const targetChainBridgeTarget = props.targetChain.bridge_target[props.sourceChain.bridge_chain_name];
  // const atApi = await props.targetClient.at(source.value.lastTargetChainRelayedBlockAtSource);
  // const inboundLaneData = await atApi
  //   .query[targetChainBridgeTarget.query_name.messages]
  //   .inboundLanes(props.lane);
  // source.value.targetChainInboundLaneData = inboundLaneData.toJSON();
  // source.value.maxConfirmEndAtTarget = Math.max(
  //   ...source.value.targetChainInboundLaneData.relayers
  //     .map(item => item.messages.end)
  // )
}


onMounted(() => {
  initState();
});

onBeforeUnmount(() => {
  subscriber.value.sourceChainOutboundLaneData && subscriber.value.sourceChainOutboundLaneData();
  subscriber.value.targetRelayedBlockAtSource && subscriber.value.targetRelayedBlockAtSource();
});
</script>
