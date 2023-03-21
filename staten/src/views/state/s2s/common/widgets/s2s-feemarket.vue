<template>
  <v-row>
    <v-col cols="12">
      <h2 class="text-h5 font-weight-thin">Fee market</h2>
    </v-col>
    <v-col cols="12" v-if="source.ampleRelayers">
      <v-table density="compact">
        <template v-slot:default>
          <thead>
          <tr>
            <th class="text-left">Account</th>
            <th class="text-left">Collateral</th>
            <th class="text-left">Fee ({{ sourceChain.currency }})</th>
          </tr>
          </thead>
          <tbody>
          <tr v-if="loading.assignedRelayers">
            <td>
              <v-progress-linear :color="sourceChain.color" indeterminate/>
            </td>
            <td>
              <v-progress-linear :color="sourceChain.color" indeterminate/>
            </td>
            <td>
              <v-progress-linear :color="sourceChain.color" indeterminate/>
            </td>
          </tr>
          <tr v-for="(relayer, ix) in source.assignedRelayers" :key="relayer.id">
            <td>
              <external-explorer :identity="relayer.id" type="account" :chain="sourceChain"/>
            </td>
            <td><code>{{ relayer.collateral }}</code></td>
            <td>
              <code :class="{'green--text': ix === source.assignedRelayers.length - 1}" v-text="relayer.fee"/>
            </td>
          </tr>
          </tbody>
        </template>
      </v-table>
    </v-col>
    <v-col cols="12" v-else>
      <p>No assigned relayers</p>
    </v-col>
  </v-row>
</template>

<script lang="ts" setup>

import BigNumber from "bignumber.js";
import {defineProps, onBeforeUnmount, onMounted, PropType, reactive, toRaw, toRefs} from "vue";
import {BridgeSubstrateChainInfo} from "@/types/app";
import {ApiPromise} from "@polkadot/api";
import ExternalExplorer from "@/components/widgets/external-explorer.vue";


const props = defineProps({
  sourceChain: {
    type: Object as PropType<BridgeSubstrateChainInfo>,
  },
  sourceClient: {
    type: Object as PropType<ApiPromise>,
  },
  targetChain: {
    type: Object as PropType<BridgeSubstrateChainInfo>,
  },
});

interface _StateSource {
  ampleRelayers: boolean,
  assignedRelayers: Record<string, any>[],
}

interface _StateLoading {
  assignedRelayers: boolean,
}

const state = reactive({
  source: {} as _StateSource,
  loading: {} as _StateLoading,
  subscriber: {
    assignedRelayers: null,
  },
});

const {source, loading, subscriber} = toRefs(state);

async function initState() {
  const {sourceChain, targetChain} = props;
  const sourceClient = toRaw(props.sourceClient);
  const bridgeTarget = sourceChain.bridge_target[targetChain.bridge_chain_name];
  subscriber.value.assignedRelayers = await sourceClient.query[bridgeTarget.query_name.feemarket]
    .assignedRelayers((v: any) => {
      source.value.ampleRelayers = v.isSome;
      source.value.assignedRelayers = v.toJSON();
      loading.value.assignedRelayers = false;
      const precision = new BigNumber(10).pow(sourceChain.precision);
      source.value.assignedRelayers = source.value.assignedRelayers.map(item => {
        const collateral = new BigNumber(item.collateral);
        const fee = new BigNumber(item.fee);
        return {
          ...item,
          collateral: collateral.div(precision),
          fee: fee.div(precision),
        }
      });
    });
}

onMounted(() => {
  initState();
});


onBeforeUnmount(() => {
  // @ts-ignore
  subscriber.value.assignedRelayers && subscriber.value.assignedRelayers();
});

</script>
