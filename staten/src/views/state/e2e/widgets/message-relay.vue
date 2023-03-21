<template>
  <v-row>
    <v-col cols="12">
      <v-card>
        <v-card-title>
          <h2 class="text-h5">Message</h2>
          <v-spacer/>
          <v-progress-circular v-if="loading.sourceChainOutboundLaneData" indeterminate :color="sourceChain.color"/>
          <span v-else class="subtitle-1">
            <span>[</span>
            <span>{{ source.sourceChainOutboundLaneData.latestReceivedNonce }}</span>
            <span>,</span>
            <span>{{ source.sourceChainOutboundLaneData.latestGeneratedNonce }}</span>
            <span>]</span>
          </span>
        </v-card-title>
        <v-divider/>
        <v-container>
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
                  <th style="width: 50%">Title</th>
                  <th>Value</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                  <td class="subtitle-2">Last relayed at source</td>
                  <td>
                    <v-progress-linear v-if="loading.lastTargetChainRelayedBlockAtSource" :color="sourceChain.color"
                                       indeterminate/>
                    <external-explorer v-else :identity="`${source.lastTargetChainRelayedBlockAtSource}`"
                                       type="block"
                                       :chain="targetChain"/>
                  </td>
                </tr>
                <tr>
                  <td class="subtitle-2">Last confirmed nonce</td>
                  <td>
                    <v-progress-linear v-if="loading.targetChainInboundLaneData" :color="sourceChain.color"
                                       indeterminate/>
                    <span v-else v-text="source.targetChainInboundLaneData.lastConfirmedNonce"/>
                  </td>
                </tr>
                <tr>
                  <td class="subtitle-2">Last delivered nonce</td>
                  <td>
                    <v-progress-linear v-if="loading.targetChainInboundLaneData" :color="sourceChain.color"
                                       indeterminate/>
                    <span v-else v-text="source.targetChainInboundLaneData.lastDeliveredNonce"/>
                  </td>
                </tr>
                <tr>
                  <td class="subtitle-2">Relayer range back</td>
                  <td>
                    <v-progress-linear v-if="loading.targetChainInboundLaneData" :color="sourceChain.color"
                                       indeterminate/>
                    <span v-else v-text="source.targetChainInboundLaneData.relayerRangeBack"/>
                  </td>
                </tr>
                <tr>
                  <td class="subtitle-2">Relayer range front</td>
                  <td>
                    <v-progress-linear v-if="loading.targetChainInboundLaneData" :color="sourceChain.color"
                                       indeterminate/>
                    <span v-else v-text="source.targetChainInboundLaneData.relayerRangeFront"/>
                  </td>
                </tr>
                </tbody>
              </v-table>
              <span class="body-2 red--text" v-if="cond.noTargetChainHeaderAtSource">
                The bridge not initialized or the header not relayed, please wait.
              </span>
            </v-col>
          </v-row>
        </v-container>
      </v-card>
    </v-col>
  </v-row>
</template>


<script lang="ts" setup>


import {defineProps, onBeforeUnmount, onMounted, PropType, reactive, toRaw, toRefs} from "vue";
import {BridgeEthereumChainInfo} from "@/types/app";
import {EvmClient} from "@/plugins/eth2/evm";
import {ExecutionClient} from "@/plugins/eth2/execution";
import {ConsensusClient} from "@/plugins/eth2/consensus";
import BigNumber from "bignumber.js";
import ExternalExplorer from "@/components/widgets/external-explorer.vue";

const props = defineProps({
  direction: {
    type: String,
  },
  sourceChain: {
    type: Object as PropType<BridgeEthereumChainInfo>,
  },
  sourceClient: {
    type: Object as PropType<EvmClient>,
  },
  targetChain: {
    type: Object as PropType<BridgeEthereumChainInfo>,
  },
  targetClient: {
    type: Object as PropType<ExecutionClient>,
  },
  consensusClient: {
    type: Object as PropType<ConsensusClient>,
  },
});

interface _StateSource {
  sourceChainOutboundLaneData: Record<string, any>;
  targetChainInboundLaneData: Record<string, any>;
  lastTargetChainRelayedBlockAtSource: Record<string, any>;
}

interface _StateCond {
  noTargetChainHeaderAtSource: boolean;
}

interface _StateLoading {
  sourceChainOutboundLaneData: boolean;
  targetChainInboundLaneData: boolean;
  lastTargetChainRelayedBlockAtSource: boolean;
}

const state = reactive({
  source: {} as _StateSource,
  cond: {
    noTargetChainHeaderAtSource: false,
  } as _StateCond,
  loading: {
    sourceChainOutboundLaneData: true,
    targetChainInboundLaneData: true,
    lastTargetChainRelayedBlockAtSource: true,
  } as _StateLoading,
  subscriber: {
    inboundLaneData: null,
    outboundLaneData: null,
  },
});

