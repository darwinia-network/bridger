<template>
  <v-container fluid>
    <v-row>
      <v-col cols="12">
        <bridge-skeleton
          ref="solochain_to_parachain"
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
          <span v-else>solochain -> parachain</span>
        </bridge-skeleton>
      </v-col>
      <v-col cols="12">
        <bridge-skeleton
          ref="relaychain_to_solochain_header"
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
          <span v-else>relaychain -> solochain header</span>
        </bridge-skeleton>
      </v-col>
      <v-col cols="12">
        <bridge-skeleton
          ref="relaychain_to_solochain_message"
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
          <span v-else>relaychain -> solochain mesage</span>
        </bridge-skeleton>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>

import BridgeSkeleton from '@/views/status/common/bridge-skeleton'
import * as dataSource from "@/data/data_source";
import {ApiPromise, WsProvider} from "@polkadot/api";

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
  // vm.$refs['left_to_right'].initState(vm.source.client.left);

  vm.source.client.parachain = await ApiPromise.create({provider: parachainProvider});
  vm.loading.parachainClient = false;
  // vm.$refs['right_to_left'].initState(vm.source.client.right);

  vm.source.client.relaychain = await ApiPromise.create({provider: relaychainProvider});
  vm.loading.relaychainClient = false;
}

export default {
  props: {
    bridge: {
      type: Object,
    },
  },
  components: {
    BridgeSkeleton,
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
