<template>
  <v-row>
    <v-col cols="12">
      <h2 class="text-h5 font-weight-thin">Feemarket</h2>
    </v-col>
    <v-col cols="12" v-if="source.ampleRelayers">
      <v-table density="compact">
        <thead>
        <tr>
          <th class="text-left" style="width: 50%;">Account</th>
          <th class="text-left">Balance</th>
          <th class="text-left">Locked</th>
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
          <td>
            <v-progress-linear :color="sourceChain.color" indeterminate/>
          </td>
        </tr>
        <tr v-for="(relayer, ix) in source.assignedRelayers" :key="relayer.id">
          <td>
            <external-explorer :identity="relayer.id" type="address" :chain="sourceChain"/>
          </td>
          <td><code>{{ relayer.balance }}</code></td>
          <td><code>{{ relayer.locked }}</code></td>
          <td>
            <code :class="{'green--text': ix === source.assignedRelayers.length - 1}" v-text="relayer.fee"/>
          </td>
        </tr>
        </tbody>
      </v-table>
    </v-col>
    <v-col cols="12" v-else>
      <p>No assigned relayers</p>
    </v-col>
  </v-row>
</template>


<script lang="ts" setup>
import {defineProps, onMounted, PropType, reactive, toRefs} from 'vue'
import {EvmClient} from "@/plugins/eth2/evm";
import {ExecutionClient} from "@/plugins/eth2/execution";
import {BridgeEthereumChainInfo} from "@/types/app";
import BigNumber from "bignumber.js";
import ExternalExplorer from "@/components/widgets/external-explorer.vue";


const props = defineProps({
  sourceChain: {
    type: Object as PropType<BridgeEthereumChainInfo>,
  },
  targetChain: {
    type: Object as PropType<BridgeEthereumChainInfo>,
  },
  sourceClient: {
    type: Object as PropType<EvmClient> | PropType<ExecutionClient>,
  },
});


interface _StateSource {
  ampleRelayers: boolean;
  assignedRelayers: _AssignedRelayers[];
}

interface _AssignedRelayers {
  id: string;
  balance: BigNumber;
  locked: BigNumber;
  fee: BigNumber;
}

interface _StateLoading {
  assignedRelayers: boolean;
}


const state = reactive({
  source: {} as _StateSource,
  loading: {
    assignedRelayers: true,
  } as _StateLoading,
})

const {source, loading} = toRefs(state);

async function initState() {
  const {sourceChain, targetChain, sourceClient} = props;
  loading.value.assignedRelayers = true;
  const bridgeTarget = sourceChain.bridge_target[targetChain.bridge_chain_name];
  const assignedRelayers = await sourceClient
    .feemarket(bridgeTarget.contract.feemarket)
    .assignedRelayers();
  loading.value.assignedRelayers = false;
  if (!assignedRelayers || assignedRelayers.isEmpty) {
    source.value.ampleRelayers = false;
    return;
  }
  source.value.ampleRelayers = true;
  const precision = new BigNumber(10).pow(sourceChain.precisionEvm || sourceChain.precision);
  source.value.assignedRelayers = assignedRelayers.map(item => {
    const balance = new BigNumber(item.balance.toString()).div(precision);
    const locked = new BigNumber(item.locked.toString()).div(precision);
    const fee = new BigNumber(item.fee.toString()).div(precision);
    return {...item, balance, locked, fee};
  });
}


onMounted(() => {
  initState();
});

</script>

