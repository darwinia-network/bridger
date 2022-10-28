<template>
  <v-row>
    <v-col cols="12">
      <h2 class="text-h5 font-weight-thin">New message</h2>
      <v-simple-table dense>
        <template v-slot:default>
          <thead>
          <tr>
            <th style="width: 30%">Title</th>
            <th>Value</th>
          </tr>
          </thead>
          <span class="body-2 font-weight-thin">Last collecting</span>
          <tbody>
          <tr>
            <td>Block hash</td>
            <td>
              <v-progress-linear v-if="loading.lastCollectingMessageRootSignature" :color="evmChain.color"
                                 indeterminate/>
              <external-explorer v-else :identity="source.lastCollectingMessageRootSignature.blockHash" type="block"
                                 :chain="evmChain"/>
            </td>
          </tr>
          <tr>
            <td>Block number</td>
            <td>
              <v-progress-linear v-if="loading.lastCollectingMessageRootSignature" :color="evmChain.color"
                                 indeterminate/>
              <external-explorer v-else :identity="`${source.lastCollectingMessageRootSignature.blockNumber}`"
                                 type="block"
                                 :chain="evmChain"/>
            </td>
          </tr>
          </tbody>
          <span class="body-2 font-weight-thin">Last collected</span>
          <tbody>
          <tr>
            <td>Block hash</td>
            <td>
              <v-progress-linear v-if="loading.lastCollectedMessageRootSignature" :color="evmChain.color"
                                 indeterminate/>
              <external-explorer v-else :identity="source.lastCollectedMessageRootSignature.blockHash" type="block"
                                 :chain="evmChain"/>
            </td>
          </tr>
          <tr>
            <td>Block number</td>
            <td>
              <v-progress-linear v-if="loading.lastCollectedMessageRootSignature" :color="evmChain.color"
                                 indeterminate/>
              <external-explorer v-else :identity="`${source.lastCollectedMessageRootSignature.blockNumber}`"
                                 type="block"
                                 :chain="evmChain"/>
            </td>
          </tr>
          <tr>
            <td>Commitment block number</td>
            <td>
              <v-progress-linear v-if="loading.lastCollectedMessageRootSignature" :color="evmChain.color"
                                 indeterminate/>
              <external-explorer v-else :identity="`${source.lastCollectedMessageRootSignature.commitmentBlockNumber}`"
                                 type="block" :chain="evmChain"/>
            </td>
          </tr>
          <tr>
            <td>Commitment nonce</td>
            <td>
              <v-progress-linear v-if="loading.lastCollectedMessageRootSignature" :color="evmChain.color"
                                 indeterminate/>
              <span v-else>{{ source.lastCollectedMessageRootSignature.commitmentNonce }}</span>
            </td>
          </tr>
          </tbody>
          <v-btn icon @click="cond.showConnectedMessageSignature = !cond.showConnectedMessageSignature">
            <v-icon>{{ cond.showConnectedMessageSignature ? 'mdi-chevron-up' : 'mdi-chevron-down' }}</v-icon>
          </v-btn>
          <v-expand-transition>
            <v-container
              class="pl-5"
              v-if="!loading.lastCollectedMessageRootSignature && cond.showConnectedMessageSignature"
            >
              <span class="body-2 font-weight-thin">Signatures</span>
              <v-list dense>
                <v-list-item v-for="(signature, ix) in source.lastCollectedMessageRootSignature.signatures.nodes">
                  <v-list-item-title>
                    <external-explorer :identity="signature.address" type="account" :chain="evmChain"/>
                  </v-list-item-title>
                </v-list-item>
              </v-list>
            </v-container>
          </v-expand-transition>
        </template>
      </v-simple-table>
    </v-col>
    <v-col cols="12" v-if="source.lastCollectedAuthoritiesChange || source.lastCollectedAuthoritiesChange">
      <h2 class="text-h5 font-weight-thin">Authorities</h2>
      <v-simple-table dense>
        <template v-slot:default>
          <thead>
          <tr>
            <th style="width: 30%">Title</th>
            <th>Value</th>
          </tr>
          </thead>
          <span class="body-2 font-weight-thin">Last collecting</span>
          <tbody>
          <tr>
            <td>Block hash</td>
            <td>
              <v-progress-linear v-if="loading.lastCollectedAuthoritiesChange" :color="evmChain.color"
                                 indeterminate/>
              <external-explorer v-else :identity="source.lastCollectedAuthoritiesChange.blockHash" type="block"
                                 :chain="evmChain"/>
            </td>
          </tr>
          <tr>
            <td>Block number</td>
            <td>
              <v-progress-linear v-if="loading.lastCollectedAuthoritiesChange" :color="evmChain.color"
                                 indeterminate/>
              <external-explorer v-else :identity="`${source.lastCollectedAuthoritiesChange.blockNumber}`"
                                 type="block"
                                 :chain="evmChain"/>
            </td>
          </tr>
          </tbody>
          <span class="body-2 font-weight-thin">Last collected</span>
          <tbody>
          <tr>
            <td>Block hash</td>
            <td>
              <v-progress-linear v-if="loading.lastCollectedAuthoritiesChange" :color="evmChain.color"
                                 indeterminate/>
              <external-explorer v-else :identity="source.lastCollectedAuthoritiesChange.blockHash" type="block"
                                 :chain="evmChain"/>
            </td>
          </tr>
          <tr>
            <td>Block number</td>
            <td>
              <v-progress-linear v-if="loading.lastCollectedAuthoritiesChange" :color="evmChain.color"
                                 indeterminate/>
              <external-explorer v-else :identity="`${source.lastCollectedAuthoritiesChange.blockNumber}`"
                                 type="block"
                                 :chain="evmChain"/>
            </td>
          </tr>
          </tbody>
          <v-btn icon @click="cond.showConnectedAuthoritiesSignature = !cond.showConnectedAuthoritiesSignature">
            <v-icon>{{ cond.showConnectedAuthoritiesSignature ? 'mdi-chevron-up' : 'mdi-chevron-down' }}</v-icon>
          </v-btn>
          <v-expand-transition>
            <v-container class="pl-5" v-if="!loading.lastCollectedAuthoritiesChange && cond.showConnectedAuthoritiesSignature">
              <span class="body-2 font-weight-thin">Signatures</span>
              <v-list dense>
                <v-list-item v-for="(signature, ix) in source.lastCollectedAuthoritiesChange.signatures.nodes">
                  <v-list-item-title>
                    <external-explorer :identity="signature.address" type="account" :chain="evmChain"/>
                  </v-list-item-title>
                </v-list-item>
              </v-list>
            </v-container>
          </v-expand-transition>
        </template>
      </v-simple-table>
    </v-col>
  </v-row>
