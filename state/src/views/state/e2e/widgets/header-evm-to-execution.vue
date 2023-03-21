<template>
  <v-row>
    <v-col cols="12">
      <h2 class="text-h5 font-weight-thin">New message</h2>
      <v-table density="compact">

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
        <v-btn icon variant="plain" @click="cond.showConnectedMessageSignature = !cond.showConnectedMessageSignature">
          <v-icon>{{ cond.showConnectedMessageSignature ? 'mdi-chevron-up' : 'mdi-chevron-down' }}</v-icon>
        </v-btn>
        <v-expand-transition>
          <v-container
            class="pl-5"
            v-if="!loading.lastCollectedMessageRootSignature && cond.showConnectedMessageSignature"
          >
            <span class="body-2 font-weight-thin">Signatures</span>
            <v-list dense>
              <v-list-item :key="`signatures-1-${ix}`"
                           v-for="(signature, ix) in source.lastCollectedMessageRootSignature.signatures.nodes">
                <v-list-item-title>
                  <external-explorer :identity="signature.address" type="account" :chain="evmChain"/>
                </v-list-item-title>
              </v-list-item>
            </v-list>
          </v-container>
        </v-expand-transition>
      </v-table>
    </v-col>
    <v-col cols="12" v-if="source.lastCollectedAuthoritiesChange || source.lastCollectedAuthoritiesChange">
      <h2 class="text-h5 font-weight-thin">Authorities</h2>
      <v-table density="compact">

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
        <v-btn icon variant="plain" @click="cond.showConnectedAuthoritiesSignature = !cond.showConnectedAuthoritiesSignature">
          <v-icon>{{ cond.showConnectedAuthoritiesSignature ? 'mdi-chevron-up' : 'mdi-chevron-down' }}</v-icon>
        </v-btn>
        <v-expand-transition>
          <v-container class="pl-5"
                       v-if="!loading.lastCollectedAuthoritiesChange && cond.showConnectedAuthoritiesSignature">
            <span class="body-2 font-weight-thin">Signatures</span>
            <v-list dense>
              <v-list-item :key="`signatures-2-${ix}`"
                           v-for="(signature, ix) in source.lastCollectedAuthoritiesChange.signatures.nodes">
                <v-list-item-title>
                  <external-explorer :identity="signature.address" type="account" :chain="evmChain"/>
                </v-list-item-title>
              </v-list-item>
            </v-list>
          </v-container>
        </v-expand-transition>
      </v-table>
    </v-col>
  </v-row>
</template>


<script lang="ts" setup>

import {defineProps, inject, onBeforeUnmount, onMounted, PropType, reactive, toRefs} from 'vue'
import {BridgeEthereumChainInfo} from "@/types/app";
import {EvmClient} from "@/plugins/eth2/evm";
import {ExecutionClient} from "@/plugins/eth2/execution";
import {Subql} from "@/plugins/subql";
import ExternalExplorer from "@/components/widgets/external-explorer.vue";


const subql = inject('subql') as Subql;

const props = defineProps({
  evmChain: {
    type: Object as PropType<BridgeEthereumChainInfo>,
  },
  evmClient: {
    type: Object as PropType<EvmClient>,
  },
  executionClient: {
    type: Object as PropType<ExecutionClient>,
  },
});

interface _StateSource {
  lastCollectingMessageRootSignature: Record<string, any>;
  lastCollectedMessageRootSignature: Record<string, any>;
  lastCollectingAuthoritiesChange: Record<string, any>;
  lastCollectedAuthoritiesChange: Record<string, any>;
}

interface _StateLoading {
  lastCollectingMessageRootSignature: boolean,
  lastCollectedMessageRootSignature: boolean,
  lastCollectingAuthoritiesChange: boolean,
  lastCollectedAuthoritiesChange: boolean,
}

interface _StateCond {
  showConnectedMessageSignature: boolean;
  showConnectedAuthoritiesSignature: boolean;
}

const state = reactive({
  source: {} as _StateSource,
  loading: {
    lastCollectingMessageRootSignature: true,
    lastCollectedMessageRootSignature: true,
    lastCollectingAuthoritiesChange: true,
    lastCollectedAuthoritiesChange: true,
  } as _StateLoading,
  cond: {
    showConnectedMessageSignature: false,
    showConnectedAuthoritiesSignature: false,
  } as _StateCond,
  subscriber: {
    queryMessage: null,
  },
});

const {source, loading, cond, subscriber} = toRefs(state);

async function initState() {
  await queryAuthorityChange();
  await queryMessage();
  subscriber.value.queryMessage = setInterval(queryMessage, 1000 * 14);
}


async function queryMessage() {
  loading.value.lastCollectingMessageRootSignature = true;
  source.value.lastCollectingMessageRootSignature = await subql
    .bridge_e2e(props.evmChain.subql)
    .lastCollectingMessageRootSignatureEvent();
  loading.value.lastCollectingMessageRootSignature = false;

  loading.value.lastCollectedMessageRootSignature = true;
  source.value.lastCollectedMessageRootSignature = await subql
    .bridge_e2e(props.evmChain.subql)
    .lastCollectedMessageRootSignatureEvent();
  loading.value.lastCollectedMessageRootSignature = false;
}

async function queryAuthorityChange() {
  source.value.lastCollectingAuthoritiesChange = await subql
    .bridge_e2e(props.evmChain.subql)
    .lastCollectingAuthoritiesChangeSignatureEvent();
  loading.value.lastCollectingAuthoritiesChange = false;

  source.value.lastCollectedAuthoritiesChange = await subql
    .bridge_e2e(props.evmChain.subql)
    .lastCollectedAuthoritiesChangeSignatureEvent();
  loading.value.lastCollectedAuthoritiesChange = false;

}


onMounted(() => {
  initState();
});


onBeforeUnmount(() => {
  // @ts-ignore
  subscriber.value.queryMessage && clearInterval(subscriber.value.queryMessage);
});

</script>

