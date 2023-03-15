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
    allowDisabled: false,
  } as AppSettings
});

const {
  settings,
} = toRefs(state);

const emit = defineEmits(['change']);

function loadData() {
  const saved = localStorage.getItem('APP_SETTINGS');
  if (!saved) return;
  settings.value = JSON.parse(saved);
}

function changedData() {
  const jsonValue = JSON.stringify(settings.value);
  localStorage.setItem('APP_SETTINGS', jsonValue);
  emit('change', settings.value);
}

onMounted(() => {
  loadData();
  changedData();
});
</script>
