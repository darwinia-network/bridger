<template>

  <v-navigation-drawer app v-if="cond.enable_drawer ">
    <v-row>
      <v-col cols="12">

        <v-list dense>
          <v-list-group no-action v-for="group in source.bridge_groups"
                        :key="group.name"
                        prepend-icon="mdi-bridge">
            <template v-slot:activator>
              <v-list-item-title class="text-uppercase">{{ group.name }}</v-list-item-title>
            </template>

            <template v-for="(bridge, i) in group.bridges">
              <v-tooltip bottom>
                <template v-slot:activator="{ on, attrs }">

                  <v-list-item
                    :key="`${bridge.name}-${i}`"
                    link
                    v-bind="attrs"
                    v-on="on"
                    :to="`/status/${bridge.name}`"
                  >
                    <v-list-item-title v-text="bridge.name"/>
                    <v-list-item-icon v-if="bridge.mode === 'testnet'">
                      <v-icon>mdi-test-tube</v-icon>
                    </v-list-item-icon>
                  </v-list-item>
                </template>
                <span>{{ bridge.name }}</span>
              </v-tooltip>
            </template>
          </v-list-group>

<!--
          <v-list-group prepend-icon="mdi-dev-to">
            <template v-slot:activator>
              <v-list-item-title class="text-uppercase">DEVELOPMENT</v-list-item-title>
            </template>
            <v-list-item>
              <drawer-development/>
            </v-list-item>
          </v-list-group>
-->

          <v-list-group prepend-icon="mdi-cog">
            <template v-slot:activator>
              <v-list-item-title class="text-uppercase">SETTINGS</v-list-item-title>
            </template>
            <v-list-item>
              <drawer-settings @change="changedSettings"/>
            </v-list-item>
          </v-list-group>

        </v-list>


      </v-col>
    </v-row>
  </v-navigation-drawer>

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
    },
    cond: {
      enable_drawer: true,
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
  watch: {
    '$route.path': {
      handler(path) {
        const vm = this;
        vm.cond.enable_drawer = path !== '/';
      },
      deep: true,
    }
  },
  created() {
    const vm = this;
    vm.cond.enable_drawer = vm.$route.path !== '/';
    vm.changedSettings();
  }
}
</script>


<style scoped>
.box-bar {
  left: 3% !important;
  top: 5% !important;
  max-height: 80% !important;
  height: auto !important;
  overflow-y: auto !important;

  border: 1px dotted #ccc;
}

.btn-mininav {
  position: fixed;
  left: 3%;
  top: 5%;
  z-index: 2;
}

</style>
