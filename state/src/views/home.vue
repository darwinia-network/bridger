<template>
  <v-container fluid pa-0>
    <v-row align="center"
           justify="center"
           style="height:100vh"
           dense>
      <v-col
        cols="12" lg="8" md="8"
        class="d-flex flex-column justify-center align-center"
      >
        <h1 class="text-h1 text-uppercase">Bridger state</h1>
        <h2 class="subtitle-1 text-uppercase">Darwinia</h2>
        <p class="mt-5"></p>

        <v-card tile dense class="mt-2" width="50%" v-for="group in source.bridge_groups">
          <v-card-title>
            <v-icon>mdi-bridge</v-icon>
            <v-spacer/>
            <span class="subtitle-2">{{ group.name }}</span>
          </v-card-title>
          <v-divider/>

          <v-list dense>
            <v-list-item :to="`/state/${bridge.name}`" v-for="bridge in group.bridges">
              <v-list-item-content>
                <v-list-item-title>{{ bridge.name }}</v-list-item-title>
              </v-list-item-content>
              <v-list-item-icon v-if="bridge.mode === 'testnet'">
                <v-icon>mdi-test-tube</v-icon>
              </v-list-item-icon>
            </v-list-item>
          </v-list>
        </v-card>

<!--
        <v-card tile dense class="mt-2" width="50%">
          <v-card-title>
            <v-icon>mdi-dev-to</v-icon>
            <v-spacer/>
            <span class="subtitle-2 text-uppercase">Development</span>
          </v-card-title>
          <v-divider/>
          <drawer-development/>
        </v-card>
-->

        <v-card tile dense class="mt-2" width="50%">
          <v-card-title>
            <v-icon>mdi-cog</v-icon>
            <v-spacer/>
            <span class="subtitle-2 text-uppercase">Settings</span>
          </v-card-title>
          <v-divider/>
          <drawer-settings @change="changedSettings"/>
        </v-card>

      </v-col>
    </v-row>
  </v-container>
</template>

<script>
import * as dataSource from '@/data/data_source.js'

import DrawerSettings from '@/components/drawer/drawer-settings'
import DrawerDevelopment from '@/components/drawer/drawer-development'


export default {
  name: 'page-home',
  components: {
    DrawerSettings,
    DrawerDevelopment,
  },
  data: vm => ({
    source: {
      bridge_groups: [],
    }
  }),
  methods: {
    changedSettings(setting) {
      const vm = this;
      setting = setting || {};
      const enableTestnet = setting.enableTestnet === undefined ? true : setting.enableTestnet;
      vm.source.bridge_groups = dataSource.bridgerGroups({
        allowDisabled: true,
        enableTestnet,
      });
    },
  },
  created() {
    const vm = this;
    vm.changedSettings();
  }
}
</script>
