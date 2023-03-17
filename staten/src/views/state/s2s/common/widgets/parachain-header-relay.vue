<template>
  <div>
    <v-progress-linear
      class="mt-15"
      :color="paraChain.color"
      indeterminate
      v-if="!source.grandpaPalletName || !relayClient || !targetClient"
    />
    <s2s-header-relay
      v-else
      :key="`s2s-header-${relayChain.name}-${targetChain.name}`"
      :parachain-bridge="true"
      :grandpa-pallet-name="source.grandpaPalletName"
      :source-client="relayClient"
      :source-chain="relayChain"
      :target-chain="targetChain"
      :target-client="targetClient"
    >
      <template v-slot:default>
        <tr :key="`on-demand-${paraChain.name}-${targetChain.name}`">
          <td class="subtitle-2">Next on-demand block</td>
          <td>
            <v-progress-linear
              v-if="loading.nextOnDemandBlock || loading.bestFinalizedHash"
              :color="relayChain.color"
              indeterminate
            />
            <template v-else>
              <div
                :class="{
                'text-green': source.nextOnDemandBlock <= source.bestFinalizedBlock,
                'text-red': source.nextOnDemandBlock > source.bestFinalizedBlock,
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


<script lang="ts" setup>

import {defineProps, inject, onBeforeUnmount, onMounted, PropType, reactive, toRefs} from 'vue'
import {BridgeSubstrateChainInfo} from "@/types/app";
import {ApiPromise} from "@polkadot/api";
import S2sHeaderRelay from "@/views/state/s2s/common/widgets/s2s-header-relay.vue";
import {Subql} from "@/plugins/subql";
import ExternalExplorer from "@/components/widgets/external-explorer.vue";


const subql = inject('subql') as Subql;

const props = defineProps({
  relayChain: {
    type: Object as PropType<BridgeSubstrateChainInfo>,
  },
  relayClient: {
    type: Object as PropType<ApiPromise>,
  },
  paraChain: {
    type: Object as PropType<BridgeSubstrateChainInfo>,
  },
  paraClient: {
    type: Object as PropType<ApiPromise>,
  },
  targetChain: {
    type: Object as PropType<BridgeSubstrateChainInfo>,
  },
  targetClient: {
    type: Object as PropType<ApiPromise>,
  },
});


interface _StateSource {
  grandpaPalletName?: string;
  bestFinalizedBlock?: number;
  bestFinalizedHash?: string;
  nextOnDemandBlock?: number;
}

interface _StateLoading {
  bestFinalizedHash: boolean;
  nextOnDemandBlock: boolean;
}

const state = reactive({
  source: {} as _StateSource,
  loading: {
    bestFinalizedHash: true,
    nextOnDemandBlock: true,
  } as _StateLoading,
  subscriber: {
    bestFinalizedHash: null,
    nextOnDemandBlock: null,
  },
});

const {
  source, loading, subscriber
} = toRefs(state);

async function subscribeNextOnDemandBlock() {
  subscriber.value.nextOnDemandBlock = setInterval(async () => {
    try {
      await queryNextOnDemandBlock();
    } catch (e) {
      console.error('Failed query subql: ', e);
    }
  }, 10000);
}

async function queryNextOnDemandBlock() {
  loading.value.nextOnDemandBlock = true;
  const nextOnDemandBlock = await subql.bridge_s2s(props.paraChain.subql)
    .nextOnDemandBlock(`bridge-${props.targetChain.bridge_chain_name}`);
  if (!nextOnDemandBlock || !nextOnDemandBlock.blockHash) {
    loading.value.nextOnDemandBlock = false;
    return;
  }
  const nextCandidate = await subql.bridge_s2s(props.relayChain.subql)
    .queryNextCandidateIncludedEvent(nextOnDemandBlock.blockHash);
  source.value.nextOnDemandBlock = nextCandidate.includedRelayBlock;
  loading.value.nextOnDemandBlock = false;
}

async function initState() {
  source.value.grandpaPalletName = props.targetChain.bridge_target[props.paraChain.bridge_chain_name].query_name.grandpa;
  console.log(props.targetClient);
  // @ts-ignore
  subscriber.value.bestFinalizedHash = await props.targetClient?.query[source.value.grandpaPalletName]
    .bestFinalized(async (v: any) => {
      loading.value.bestFinalizedHash = false;
      const [blockNumber, blockHash] = v.toHuman() as unknown as [string, string];
      source.value.bestFinalizedBlock = +(blockNumber.replaceAll(',', ''));
      source.value.bestFinalizedHash = blockHash;

      await queryNextOnDemandBlock();
    });
}

onMounted(() => {
  initState();
  subscribeNextOnDemandBlock();
});

onBeforeUnmount(() => {
  // @ts-ignore
  subscriber.value.bestFinalizedHash && subscriber.value.bestFinalizedHash();
  subscriber.value.nextOnDemandBlock && clearInterval(subscriber.value.nextOnDemandBlock);
});
</script>
