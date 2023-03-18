<template>
  <v-container fluid>
    <v-row>
      <v-col cols="12">
        <bridge-skeleton
          v-if="pickedClient.soloClient && pickedChain.paraChain"
          chain-type="substrate"
          :key="`s2s-parachain-${pickedChain.soloChain.name}-${pickedChain.paraChain.name}`"
          :source-client="pickedClient.soloClient"
          :source-chain="pickedChain.soloChain"
          :target-chain="pickedChain.paraChain"
        >
          <v-progress-linear
            class="mt-15"
            :color="pickedChain.soloChain"
            indeterminate
            v-if="loading.soloClient || loading.paraClient"
          />
          <bridge-s2s-basic
            v-else
            :key="`bridge-${pickedChain.soloChain.name}-${pickedChain.paraChain.name}`"
            :source-client="pickedClient.soloClient"
            :target-client="pickedClient.paraClient"
            :source-chain="pickedChain.soloChain"
            :target-chain="pickedChain.paraChain"
          />
        </bridge-skeleton>
        <v-progress-linear v-else class="mt-15" indeterminate/>
      </v-col>
      <v-col cols="12">
        <v-divider/>
      </v-col>
      <v-col cols="12">
        <bridge-skeleton
          chain-type="substrate"
          :key="`s2s-parachain-${pickedChain.relayChain.name}-${pickedChain.soloChain.name}`"
          :source-client="pickedClient.relayClient"
          :source-chain="pickedChain.relayChain"
          :target-chain="pickedChain.soloChain"
          v-if="pickedClient.relayClient && pickedChain.soloChain"
        >
          <v-progress-linear
            class="mt-15"
            :color="pickedChain.relayChain.color"
            indeterminate
            v-if="loading.relayClient || loading.soloClient"
          />
          <bridge-s2s-parachain-header
            v-else
            :solo-client="pickedClient.soloClient"
            :para-client="pickedClient.paraClient"
            :relay-client="pickedClient.relayClient"
            :solo-chain="pickedChain.soloChain"
            :para-chain="pickedChain.paraChain"
            :relay-chain="pickedChain.relayChain"
          />
        </bridge-skeleton>
        <v-progress-linear v-else class="mt-15" indeterminate/>
      </v-col>
      <v-col cols="12">
        <bridge-skeleton
          ref="parachainchain_to_solochain"
          chain-type="substrate"
          :key="`s2s-parachain-${pickedChain.paraChain.name}-${pickedChain.soloChain.name}`"
          :source-client="pickedClient.paraClient"
          :source-chain="pickedChain.paraChain"
          :target-chain="pickedChain.soloChain"
          v-if="pickedClient.paraClient && pickedChain.soloChain"
        >
          <v-progress-linear
            class="mt-15"
            :color="pickedChain.paraChain.color"
            indeterminate
            v-if="loading.paraClient || loading.soloClient"
          />
          <bridge-s2s-parachain-message
            v-else
            :solo-client="pickedClient.soloClient"
            :para-client="pickedClient.paraClient"
            :relay-client="pickedClient.relayClient"
            :solo-chain="pickedChain.soloChain"
            :para-chain="pickedChain.paraChain"
            :relay-chain="pickedChain.relayChain"
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
import {SoloWithParaChainPair, SoloWithParaClientPair} from "@/types/app";
import BridgeS2sParachainHeader from "@/views/state/s2s/common/bridge-s2s-parachain-header.vue";
import BridgeS2sBasic from "@/views/state/s2s/common/bridge-s2s-basic.vue";
import BridgeS2sParachainMessage from "@/views/state/s2s/common/bridge-s2s-parachain-message.vue";

const router = useRouter();

const props = defineProps({
  bridge: {
    type: Object as PropType<Bridge>,
  },
})

const state = reactive({
  pickedChain: {} as SoloWithParaChainPair,
  pickedClient: {} as SoloWithParaClientPair,
  loading: {
    paraClient: false,
    relayClient: false,
    soloClient: false,
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
  const [soloChainName, paraChainName] = name.split('-');
  const [soloChain, paraChain] = [
    dataSource.chainInfo(soloChainName) as SubstrateChainInfo,
    dataSource.chainInfo(paraChainName) as SubstrateChainInfo,
  ];
  // @ts-ignore
  const relaychainName = soloChain.bridge_target[parachainName].relay_chain;
  const relayChain = dataSource.chainInfo(relaychainName) as SubstrateChainInfo;
  if (!soloChain || !paraChain || !relayChain) {
    await router.push({path: '/'})
    return;
  }
  pickedChain.value = {
    soloChain: {...soloChain, bridge_chain_name: soloChainName},
    paraChain: {...paraChain, bridge_chain_name: paraChainName},
    relayChain: {...relayChain, bridge_chain_name: relaychainName},
  };

  loading.value.soloClient = true;
  loading.value.paraClient = true;
  loading.value.relayClient = true;

  const soloChainProvider = new WsProvider(soloChain.endpoint.websocket);
  const paraChainProvider = new WsProvider(paraChain.endpoint.websocket);
  const relayChainProvider = new WsProvider(relayChain.endpoint.websocket);

  const soloClient = await ApiPromise.create({provider: soloChainProvider});
  loading.value.soloClient = false;
  const paraClient = await ApiPromise.create({provider: paraChainProvider});
  loading.value.paraClient = false;
  const relayClient = await ApiPromise.create({provider: relayChainProvider});
  loading.value.relayClient = false;

  pickedClient.value = {
    soloClient,
    paraClient,
    relayClient,
  };

}

onMounted(() => {
  initState();
});
</script>

