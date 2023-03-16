<template>
  <v-container fluid>
    <v-row>
      <v-col cols="12">
        <bridge-skeleton>
          para with para b
        </bridge-skeleton>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts" setup>

import {onMounted, defineProps, PropType, reactive, toRefs} from 'vue'
import BridgeSkeleton from "@/components/skeleton/bridge-skeleton.vue"

import * as dataSource from '@/data/data_source'

import {Bridge} from "@/types/bridge";
import {useRouter} from "vue-router";

const router = useRouter();

const props = defineProps({
  bridge: {
    type: Object as PropType<Bridge>,
  },
})

async function initState() {
  if (!props.bridge) {
    await router.push({path: '/'});
    return;
  }
  const name = props.bridge?.name;
  const [leftParaChainName, rightParaChainName] = name.split('-');
  const [leftParaChain, rightParaChain] = [
    dataSource.chainInfo(leftParaChainName),
    dataSource.chainInfo(rightParaChainName),
  ];
  const leftRelayChainName = leftParaChain.bridge_target[rightParaChainName].relay_chain;
  const rightRelayChainName = rightParaChain.bridge_target[leftParaChainName].relay_chain;
  const [leftRelayChain, rightRelayChain] = [
    dataSource.chainInfo(leftRelayChainName),
    dataSource.chainInfo(rightRelayChainName),
  ];

  console.log(leftParaChain, rightParaChain);
  console.log(leftRelayChain, rightRelayChain);
}

onMounted(() => {
  initState();
});
</script>

