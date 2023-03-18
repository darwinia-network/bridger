<template>
  <v-container fluid>
    <v-row>
      <v-col cols="12">
        <span v-if="pickedChain.evm && pickedChain.execution">abcd</span>
        <bridge-skeleton
          ref="left_to_right"
          chain-type="execution"
          :key="`e2e-${pickedChain.evm.name}-${pickedChain.execution.name}`"
          :source-client="pickedClient.evm"
          :source-chain="pickedChain.evm"
          :target-chain="pickedChain.execution"
          v-if="pickedChain.evm && pickedChain.execution"
        >
          asdfasdf
          <v-progress-linear
            class="mt-15"
            :color="pickedChain.evm.color"
            indeterminate
            v-if="loading.evmClient || loading.executionClient"
          />
          <evm-to-execution
            v-else
            :evm-chain="pickedChain.evm"
            :execution-chain="pickedChain.execution"
            :consensus-chain="pickedChain.consensus"
            :evm-client="pickedClient.evm"
            :execution-client="pickedClient.execution"
            :consensus-client="pickedClient.consensus"
          />
        </bridge-skeleton>
        <v-progress-linear v-else class="mt-15" indeterminate/>
      </v-col>
      <v-col cols="12">
        <v-divider/>
      </v-col>
      <v-col cols="12">
        <bridge-skeleton
          ref="right_to_left"
          chain-type="execution"
          :key="`e2e-${pickedChain.execution.name}-${pickedChain.evm.name}`"
          :source-client="pickedClient.execution"
          :source-chain="pickedClient.execution"
          :target-chain="pickedChain.evm"
          v-if="pickedChain.evm && pickedChain.execution"
        >
          <v-progress-linear
            class="mt-15"
            :color="pickedChain.execution.color"
            indeterminate
            v-if="loading.evmClient || loading.executionClient"
          />
          <execution-to-evm
            v-else
            :evm-chain="pickedChain.evm"
            :execution-chain="pickedChain.execution"
            :consensus-chain="pickedChain.consensus"
            :evm-client="pickedClient.evm"
            :execution-client="pickedClient.execution"
            :consensus-client="pickedClient.consensus"
          />
        </bridge-skeleton>
        <v-progress-linear v-else class="mt-15" indeterminate/>
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts" setup>
import BridgeSkeleton from '@/components/skeleton/bridge-skeleton';
import ExecutionToEvm from '@/views/state/e2e/wrapper/execution-to-evm';

import * as dataSource from '@/data/data_source';
import EvmToExecution from "@/views/state/e2e/wrapper/evm-to-execution";


import {defineProps, inject, onMounted, PropType, reactive, toRefs} from 'vue'
import {
  BridgeEthereumChainInfo,
  SubstrateEvmWithEthereumChainPair,
  SubstrateEvmWithEthereumClientPair
} from "@/types/app";
import {Bridge} from "@/types/bridge";
import {useRouter} from "vue-router";
import {Eth2Client} from "@/plugins/eth2";

const router = useRouter();
const eth2 = inject('eth2') as Eth2Client;

const props = defineProps({
  bridge: {
    type: Object as PropType<Bridge>,
  },
})

interface _StateLoading {
  evmClient: boolean,
  executionClient: boolean,
  consensusClient: boolean,
}

const state = reactive({
  pickedChain: {} as SubstrateEvmWithEthereumChainPair,
  pickedClient: {} as SubstrateEvmWithEthereumClientPair,
  loading: {
    evmClient: true,
    executionClient: true,
    consensusClient: true,
  } as _StateLoading,
});

const {pickedChain, pickedClient, loading} = toRefs(state);

async function initState() {
  if (!props.bridge) {
    await router.push({path: '/'});
    return;
  }
  const name = props.bridge.name;
  const [evmChainName, executionChainName] = name.split('-');
  const [evmChain, executionChain] = [
    dataSource.chainInfo(evmChainName) as BridgeEthereumChainInfo,
    dataSource.chainInfo(executionChainName) as BridgeEthereumChainInfo,
  ];
  if (!evmChain || !executionChain) {
    await router.push({path: '/'});
    return;
  }
  const consensusChain = dataSource.chainInfo(executionChain.consensus_chain);
  pickedChain.value.evm = {...evmChain, bridge_chain_name: evmChainName};
  pickedChain.value.execution = {...executionChain, bridge_chain_name: executionChainName};
  pickedChain.value.consensus = {...consensusChain, bridge_chain_name: executionChain.consensus_chain};
  pickedClient.value.evm = eth2.evm({endpoint: pickedChain.value.evm.endpoint.evm});
  pickedClient.value.execution = eth2.execution({endpoint: pickedChain.value.execution.endpoint.http});
  pickedClient.value.consensus = eth2.consensus({endpoint: pickedChain.value.consensus.endpoint.http});
  loading.value.evmClient = false;
  loading.value.executionClient = false;
  loading.value.consensusClient = false;
}


onMounted(() => {
  initState();
});

</script>