</template>

<script>

import ExternalExplorer from "@/components/widgets/external-explorer";
import EllipsisText from "@/components/widgets/ellipsis-text";

async function initState(vm) {
  await queryAuthorityChange(vm);
  await queryMessage(vm);
  vm.subscriber.queryMessage = setInterval(() => queryMessage(vm), 1000 * 14);
}

async function queryMessage(vm) {
  vm.loading.lastCollectingMessageRootSignature = true;
  vm.source.lastCollectingMessageRootSignature = await vm.$subql
    .bridge_e2e(vm.evmChain.subql)
    .lastCollectingMessageRootSignatureEvent();
  vm.loading.lastCollectingMessageRootSignature = false;

  vm.loading.lastCollectedMessageRootSignature = true;
  vm.source.lastCollectedMessageRootSignature = await vm.$subql
    .bridge_e2e(vm.evmChain.subql)
    .lastCollectedMessageRootSignatureEvent();
  vm.loading.lastCollectedMessageRootSignature = false;
}

async function queryAuthorityChange(vm) {
  vm.source.lastCollectingAuthoritiesChange = await vm.$subql
    .bridge_e2e(vm.evmChain.subql)
    .lastCollectingAuthoritiesChangeSignatureEvent();
  vm.loading.lastCollectingAuthoritiesChange = false;

  vm.source.lastCollectedAuthoritiesChange = await vm.$subql
    .bridge_e2e(vm.evmChain.subql)
    .lastCollectedAuthoritiesChangeSignatureEvent();
  vm.loading.lastCollectedAuthoritiesChange = false;

}


export default {
  components: {EllipsisText, ExternalExplorer},
  props: {
    evmChain: {
      type: Object,
    },
    evmClient: {
      type: Object,
    },
    executionChain: {
      type: Object,
    },
  },
  data: () => ({
    source: {
      lastCollectingMessageRootSignature: null,
      lastCollectedMessageRootSignature: null,
      lastCollectingAuthoritiesChange: null,
      lastCollectedAuthoritiesChange: null,
    },
    loading: {
      lastCollectingMessageRootSignature: true,
      lastCollectedMessageRootSignature: true,
      lastCollectingAuthoritiesChange: true,
      lastCollectedAuthoritiesChange: true,
    },
    cond: {
      showConnectedMessageSignature: false,
      showConnectedAuthoritiesSignature: false,
    },
    subscriber: {
      queryMessage: null,
    },
  }),
  created() {
    initState(this);
  },
  destroyed() {
    const vm = this;
    vm.subscriber.queryMessage && clearInterval(vm.subscriber.queryMessage);
  }
}
</script>

<style scoped>

</style>
