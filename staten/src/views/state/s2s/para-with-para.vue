<template>
  <v-container fluid>
    <v-row>
      <v-col cols="12">
        <bridge-skeleton
          v-if="pickedChain.leftParaChain && pickedChain.rightParaChain"
        >
          <v-progress-linear
            class="mt-15"
            :color="pickedChain.leftParaChain.color"
            indeterminate
            v-if="loading.leftParaClient || loading.rightParaClient"
          />
          <bridge-s2s-basic
            v-else
            :source-chain="pickedChain.leftParaChain"
            :source-client="pickedClient.leftParaClient"
            :target-chain="pickedChain.rightParaChain"
            :target-client="pickedClient.rightParaClient"
          />
        </bridge-skeleton>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts" setup>

import BridgeSkeleton from "@/components/skeleton/bridge-skeleton.vue"
import BridgeS2sBasic from "@/views/state/s2s/common/bridge-s2s-basic.vue";

import {defineProps, onMounted, PropType, reactive, toRefs} from 'vue'

import * as dataSource from '@/data/data_source'

import {Bridge} from "@/types/bridge";
import {useRouter} from "vue-router";
import {SubstrateChainInfo} from "@/types/chain";
import {ApiPromise, WsProvider} from "@polkadot/api";
import {ParaWithParaChainPair, ParaWithParaClientPair} from "@/types/app";

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
    leftParaChain,
    leftRelayChain,
    rightParaChain,
    rightRelayChain,
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
</script>

