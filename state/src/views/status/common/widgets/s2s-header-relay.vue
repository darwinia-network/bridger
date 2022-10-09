<template>
  <v-row>
    <v-col cols="12">
      <h2 class="text-h5 font-weight-thin">Header</h2>
    </v-col>
    <v-col cols="12">
      <v-simple-table dense>
        <template v-slot:default>
          <thead>
          <tr>
            <th style="width: 45%">Title</th>
            <th>Value</th>
          </tr>
          </thead>
          <tbody>
          <tr>
            <td class="subtitle-2">Last relayed block (hash)</td>
            <td>
              <v-progress-linear v-if="loading.bestFinalizedHash" :color="sourceChain.color" indeterminate/>
              <external-subscan
                v-else
                :identity="source.bestFinalizedHash"
                type="block"
                :chain="sourceChain"
              />
            </td>
          </tr>
          <tr>
            <td class="subtitle-2">Last relayed block (number)</td>
            <td>
              <v-progress-linear v-if="loading.bestFinalizedBlock" :color="sourceChain.color" indeterminate/>
              <external-subscan
                v-else
                :identity="`${source.bestFinalizedBlock.block.header.number}`"
                type="block"
                :chain="sourceChain"
              />
            </td>
          </tr>
          <tr>
            <td class="subtitle-2">Next mandatory block</td>
            <td>
              <v-progress-linear v-if="loading.nextMandatoryHeader" :color="sourceChain.color" indeterminate/>
              <div
                v-else
                :class="{
                'green--text': source.nextMandatoryHeader <= source.bestFinalizedBlock.block.header.number,
                'red--text': source.nextMandatoryHeader > source.bestFinalizedBlock.block.header.number,
                }"
              >
                <external-subscan
                  v-if="source.nextMandatoryHeader"
                  :identity="`${source.nextMandatoryHeader}`"
                  type="block"
                  :chain="sourceChain"
                />
              </div>
            </td>
          </tr>
          <tr>
            <td class="subtitle-2">Next on-demand block</td>
            <td>
              <v-progress-linear v-if="loading.nextOnDemandBlock" :color="sourceChain.color" indeterminate/>
              <div
                v-else
                :class="{
                'green--text': source.nextOnDemandBlock <= source.bestFinalizedBlock.block.header.number,
                'red--text': source.nextOnDemandBlock > source.bestFinalizedBlock.block.header.number,
                }"
              >
                <external-subscan
                  v-if="source.nextOnDemandBlock"
                  :identity="`${source.nextOnDemandBlock}`"
                  type="block"
                  :chain="sourceChain"
                />
              </div>
            </td>
          </tr>
          </tbody>
        </template>
      </v-simple-table>
    </v-col>
  </v-row>
</template>

<script>

import ExternalSubscan from '@/components/widgets/external-subscan';

async function initState(vm) {
  // subscribe best finalized
  vm.subscriber.bestFinalized = await vm.targetClient.query[
    vm.targetChain.bridge_target[vm.sourceChain.bridge_chain_name].query_name.grandpa
    ].bestFinalized(async v => {
    vm.loading.bestFinalizedHash = false;
    vm.loading.bestFinalizedBlock = true;

    // query block from best finalized
    const blockHash = v.toHuman();
    vm.source.bestFinalizedHash = blockHash;
    const block = await vm.sourceClient.rpc.chain.getBlock(blockHash);
    vm.source.bestFinalizedBlock = block.toJSON();
    vm.loading.bestFinalizedBlock = false;

    // query next on-demand header
    await queryNextMandatoryBlock(vm);
    await queryNextOnDemandBlock(vm);
  });
}

async function subscribeNextMandatoryBlock(vm) {
  vm.subscriber.nextMandatoryBlock = setInterval(async () => {
    try {
      await queryNextMandatoryBlock(vm)
    } catch (e) {
      vm.$toast.error(`Failed query subql, more details please view browser console.`);
      vm.$log.error('Failed query subql: ', e);
    }
  }, 10000);
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

async function queryNextMandatoryBlock(vm) {
  if (!vm.source.bestFinalizedBlock) {
    return;
  }
  // query next mandatory header
  vm.loading.nextMandatoryHeader = true;
  const nextMandatroyBlock = await vm.$subql.bridge_s2s(vm.sourceChain.subql)
    .nextMandatoryBlock(vm.source.bestFinalizedBlock.block.header.number);
  vm.source.nextMandatoryHeader = nextMandatroyBlock ? nextMandatroyBlock.blockNumber : null;
  vm.loading.nextMandatoryHeader = false;
}

async function queryNextOnDemandBlock(vm) {
  if (!vm.source.bestFinalizedBlock) {
    return;
  }
  // query next mandatory header
  vm.loading.nextOnDemandBlock = true;
  const nextOnDemandBlock = await vm.$subql.bridge_s2s(vm.sourceChain.subql)
    .nextOnDemandBlock(`bridge-${vm.targetChain.bridge_chain_name}`);
  vm.source.nextOnDemandBlock = nextOnDemandBlock ? nextOnDemandBlock.blockNumber : null;
  vm.loading.nextOnDemandBlock = false;
}

export default {
  components: {ExternalSubscan},
  props: {
    sourceClient: {
      type: Object,
    },
    targetClient: {
      type: Object,
    },
    sourceChain: {
      type: Object,
    },
    targetChain: {
      type: Object,
    },
  },
  data: () => ({
    source: {
      bestFinalizedHash: null,
      bestFinalizedBlock: null,
      nextMandatoryHeader: null,
      nextOnDemandBlock: null,
    },
    subscriber: {
      bestFinalized: null,
      nextMandatoryBlock: null,
      nextOnDemandBlock: null,
    },
    loading: {
      bestFinalizedHash: true,
      bestFinalizedBlock: true,
      nextMandatoryHeader: true,
      nextOnDemandBlock: true,
    },
  }),
  async created() {
    await initState(this);
    await subscribeNextMandatoryBlock(this);
    await subscribeNextOnDemandBlock(this);
  },
  destroyed() {
    const vm = this;
    vm.subscriber.bestFinalized && vm.subscriber.bestFinalized();
    vm.subscriber.nextMandatoryBlock && clearInterval(vm.subscriber.nextMandatoryBlock);
    vm.subscriber.nextOnDemandBlock && clearInterval(vm.subscriber.nextOnDemandBlock);
  }
}
</script>

<style scoped>

</style>
