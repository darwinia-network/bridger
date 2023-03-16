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
import BridgeSkeleton from "@/components/skeleton/bridge-skeleton.vue";


import {Bridge} from "@/types/bridge";

const props = defineProps({
  bridge: {
    type: Object as PropType<Bridge>,
  },
})

async function initState() {
  const name = props.bridge.name;
  console.log(name);
}

onMounted(() => {
  initState();
});
</script>

