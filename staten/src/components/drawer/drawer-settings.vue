<template>
  <v-container>
    <v-row>
      <v-col cols="12" md="12" sm="6">
        <v-checkbox v-model="settings.enableTestnet" label="Allow testnet" @change="changedData"/>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts" setup>
import {onMounted, reactive, toRefs} from 'vue'
import {AppSettings} from "@/types/app";

const state = reactive({
  settings: {
    enableTestnet: true,
  } as AppSettings
});

const {
  settings,
} = toRefs(state);

const emit = defineEmits(['change']);


function changedData() {
  emit('change', settings.value);
}

onMounted(() => {
  changedData();
});
</script>
