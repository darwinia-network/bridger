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
              <v-simple-table dense>
                <template v-slot:default>
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
                </template>
              </v-simple-table>
            </v-col>
            <v-col cols="6">
              <h3 class="subtitle-1">Receiving</h3>
              <v-divider/>
              <v-simple-table dense>
                <template v-slot:default>
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
                </template>
              </v-simple-table>
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

<script>

import BigNumber from "bignumber.js";
import ExternalExplorer from "@/components/widgets/external-explorer";

async function initState(vm) {
  await queryOutbound(vm);
  await queryInbound(vm);
  vm.subscriber.inboundLaneData = setInterval(() => queryInbound(vm), 1000 * 15);
  vm.subscriber.outboundLaneData = setInterval(() => queryOutbound(vm), 1000 * 20);
}

async function queryOutbound(vm) {
  const bridgeTargetAtSource = vm.sourceChain.bridge_target[vm.targetChain.bridge_chain_name];

  vm.loading.sourceChainOutboundLaneData = true;
  const outboundLaneNonce = await vm.sourceClient.message({
    inbound: bridgeTargetAtSource.contract.inbound,
    outbound: bridgeTargetAtSource.contract.outbound,
  }).outboundLaneNonce();
  vm.source.sourceChainOutboundLaneData = {
    latestGeneratedNonce: outboundLaneNonce.latest_generated_nonce,
    latestReceivedNonce: outboundLaneNonce.latest_received_nonce,
    oldestUnprunedNonce: outboundLaneNonce.oldest_unpruned_nonce,
  };
  vm.loading.sourceChainOutboundLaneData = false;
}

async function queryInbound(vm) {
  const bridgeTargetAtSource = vm.sourceChain.bridge_target[vm.targetChain.bridge_chain_name];
  const bridgeTargetAtTarget = vm.targetChain.bridge_target[vm.sourceChain.bridge_chain_name];

  vm.loading.lastTargetChainRelayedBlockAtSource = true;
  let lastTargetChainRelayedBlockAtSource;
  switch (vm.direction) {
    case 'execution-to-evm':
      const blockNumber = await vm.sourceClient
        .posaLightClient(bridgeTargetAtSource.contract.posa)
        .blockNumber();
      lastTargetChainRelayedBlockAtSource = new BigNumber(blockNumber.toString());
      break;
    case 'evm-to-execution':
      const finalizedHeader = await vm.sourceClient
        .consensusLightClient(bridgeTargetAtSource.contract.lc_consensus)
        .finalizedHeader();
      const _consensusBlock = await vm.consensusClient.block(finalizedHeader.slot);
      const consensusBlock = _consensusBlock.data;
      const executionStateRoot = await vm.sourceClient
        .executionLightClient(bridgeTargetAtSource.contract.lc_execution)
        .stateRoot();
      const executionPayload = consensusBlock.message.body.execution_payload;
      if (executionPayload.state_root === executionStateRoot) {
        lastTargetChainRelayedBlockAtSource = new BigNumber(executionPayload.block_number);
      }
      break;
  }
  vm.source.lastTargetChainRelayedBlockAtSource = lastTargetChainRelayedBlockAtSource;
  vm.loading.lastTargetChainRelayedBlockAtSource = false;
  if (!lastTargetChainRelayedBlockAtSource) {
    vm.cond.noTargetChainHeaderAtSource = true;
    vm.loading.targetChainInboundLaneData = false;
    return;
  }
  vm.cond.noTargetChainHeaderAtSource = false;

  vm.loading.targetChainInboundLaneData = true;
  const inboundLaneNonce = await vm.targetClient.message({
    inbound: bridgeTargetAtTarget.contract.inbound,
    outbound: bridgeTargetAtTarget.contract.outbound,
  }).inboundLaneNonce(lastTargetChainRelayedBlockAtSource);
  vm.source.targetChainInboundLaneData = {
    lastConfirmedNonce: inboundLaneNonce.last_confirmed_nonce,
    lastDeliveredNonce: inboundLaneNonce.last_delivered_nonce,
    relayerRangeBack: inboundLaneNonce.relayer_range_back,
    relayerRangeFront: inboundLaneNonce.relayer_range_front,
  };
  vm.loading.targetChainInboundLaneData = false;
}

export default {
  components: {ExternalExplorer},
  props: {
    direction: {
      type: String,
    },
    sourceChain: {
      type: Object,
    },
    targetChain: {
      type: Object,
    },
    sourceClient: {
      type: Object,
    },
    targetClient: {
      type: Object,
    },
    consensusClient: {
      type: Object,
    },
  },
  data: () => ({
    source: {
      sourceChainOutboundLaneData: null,
      targetChainInboundLaneData: null,
      lastTargetChainRelayedBlockAtSource: null,
    },
    cond: {
      noTargetChainHeaderAtSource: false,
    },
    loading: {
      sourceChainOutboundLaneData: true,
      targetChainInboundLaneData: true,
      lastTargetChainRelayedBlockAtSource: true,
    },
    subscriber: {
      inboundLaneData: null,
      outboundLaneData: null,
    }
  }),
  created() {
    initState(this)
  },
  destroyed() {
    const vm = this;
    vm.subscriber.inboundLaneData && clearInterval(vm.subscriber.inboundLaneData);
    vm.subscriber.outboundLaneData && clearInterval(vm.subscriber.outboundLaneData);
  }
}
</script>

<style scoped>

</style>
