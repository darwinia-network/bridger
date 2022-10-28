<template>
  <v-container fluid>
    <v-row>
      <v-col cols="12">
        <bridge-skeleton
          ref="solochain_to_parachain"
          chain-type="substrate"
          :key="`s2s-parachain-${source.chain.solochain.name}-${source.chain.parachain.name}`"
          :source-client="source.client.solochain"
          :source-chain="source.chain.solochain"
          :target-chain="source.chain.parachain"
          v-if="source.chain.solochain && source.chain.parachain"
        >
          <v-progress-linear
            class="mt-15"
            :color="source.chain.solochain.color"
            indeterminate
            v-if="loading.solochainClient || loading.parachainClient"
          />
          <bridge-basic-s2s
            v-else
            :key="`bridge-${source.chain.solochain.name}-${source.chain.parachain.name}`"
            :parachain-bridge="true"
            :source-client="source.client.solochain"
            :target-client="source.client.parachain"
            :source-chain="source.chain.solochain"
            :target-chain="source.chain.parachain"
          />
        </bridge-skeleton>
      </v-col>
      <v-col cols="12">
        <v-divider/>
      </v-col>
      <v-col cols="12">
        <bridge-skeleton
          ref="relaychain_to_solochain"
          chain-type="substrate"
          :key="`s2s-parachain-${source.chain.relaychain.name}-${source.chain.solochain.name}`"
          :source-client="source.client.relaychain"
          :source-chain="source.chain.relaychain"
          :target-chain="source.chain.solochain"
          v-if="source.chain.relaychain && source.chain.solochain"
        >
          <v-progress-linear
            class="mt-15"
            :color="source.chain.relaychain.color"
            indeterminate
            v-if="loading.relaychainClient || loading.solochainClient"
          />
          <bridge-s2s-parachain-header
            v-else
            :solo-client="source.client.solochain"
            :para-client="source.client.parachain"
            :relay-client="source.client.relaychain"
            :solo-chain="source.chain.solochain"
            :para-chain="source.chain.parachain"
            :relay-chain="source.chain.relaychain"
          />
        </bridge-skeleton>
      </v-col>
      <v-col cols="12">
        <bridge-skeleton
          ref="parachainchain_to_solochain"
          chain-type="substrate"
          :key="`s2s-parachain-${source.chain.parachain.name}-${source.chain.solochain.name}`"
          :source-client="source.client.parachain"
          :source-chain="source.chain.parachain"
          :target-chain="source.chain.solochain"
          v-if="source.chain.parachain && source.chain.solochain"
        >
          <v-progress-linear
            class="mt-15"
            :color="source.chain.parachain.color"
            indeterminate
            v-if="loading.parachainClient || loading.solochainClient"
          />
          <bridge-s2s-parachain-message
            v-else
            :solo-client="source.client.solochain"
            :para-client="source.client.parachain"
            :relay-client="source.client.relaychain"
            :solo-chain="source.chain.solochain"
            :para-chain="source.chain.parachain"
            :relay-chain="source.chain.relaychain"
          />
        </bridge-skeleton>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>

import BridgeSkeleton from '@/components/skeleton/bridge-skeleton'
import BridgeBasicS2s from '@/views/state/s2s/common/bridge-basic-s2s'
import BridgeS2sParachainHeader from '@/views/state/s2s/common/bridge-s2s-parachain-header'

import * as dataSource from "@/data/data_source";
import {ApiPromise, WsProvider} from "@polkadot/api";
import BridgeS2sParachainMessage from "@/views/state/s2s/common/bridge-s2s-parachain-message";

async function initState(vm) {
  const name = vm.bridge.name;
  const [solochainName, parachainName] = name.split('-');
  const [solochain, parachain] = [
    dataSource.chainInfo(solochainName),
    dataSource.chainInfo(parachainName),
  ];
  const relaychainName = solochain.bridge_target[parachainName].relay_chain;
  const relaychain = dataSource.chainInfo(relaychainName);
  if (!solochain || !parachain || !relaychain) {
    await vm.$router.push({path: '/'})
    return;
  }
  vm.source.chain.solochain = {...solochain, bridge_chain_name: solochainName};
  vm.source.chain.parachain = {...parachain, bridge_chain_name: parachainName};
  vm.source.chain.relaychain = {...relaychain, bridge_chain_name: relaychainName};

  vm.loading.solochainClient = true;
  vm.loading.parachainClient = true;
  vm.loading.relaychainClient = true;

  const solochainProvider = new WsProvider(vm.source.chain.solochain.endpoint.websocket);
  const parachainProvider = new WsProvider(vm.source.chain.parachain.endpoint.websocket);
  const relaychainProvider = new WsProvider(vm.source.chain.relaychain.endpoint.websocket);
  vm.source.client.solochain = await ApiPromise.create({provider: solochainProvider});
  vm.loading.solochainClient = false;
  vm.$refs['solochain_to_parachain'].initState(vm.source.client.solochain);

  vm.source.client.parachain = await ApiPromise.create({provider: parachainProvider});
  vm.loading.parachainClient = false;
  vm.$refs['parachainchain_to_solochain'].initState(vm.source.client.parachain);

  vm.source.client.relaychain = await ApiPromise.create({provider: relaychainProvider});
  vm.loading.relaychainClient = false;
  vm.$refs['relaychain_to_solochain'].initState(vm.source.client.relaychain);
}

export default {
  props: {
    bridge: {
      type: Object,
    },
  },
  components: {
    BridgeS2sParachainMessage,
    BridgeS2sParachainHeader,
    BridgeSkeleton,
    BridgeBasicS2s,
  },
  data: () => ({
    source: {
      client: {
        solochain: null,
        parachain: null,
        relaychain: null,
      },
      chain: {
        solochain: null,
        parachain: null,
        relaychain: null,
      }
    },
    loading: {
      solochainClient: false,
      parachainClient: false,
      relaychainClient: false,
    },
  }),
  created() {
    initState(this);
  },
  destroyed() {
    const vm = this;
    vm.source.client.solochain && (vm.source.client.solochain.disconnect())
    vm.source.client.parachain && (vm.source.client.parachain.disconnect())
    vm.source.client.relaychain && (vm.source.client.relaychain.disconnect())
  }
}
</script>

<style scoped>

</style>
