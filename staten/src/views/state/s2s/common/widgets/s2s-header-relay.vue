<template>
  <v-row>
    <v-col cols="12">
      <h2 class="text-h5 font-weight-thin">Header</h2>
    </v-col>
    <v-col cols="12">
      <v-table>
        <thead>
        <tr>
          <th>Title</th>
          <th>Value</th>
        </tr>
        </thead>
        <tbody>
        <tr>
          <td class="subtitle-2">Last relayed block (hash)</td>
          <td>
            <v-progress-linear v-if="loading.bestFinalizedHash" :color="sourceChain.color" indeterminate/>
            <external-explorer
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
            <v-progress-linear v-if="loading.bestFinalizedHash" :color="sourceChain.color" indeterminate/>
            <external-explorer
              v-else
              :identity="`${source.bestFinalizedBlock}`"
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
                'text-green': source.nextMandatoryHeader <= source.bestFinalizedBlock,
                'text-red': source.nextMandatoryHeader > source.bestFinalizedBlock,
                }"
            >
              <external-explorer
                v-if="source.nextMandatoryHeader"
                :identity="`${source.nextMandatoryHeader}`"
                type="block"
                :chain="sourceChain"
              />
            </div>
          </td>
        </tr>
        <tr v-if="!parachainBridge">
          <td class="subtitle-2">Next on-demand block</td>
          <td>
            <v-progress-linear v-if="loading.nextOnDemandBlock" :color="sourceChain.color" indeterminate/>
            <div
              v-else
              :class="{
                'text-green': source.nextOnDemandBlock <= source.bestFinalizedBlock,
                'text-red': source.nextOnDemandBlock > source.bestFinalizedBlock,
                }"
            >
              <external-explorer
                v-if="source.nextOnDemandBlock"
                :identity="`${source.nextOnDemandBlock}`"
                type="block"
                :chain="sourceChain"
              />
            </div>
          </td>
        </tr>
        <template v-else>
          <slot/>
        </template>
        </tbody>
      </v-table>
    </v-col>
  </v-row>
</template>

<script lang="ts" setup>
import {onMounted, onBeforeUnmount, defineProps, PropType, reactive, toRefs, inject} from 'vue'
import {ApiPromise} from "@polkadot/api";
import {BridgeSubstrateChainInfo} from "@/types/app";
import ExternalExplorer from "@/components/widgets/external-explorer.vue";
import {Subql} from "@/plugins/subql";

const subql = inject('subql') as Subql;

const props = defineProps({
  parachainBridge: {
    type: Boolean,
    default: false,
  },
  sourceChain: {
    type: Object as PropType<BridgeSubstrateChainInfo>,
  },
  sourceClient: {
    type: Object as PropType<ApiPromise>,
  },
  targetChain: {
    type: Object as PropType<BridgeSubstrateChainInfo>,
  },
  targetClient: {
    type: Object as PropType<ApiPromise>,
  },
  grandpaPalletName: {
    type: String,
  },
});

interface _StateSource {
  bestFinalizedBlock?: number;
  bestFinalizedHash?: string;
  nextMandatoryHeader?: number;
  nextOnDemandBlock?: number;
}

interface _StateLoading {
  bestFinalizedHash: boolean;
  nextMandatoryHeader: boolean;
  nextOnDemandBlock: boolean;
}

const state = reactive({
  source: {} as _StateSource,
  loading: {
    bestFinalizedHash: true,
    nextMandatoryHeader: true,
    nextOnDemandBlock: true,
  } as _StateLoading,
  subscriber: {
    bestFinalizedHash: null,
    nextMandatoryBlock: null,
    nextOnDemandBlock: null,
  },
});

const {
  source, loading, subscriber
} = toRefs(state);

function _grandpaPalletName(): string {
  if (props.parachainBridge) {
    return props.grandpaPalletName ?? '';
  }
  // @ts-ignore
  return props.targetChain.bridge_target[props.sourceChain.bridge_chain_name].query_name.grandpa;
}

async function subscribeNextMandatoryBlock() {
  subscriber.value.nextMandatoryBlock = setInterval(async () => {
    try {
      await queryNextMandatoryBlock()
    } catch (e) {
      console.error('Failed query subql: ', e);
    }
  }, 10000);
}

async function subscribeNextOnDemandBlock() {
  subscriber.value.nextOnDemandBlock = setInterval(async () => {
    try {
      await queryNextOnDemandBlock();
    } catch (e) {
      console.error('Failed query subql: ', e);
    }
  }, 10000);
}

async function queryNextMandatoryBlock() {
  if (!source.value.bestFinalizedBlock) {
    return;
  }
  // query next mandatory header
  loading.value.nextMandatoryHeader = true;
  const nextMandatroyBlock = await subql.bridge_s2s(props.sourceChain.subql)
    .nextMandatoryBlock(source.value.bestFinalizedBlock);
  source.value.nextMandatoryHeader = nextMandatroyBlock ? nextMandatroyBlock.blockNumber : null;
  loading.value.nextMandatoryHeader = false;
}

async function queryNextOnDemandBlock() {
  if (!source.value.bestFinalizedBlock) {
    return;
  }
  // query next mandatory header
  loading.value.nextOnDemandBlock = true;
  const nextOnDemandBlock = await subql.bridge_s2s(props.sourceChain.subql)
    .nextOnDemandBlock(`bridge-${props.targetChain.bridge_chain_name}`);
  source.value.nextOnDemandBlock = nextOnDemandBlock ? nextOnDemandBlock.blockNumber : null;
  loading.value.nextOnDemandBlock = false;
}

async function initState() {
  const {targetClient} = props;
  const grandpaPalletName = _grandpaPalletName();
  // @ts-ignore
  subscriber.value.bestFinalizedHash = await targetClient?.query[grandpaPalletName]
    .bestFinalized(async (v: any) => {
      console.log('subscribed best finalized hash');
      loading.value.bestFinalizedHash = false;
      const [blockNumber, blockHash] = v.toHuman() as unknown as [string, string];
      source.value.bestFinalizedBlock = +(blockNumber.replaceAll(',', ''));
      source.value.bestFinalizedHash = blockHash;
      await queryNextMandatoryBlock();
      if (!props.parachainBridge) {
        await queryNextOnDemandBlock();
      }
    });

}

onMounted(() => {
  initState();
  subscribeNextMandatoryBlock();
  subscribeNextOnDemandBlock();
});

onBeforeUnmount(() => {
  // @ts-ignore
  subscriber.value.bestFinalizedHash && subscriber.value.bestFinalizedHash();
  subscriber.value.nextMandatoryBlock && clearInterval(subscriber.value.nextMandatoryBlock);
  subscriber.value.nextOnDemandBlock && clearInterval(subscriber.value.nextOnDemandBlock);
});
</script>
