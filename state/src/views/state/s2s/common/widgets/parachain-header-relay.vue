<template>
  <div>
    <s2s-header-relay
      :key="`s2s-header-${relayChain.name}-${soloChain.name}`"
      :parachain-bridge="true"
      :grandpa-pallet-name="source.grandpaPalletName"
      :source-client="relayClient"
      :target-client="soloClient"
      :source-chain="relayChain"
      :target-chain="soloChain"
    >
      <template v-slot:default>
        <tr :key="`on-demand-${paraChain.name}-${soloChain.name}`">
          <td class="subtitle-2">Next on-demand block</td>
          <td>
            <v-progress-linear v-if="loading.nextOnDemandBlock || loading.bestFinalizedBlock" :color="relayChain.color" indeterminate/>
            <template v-else>
              <div
                :class="{
                'green--text': source.nextOnDemandBlock <= source.bestFinalizedBlock.block.header.number,
                'red--text': source.nextOnDemandBlock > source.bestFinalizedBlock.block.header.number,
                }"
              >
                <external-explorer
                  v-if="source.nextOnDemandBlock"
                  :identity="`${source.nextOnDemandBlock}`"
                  type="block"
                  :chain="relayChain"
                />
              </div>
            </template>
          </td>
        </tr>
      </template>
    </s2s-header-relay>
  </div>
</template>

<script>

import S2sHeaderRelay from '@/views/state/s2s/common/widgets/s2s-header-relay';
import ExternalExplorer from '@/components/widgets/external-explorer';


async function initState(vm) {
  vm.source.grandpaPalletName = vm.soloChain.bridge_target[vm.paraChain.bridge_chain_name].query_name.grandpa;
  vm.subscriber.bestFinalized = await vm.soloClient.query[vm.source.grandpaPalletName]
    .bestFinalized(async v => {
      vm.loading.bestFinalizedHash = false;
      vm.loading.bestFinalizedBlock = true;

      // query block from best finalized
      const blockHash = v.toHuman();
      vm.source.bestFinalizedHash = blockHash;
      const block = await vm.relayClient.rpc.chain.getBlock(blockHash);
      vm.source.bestFinalizedBlock = block.toJSON();
      vm.loading.bestFinalizedBlock = false;
      await queryNextOnDemandBlock(vm);
    });
}

async function subscribeNextOnDemandBlock(vm) {
  vm.subscriber.nextOnDemandBlock = setInterval(async () => {
    try {
      await queryNextOnDemandBlock(vm);
    } catch (e) {
      vm.$toast.error(`Failed query subql, more details please view browser console.`);
      vm.$log.error('Failed query subql: ', e);
    }
  }, 10000);
}

async function queryNextOnDemandBlock(vm) {
  vm.loading.nextOnDemandBlock = true;
  const nextOnDemandBlock = await vm.$subql.bridge_s2s(vm.paraChain.subql)
    .nextOnDemandBlock(`bridge-${vm.soloChain.bridge_chain_name}`);
  if (!nextOnDemandBlock || !nextOnDemandBlock.blockHash) {
    vm.loading.nextOnDemandBlock = false;
    return;
  }
  const nextCandidate = await vm.$subql.bridge_s2s(vm.relayChain.subql)
    .queryNextCandidateIncludedEvent(nextOnDemandBlock.blockHash);
  vm.source.nextOnDemandBlock = nextCandidate.includedRelayBlock;
  vm.loading.nextOnDemandBlock = false;
  // vm.$set(vm.source, 'nextOnDemandBlock', false);
}

export default {
  components: {ExternalExplorer, S2sHeaderRelay},
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
      grandpaPalletName: null,

      bestFinalizedHash: null,
      bestFinalizedBlock: null,
      nextOnDemandBlock: null,
    },
    subscriber: {
      nextOnDemandBlock: null,
      bestFinalized: null,
    },
    loading: {
      bestFinalizedHash: true,
      bestFinalizedBlock: true,
      nextOnDemandBlock: false,
    }
  }),
  async created() {
    const vm = this;
    await initState(vm);
    await subscribeNextOnDemandBlock(vm);
  },
  destroyed() {
    const vm = this;
    vm.subscriber.bestFinalized && vm.subscriber.bestFinalized();
    vm.subscriber.nextOnDemandBlock && clearInterval(vm.subscriber.nextOnDemandBlock);
  }
}
</script>

<style scoped>

</style>
