<template>
  <v-app-bar flat>
    <v-app-bar-title>
      <v-icon icon="mdi-bridge"/>
      Bridge State
    </v-app-bar-title>

    <v-btn to="/">Home</v-btn>
    <v-menu location="bottom" open-on-hover v-for="(item) in bridge_groups" :key="`bridge-menu-${item.name}`">
      <template v-slot:activator="{ props }">
        <v-btn v-bind="props">{{ item.name }}</v-btn>
      </template>

      <v-list>
        <v-list-item v-for="(bridge) in item.bridges"
                     :to="`/state/${bridge.name}`"
                     :key="`bridge-${bridge.name}`">
          <v-list-item-title>{{ bridge.name }}</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-menu>
  </v-app-bar>
</template>

<script lang="ts" setup>

import {onMounted, reactive, toRefs} from 'vue'
import {BridgeGroup} from "@/types/bridge";
import {AppSettings} from "@/types/app";
import * as dataSource from "@/data/data_source";

const state = reactive({
  bridge_groups: [] as BridgeGroup[],
});

const {
  bridge_groups,
} = toRefs(state);

function initBridgeGroups(settings?: AppSettings) {
  bridge_groups.value = dataSource.bridgerGroups(settings);
}

onMounted(() => {
  initBridgeGroups()
});
</script>
