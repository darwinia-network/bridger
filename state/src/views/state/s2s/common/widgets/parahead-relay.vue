<template>
  <v-row>
    <v-col cols="12">
      <h2 class="text-h5 font-weight-thin">Para head</h2>
    </v-col>
    <v-col cols="12">
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
            <td class="subtitle-2">Relayed parahead hash (parachain)</td>
            <td>
              <v-progress-linear v-if="loading.paraHeadAtTarget" :color="relayChain.color" indeterminate/>
              <external-explorer
                v-else
                :identity="`${source.paraHeadAtTarget.headHash}`"
                type="block"
                :chain="paraChain"
              />
            </td>
          </tr>
          <tr>
            <td class="subtitle-2">Relayed parahead in relay chain (relaychain)</td>
            <td>
              <v-progress-linear v-if="loading.paraHeadAtTarget || loading.relayedGrandpaBlockHash" :color="relayChain.color" indeterminate/>
              <div
                v-else
                :class="{
                'red--text': source.paraHeadAtTarget.atRelayBlockNumber < source.relayedGrandpaBlock.block.header.number,
                'green--text': source.paraHeadAtTarget.atRelayBlockNumber >= source.relayedGrandpaBlock.block.header.number,
                }"
              >
                <external-explorer
                  :identity="`${source.paraHeadAtTarget.atRelayBlockNumber}`"
                  type="block"
                  :chain="relayChain"
                />
              </div>
            </td>
          </tr>
          <tr>
            <td class="subtitle-2">Last relayed block (relaychain)</td>
            <td>
              <v-progress-linear v-if="loading.relayedGrandpaBlockHash" :color="relayChain.color" indeterminate/>
              <external-explorer
                v-else
                :identity="`${source.relayedGrandpaBlock.block.header.number}`"
                type="block"
                :chain="relayChain"
              />
            </td>
          </tr>
          <template v-if="!loading.paraHeadAtTarget && !loading.relayedGrandpaBlockHash">
            <tr v-if="source.paraHeadAtTarget.atRelayBlockNumber < source.relayedGrandpaBlock.block.header.number">
              <td class="subtitle-2">Last parachain by relayed at source (relaychain)</td>
              <td>
                <v-progress-linear v-if="loading.paraHeadAtSourceByLastRelayedGrandpa" :color="relayChain.color" indeterminate/>
                <external-explorer
                  v-else
                  :identity="`${source.paraHeadAtSourceByLastRelayedGrandpa}`"
                  type="block"
                  :chain="relayChain"
                />
              </td>
            </tr>
          </template>

          </tbody>
        </template>
      </v-simple-table>
    </v-col>
  </v-row>
</template>

<script>

import ExternalExplorer from '@/components/widgets/external-explorer';

async function initState(vm) {
  // vm.soloClient.query[]
  const bridgeTarget = vm.soloChain.bridge_target[vm.paraChain.bridge_chain_name];
  vm.subscriber.paraHeadAtTarget = await vm.soloClient.query[bridgeTarget.query_name.parachains]
    .bestParaHeads(bridgeTarget.para_id, async v => {
      vm.source.paraHeadAtTarget = v.toJSON();
      vm.loading.paraHeadAtTarget = false;
    });
  vm.subscriber.relayedGrandpaBlockHash = await vm.soloClient.query[bridgeTarget.query_name.grandpa]
    .bestFinalized(async v => {
      vm.source.relayedGrandpaBlockHash = v.toJSON();
      const relayedGrandpaBlock  = await vm.relayClient.rpc.chain.getBlock(vm.source.relayedGrandpaBlockHash);
      vm.source.relayedGrandpaBlock = relayedGrandpaBlock.toJSON();
      vm.loading.relayedGrandpaBlockHash = false;
      vm.loading.paraHeadAtSourceByLastRelayedGrandpa = true;

      const atBlockRelayClient = await vm.relayClient.at(vm.source.relayedGrandpaBlockHash);
      const paraHeadAtSourceByLastRelayedGrandpa = await atBlockRelayClient.query.paras.heads(bridgeTarget.para_id);
      vm.source.paraHeadAtSourceByLastRelayedGrandpa = paraHeadAtSourceByLastRelayedGrandpa.toJSON();
      vm.loading.paraHeadAtSourceByLastRelayedGrandpa = false;
    });
  // vm.subscriber.paraHeadAtSourceByLastBlock = await vm.relayClient.query
  //   .paras.heads(bridgeTarget.para_id, async v => {
  //     vm.source.paraHeadAtSourceByLastBlock = v.toJSON();
  //     vm.loading.paraHeadAtSourceByLastBlock = false;
  //   });
}

export default {
  components: {ExternalExplorer},
  props: {
    soloClient: {
      type: Object,
    },
    paraClient: {
      type: Object,
    },
    relayClient: {
      type: Object,
    },
    soloChain: {
      type: Object,
    },
    paraChain: {
      type: Object,
    },
    relayChain: {
      type: Object,
    },
  },
  data: () => ({
    source: {
      paraHeadAtTarget: null,
      relayedGrandpaBlockHash: null,
      paraHeadAtSourceByLastRelayedGrandpa: null,
      // paraHeadAtSourceByLastBlock: null,
      relayedGrandpaBlock: null,
    },
    subscriber: {
      paraHeadAtTarget: null,
      relayedGrandpaBlockHash: null,
      paraHeadAtSourceByLastRelayedGrandpa: null,
      // paraHeadAtSourceByLastBlock: null,
    },
    loading: {
      paraHeadAtTarget: true,
      relayedGrandpaBlockHash: true,
      paraHeadAtSourceByLastRelayedGrandpa: true,
      // paraHeadAtSourceByLastBlock: true,
    },
  }),
  created() {
    initState(this);
  },
  destroyed() {
    const vm = this;
    vm.subscriber.bestParaHeads && vm.subscriber.bestParaHeads();
    vm.subscriber.relayedGrandpaBlockHash && vm.subscriber.relayedGrandpaBlockHash();
    // vm.subscriber.paraHeadAtSourceByLastBlock && vm.subscriber.paraHeadAtSourceByLastBlock();
  }
}
</script>

<style scoped>

</style>