const {source, cond, loading, subscriber} = toRefs(state);


async function initState() {
  await queryOutbound();
  await queryInbound();
  subscriber.value.inboundLaneData = setInterval(queryInbound, 1000 * 15);
  subscriber.value.outboundLaneData = setInterval(queryOutbound, 1000 * 20);
}


async function queryOutbound() {
  const {sourceChain, targetChain, sourceClient} = props;
  const bridgeTargetAtSource = sourceChain.bridge_target[targetChain.bridge_chain_name];

  loading.value.sourceChainOutboundLaneData = true;
  const outboundLaneNonce = await toRaw(sourceClient).message({
    inbound: bridgeTargetAtSource.contract.inbound,
    outbound: bridgeTargetAtSource.contract.outbound,
  }).outboundLaneNonce();
  source.value.sourceChainOutboundLaneData = {
    latestGeneratedNonce: outboundLaneNonce.latest_generated_nonce,
    latestReceivedNonce: outboundLaneNonce.latest_received_nonce,
    oldestUnprunedNonce: outboundLaneNonce.oldest_unpruned_nonce,
  };
  loading.value.sourceChainOutboundLaneData = false;
}

async function queryInbound() {
  const {direction, sourceChain, targetChain} = props;
  const bridgeTargetAtSource = sourceChain.bridge_target[targetChain.bridge_chain_name];
  const bridgeTargetAtTarget = targetChain.bridge_target[sourceChain.bridge_chain_name];
  const sourceClient = toRaw(props.sourceClient);
  const targetClient = toRaw(props.targetClient);
  const consensusClient = toRaw(props.consensusClient);

  // query last relayed target chain header at source
  loading.value.lastTargetChainRelayedBlockAtSource = true;
  let lastTargetChainRelayedBlockAtSource;
  switch (direction) {
    case 'execution-to-evm':
      const blockNumber = await sourceClient
        .posaLightClient(bridgeTargetAtSource.contract.posa)
        .blockNumber();
      lastTargetChainRelayedBlockAtSource = new BigNumber(blockNumber.toString());
      break;
    case 'evm-to-execution':
      const finalizedHeader = await sourceClient
        .consensusLightClient(bridgeTargetAtSource.contract.lc_consensus)
        .finalizedHeader();
      const _consensusBlock = await consensusClient.block(finalizedHeader.slot);
      const consensusBlock = _consensusBlock.data;
      const executionStateRoot = await sourceClient
        .executionLightClient(bridgeTargetAtSource.contract.lc_execution)
        .stateRoot();
      const executionPayload = consensusBlock.message.body.execution_payload;
      if (executionPayload.state_root === executionStateRoot) {
        lastTargetChainRelayedBlockAtSource = new BigNumber(executionPayload.block_number);
      }
      break;
  }
  loading.value.lastTargetChainRelayedBlockAtSource = false;
  if (!lastTargetChainRelayedBlockAtSource) {
    cond.value.noTargetChainHeaderAtSource = true;
    loading.value.targetChainInboundLaneData = false;
    return;
  }
  cond.value.noTargetChainHeaderAtSource = false;

  // query inbound lane data from target chain at relayed header
  if (lastTargetChainRelayedBlockAtSource.isEqualTo(source.value.lastTargetChainRelayedBlockAtSource)) {
    return;
  }
  source.value.lastTargetChainRelayedBlockAtSource = lastTargetChainRelayedBlockAtSource;
  loading.value.targetChainInboundLaneData = true;
  const inboundLaneNonce = await targetClient.message({
    inbound: bridgeTargetAtTarget.contract.inbound,
    outbound: bridgeTargetAtTarget.contract.outbound,
  }).inboundLaneNonce(lastTargetChainRelayedBlockAtSource);
  source.value.targetChainInboundLaneData = {
    lastConfirmedNonce: inboundLaneNonce.last_confirmed_nonce,
    lastDeliveredNonce: inboundLaneNonce.last_delivered_nonce,
    relayerRangeBack: inboundLaneNonce.relayer_range_back,
    relayerRangeFront: inboundLaneNonce.relayer_range_front,
  };
  loading.value.targetChainInboundLaneData = false;
}


onMounted(() => {
  initState();
});


onBeforeUnmount(() => {
  // @ts-ignore
  subscriber.value.inboundLaneData && clearInterval(subscriber.value.inboundLaneData);
  subscriber.value.outboundLaneData && clearInterval(subscriber.value.outboundLaneData);
});


</script>

