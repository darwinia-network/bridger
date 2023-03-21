<template>
  <v-row>
    <v-col cols="12">
      <h2 class="text-h5 font-weight-thin">Para head</h2>
    </v-col>
    <v-col cols="12">
      <v-table density="compact">
        <thead>
        <tr>
          <th>Title</th>
          <th>Value</th>
        </tr>
        </thead>
        <tbody>
        <tr>
          <td>Relayed parahead hash (parachain)</td>
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
          <td>Relayed parahead in relay chain (relaychain)</td>
          <td>
            <v-progress-linear v-if="loading.paraHeadAtTarget || loading.relayedGrandpaBlockHash" :color="relayChain.color" indeterminate/>
            <div
              v-else
              :class="{
                'red--text': source.paraHeadAtTarget.atRelayBlockNumber < source.relayedGrandpaBlock,
                'green--text': source.paraHeadAtTarget.atRelayBlockNumber >= source.relayedGrandpaBlock,
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
          <td>Last relayed block (relaychain)</td>
          <td>
            <v-progress-linear v-if="loading.relayedGrandpaBlockHash" :color="relayChain.color" indeterminate/>
            <external-explorer
              v-else
              :identity="`${source.relayedGrandpaBlock}`"
              type="block"
              :chain="relayChain"
            />
          </td>
        </tr>
        <tr>
          <td>Last parachain by relayed at source (relaychain)</td>
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
        </tbody>
      </v-table>
    </v-col>
  </v-row>
</template>

<script lang="ts" setup>

import {defineProps, inject, onBeforeUnmount, onMounted, PropType, reactive, toRefs, toRaw } from 'vue'
import {BridgeSubstrateChainInfo} from "@/types/app";
import {ApiPromise} from "@polkadot/api";
import ExternalExplorer from "@/components/widgets/external-explorer.vue";



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
  paraHeadAtTarget: _ParaHeadAtTarget;
  relayedGrandpaBlockHash: string;
  paraHeadAtSourceByLastRelayedGrandpa: string;
  relayedGrandpaBlock: number;
}

interface _ParaHeadAtTarget {
  atRelayBlockNumber: number;
  headHash: number;
}

const state = reactive({
  loading: {
    paraHeadAtTarget: true,
    relayedGrandpaBlockHash: true,
    paraHeadAtSourceByLastRelayedGrandpa: true,
  },
  source: {} as _StateSource,
  subscriber: {
    paraHeadAtTarget: null,
    relayedGrandpaBlockHash: null,
    paraHeadAtSourceByLastRelayedGrandpa: null,
  },
});

const {loading, source, subscriber} = toRefs(state);



async function initState() {
  const {relayChain, relayClient, targetChain, targetClient, paraChain, paraClient} = props;
  // @ts-ignore
  const bridgeTarget = targetChain.bridge_target[paraChain.bridge_chain_name];
  // @ts-ignore
  subscriber.value.paraHeadAtTarget = await targetClient?.query[bridgeTarget.query_name.parachain]
    .parasInfo(bridgeTarget.para_id, async (v: any) => {
      source.value.paraHeadAtTarget = v.toJSON().bestHeadHash;
      loading.value.paraHeadAtTarget = false;
    });

  // @ts-ignore
  subscriber.value.relayedGrandpaBlockHash = await targetClient?.query[bridgeTarget.query_name.grandpa]
    .bestFinalized(async (v: any) => {
      const [blockNumber, blockHash] = v.toHuman() as unknown as [string, string];
      source.value.relayedGrandpaBlock = +(blockNumber.replaceAll(',', ''))
      source.value.relayedGrandpaBlockHash = blockHash;

      loading.value.relayedGrandpaBlockHash = false;
      loading.value.paraHeadAtSourceByLastRelayedGrandpa = true;

      // @ts-ignore
      const atBlockRelayClient = await toRaw(relayClient).at(blockHash);
      const paraHeadAtSourceByLastRelayedGrandpa = await atBlockRelayClient.query.paras.heads(bridgeTarget.para_id);
      // @ts-ignore
      source.value.paraHeadAtSourceByLastRelayedGrandpa = paraHeadAtSourceByLastRelayedGrandpa?.toJSON();
      loading.value.paraHeadAtSourceByLastRelayedGrandpa = false;
    });
}

onMounted(() => {
  initState();
});

onBeforeUnmount(() => {
  // @ts-ignore
  subscriber.value.bestParaHeads && subscriber.value.bestParaHeads();
  subscriber.value.relayedGrandpaBlockHash && subscriber.value.relayedGrandpaBlockHash();
});
</script>
