<template>
  <v-container fluid>
    <v-row>
      <v-col cols="12">
        <bridge-skeleton
          v-if="pickedClient.sourceClient && pickedChain.targetChain"
          chain-type="substrate"
          :key="`s2s-parachain-${pickedChain.sourceChain.name}-${pickedChain.targetChain.name}`"
          :source-client="pickedClient.sourceClient"
          :source-chain="pickedChain.sourceChain"
          :target-chain="pickedChain.targetChain"
        >
          <v-progress-linear
            class="mt-15"
            :color="source.chain.left.color"
            indeterminate
            v-if="loading.sourceClient || loading.targetClient"
          />
          <bridge-s2s-basic
            v-else
            :key="`bridge-${pickedChain.sourceChain.name}-${pickedChain.targetChain.name}`"
            :source-client="pickedClient.sourceClient"
            :target-client="pickedClient.targetClient"
            :source-chain="pickedChain.sourceChain"
            :target-chain="pickedChain.targetChain"
          />
        </bridge-skeleton>
        <v-progress-linear v-else class="mt-15" indeterminate/>
      </v-col>
      <v-col cols="12">
        <v-divider/>
      </v-col>
      <v-col cols="12">
        <bridge-skeleton
          v-if="pickedChain.sourceChain && pickedClient.targetClient"
          chain-type="substrate"
          :key="`s2s-parachain-${pickedChain.sourceChain.name}-${pickedChain.targetChain.name}`"
          :source-client="pickedClient.targetClient"
          :source-chain="pickedChain.targetChain"
          :target-chain="pickedChain.sourceChain"
        >
          <v-progress-linear
            class="mt-15"
            :color="pickedChain.targetChain.color"
            indeterminate
            v-if="loading.sourceClient || loading.targetClient"
          />
          <bridge-s2s-basic
            v-else
            :key="`bridge-${pickedChain.targetChain.name}-${pickedChain.sourceChain.name}`"
            :source-client="pickedClient.targetClient"
            :target-client="pickedClient.sourceClient"
            :source-chain="pickedChain.targetChain"
            :target-chain="pickedChain.sourceChain"
          />
        </bridge-skeleton>
        <v-progress-linear v-else class="mt-15" indeterminate/>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts" setup>

import BridgeSkeleton from "@/components/skeleton/bridge-skeleton.vue"

import {defineProps, onMounted, PropType, reactive, toRefs} from 'vue'

import * as dataSource from '@/data/data_source'

import {Bridge} from "@/types/bridge";
import {useRouter} from "vue-router";
import {SubstrateChainInfo} from "@/types/chain";
import {ApiPromise, WsProvider} from "@polkadot/api";
import {
  ParaWithParaChainPair,
  ParaWithParaClientPair,
  SoloWithSoloChainPair,
  SoloWithSoloClientPair
} from "@/types/app";
import BridgeS2sParachainHeader from "@/views/state/s2s/common/bridge-s2s-parachain-header.vue";
import BridgeS2sParachainMessage from "@/views/state/s2s/common/bridge-s2s-parachain-message.vue";
import BridgeS2sBasic from "@/views/state/s2s/common/bridge-s2s-basic.vue";

const router = useRouter();

const props = defineProps({
  bridge: {
    type: Object as PropType<Bridge>,
  },
})

const state = reactive({
  pickedChain: {} as SoloWithSoloChainPair,
  pickedClient: {} as SoloWithSoloClientPair,
  loading: {
    sourceClient: false,
    targetClient: false,
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
  const [sourceChainName, targetChainName] = name.split('-');
  const [sourceChain, targetChain] = [
    dataSource.chainInfo(sourceChainName) as SubstrateChainInfo,
    dataSource.chainInfo(targetChainName) as SubstrateChainInfo,
  ];
  pickedChain.value = {
    sourceChain: {...sourceChain, bridge_chain_name: sourceChainName},
    targetChain: {...targetChain, bridge_chain_name: targetChainName},
  };

  const sourceChainProvider = new WsProvider(sourceChain.endpoint.websocket);
  const targetChainProvider = new WsProvider(targetChain.endpoint.websocket);
  loading.value.sourceClient = true;
  loading.value.targetClient = true;
  const sourceClient = await ApiPromise.create({provider: sourceChainProvider});
  loading.value.sourceClient = false;
  const targetClient = await ApiPromise.create({provider: targetChainProvider});
  loading.value.targetClient = false;
  pickedClient.value = {
    sourceClient,
    targetClient,
  };

}

onMounted(() => {
  initState();
});
</script>

