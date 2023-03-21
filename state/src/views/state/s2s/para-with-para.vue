<template>
  <v-container fluid>
    <v-row>
      <v-col cols="12">
        <bridge-skeleton
          v-if="pickedClient.leftParaClient && pickedChain.rightParaChain"
          chain-type="substrate"
          :key="`s2s-parachain-${pickedChain.leftParaChain.name}-${pickedChain.rightParaChain.name}`"
          :source-client="pickedClient.leftParaClient"
          :source-chain="pickedChain.leftParaChain"
          :target-chain="pickedChain.rightParaChain"
        >
          <v-progress-linear
            class="mt-15"
            :color="pickedChain.leftParaChain.color"
            indeterminate
            v-if="!pickedClient.leftParaClient || !pickedClient.rightParaClient || !pickedClient.leftRelayClient"
          />
          <template v-else>
            <bridge-s2s-parachain-header
              :para-chain="pickedChain.leftParaChain"
              :para-client="pickedClient.leftParaClient"
              :target-chain="pickedChain.rightParaChain"
              :target-client="pickedClient.rightParaClient"
              :relay-chain="pickedChain.leftRelayChain"
              :relay-client="pickedClient.leftRelayClient"
            />
            <bridge-s2s-parachain-message
              :para-chain="pickedChain.leftParaChain"
              :para-client="pickedClient.leftParaClient"
              :relay-chain="pickedChain.leftRelayChain"
              :relay-client="pickedClient.leftRelayClient"
              :target-chain="pickedChain.rightParaChain"
              :target-client=" pickedClient.rightParaClient"
            />
          </template>
        </bridge-skeleton>
        <v-progress-linear v-else class="mt-15" indeterminate/>
      </v-col>
      <v-col cols="12">
        <v-divider/>
      </v-col>
      <v-col cols="12">
        <bridge-skeleton
          v-if="pickedChain.leftParaChain && pickedClient.rightParaClient"
          chain-type="substrate"
          :key="`s2s-parachain-${pickedChain.rightParaChain.name}-${pickedChain.leftParaChain.name}`"
          :source-client="pickedClient.rightParaClient"
          :source-chain="pickedChain.rightParaChain"
          :target-chain="pickedChain.leftParaChain"
        >
          <v-progress-linear
            class="mt-15"
            :color="pickedChain.rightParaChain.color"
            indeterminate
            v-if="!pickedClient.rightParaClient || !pickedClient.leftParaClient || !pickedClient.rightRelayClient"
          />
          <template v-else>
            <bridge-s2s-parachain-header
              :para-chain="pickedChain.rightParaChain"
              :para-client="pickedClient.rightParaClient"
              :target-chain="pickedChain.leftParaChain"
              :target-client="pickedClient.leftParaClient"
              :relay-chain="pickedChain.rightRelayChain"
              :relay-client="pickedClient.rightRelayClient"
            />
            <bridge-s2s-parachain-message
              :para-chain="pickedChain.rightParaChain"
              :para-client="pickedClient.rightParaClient"
              :relay-chain="pickedChain.rightRelayChain"
              :relay-client="pickedClient.rightRelayClient"
              :target-chain="pickedChain.leftParaChain"
              :target-client=" pickedClient.leftParaClient"
            />
          </template>
        </bridge-skeleton>
        <v-progress-linear v-else class="mt-15" indeterminate/>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts" setup>

import BridgeSkeleton from "@/components/skeleton/bridge-skeleton.vue"

import {defineProps, onMounted, PropType, reactive, toRefs, provide, inject, onBeforeUnmount} from 'vue'

import * as dataSource from '@/data/data_source'

import {Bridge} from "@/types/bridge";
import {useRouter} from "vue-router";
import {SubstrateChainInfo} from "@/types/chain";
import {ApiPromise, WsProvider} from "@polkadot/api";
import {ParaWithParaChainPair, ParaWithParaClientPair} from "@/types/app";
import BridgeS2sParachainHeader from "@/views/state/s2s/common/bridge-s2s-parachain-header.vue";
import BridgeS2sParachainMessage from "@/views/state/s2s/common/bridge-s2s-parachain-message.vue";

const router = useRouter();

const props = defineProps({
  bridge: {
    type: Object as PropType<Bridge>,
  },
})

const state = reactive({
  pickedChain: {} as ParaWithParaChainPair,
  pickedClient: {} as ParaWithParaClientPair,
  loading: {
    leftParaClient: false,
    leftRelayClient: false,
    rightParaClient: false,
    rightRelayClient: false,
  },
});


const {
  pickedChain,
  pickedClient,
  loading,
} = toRefs(state);


async function initState() {
  if (!props.bridge) {
    await router.push({path: '/'});
    return;
  }
  const name = props.bridge?.name;
  const [leftParaChainName, rightParaChainName] = name.split('-');
  const [leftParaChain, rightParaChain] = [
    dataSource.chainInfo(leftParaChainName) as SubstrateChainInfo,
    dataSource.chainInfo(rightParaChainName) as SubstrateChainInfo,
  ];
  // @ts-ignore
  const leftRelayChainName = leftParaChain.bridge_target[rightParaChainName].relay_chain;
  // @ts-ignore
  const rightRelayChainName = rightParaChain.bridge_target[leftParaChainName].relay_chain;
  const [leftRelayChain, rightRelayChain] = [
    dataSource.chainInfo(leftRelayChainName) as SubstrateChainInfo,
    dataSource.chainInfo(rightRelayChainName) as SubstrateChainInfo,
  ];
  pickedChain.value = {
    leftParaChain: {...leftParaChain, bridge_chain_name: leftParaChainName},
    leftRelayChain: {...leftRelayChain, bridge_chain_name: leftRelayChainName},
    rightParaChain: {...rightParaChain, bridge_chain_name: rightParaChainName},
    rightRelayChain: {...rightRelayChain, bridge_chain_name: rightRelayChainName},
  };

  const leftParaChainProvider = new WsProvider(leftParaChain.endpoint.websocket);
  const leftRelayChainProvider = new WsProvider(leftRelayChain.endpoint.websocket);
  const rightParaChainProvider = new WsProvider(rightParaChain.endpoint.websocket);
  const rightRelayChainProvider = new WsProvider(rightRelayChain.endpoint.websocket);
  loading.value.leftParaClient = true;
  loading.value.leftRelayClient = true;
  loading.value.rightParaClient = true;
  loading.value.rightRelayClient = true;
  const leftParaClient = await ApiPromise.create({provider: leftParaChainProvider});
  loading.value.leftParaClient = false;
  const leftRelayClient = await ApiPromise.create({provider: leftRelayChainProvider});
  loading.value.leftRelayClient = false;
  const rightParaClient = await ApiPromise.create({provider: rightParaChainProvider});
  loading.value.rightParaClient = false;
  const rightRelayClient = await ApiPromise.create({provider: rightRelayChainProvider});
  loading.value.rightRelayClient = false;

  pickedClient.value = {
    leftParaClient,
    leftRelayClient,
    rightParaClient,
    rightRelayClient,
  };

}

onMounted(() => {
  initState();
});

onBeforeUnmount(() => {
  pickedClient.value.leftParaClient && pickedClient.value.leftParaClient.disconnect();
  pickedClient.value.rightParaClient && pickedClient.value.rightParaClient.disconnect();
  pickedClient.value.leftRelayClient && pickedClient.value.leftRelayClient.disconnect();
  pickedClient.value.rightRelayClient && pickedClient.value.rightRelayClient.disconnect();
});
</script>

