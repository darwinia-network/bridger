<template>
  <v-row>
    <v-col cols="12" md="2" class="pt-8" v-if="sourceChain">
      <v-row>
        <v-col cols="12" class="d-flex flex-column align-center">
          <v-tooltip bottom>
            <template v-slot:activator="{ on, attrs }">
              <v-avatar v-bind="attrs" v-on="on" size="64" v-ripple>
                <img :src="sourceChain.logo" :alt="sourceChain.name" v-if="sourceChain.logo">
                <v-icon v-if="!sourceChain.logo" size="54">mdi-alpha-c</v-icon>
              </v-avatar>
              <h2 v-bind="attrs" v-on="on" class="text-h6 font-weight-light">{{ sourceChain.name }}</h2>
            </template>
            <span>Source chain</span>
          </v-tooltip>
        </v-col>
        <v-col cols="12" class="text-center">
          <v-tooltip bottom v-if="source.header.last">
            <template v-slot:activator="{ on, attrs }">
              <p class="title text-block-number" v-bind="attrs" v-on="on">
                <span
                  class="yellow lighten-5"
                  v-if="cond.highlight.last"
                  v-text="source.header.last.number"
                />
                <span v-else v-text="source.header.last.number"/>
                <v-btn icon small :href="`${sourceChain.explorer}/block/${source.header.last.number}`" target="_blank">
                  <v-icon small>mdi-open-in-new</v-icon>
                </v-btn>
              </p>
            </template>
            <span>Last block</span>
          </v-tooltip>
          <v-tooltip bottom v-if="source.header.finalized">
            <template v-slot:activator="{ on, attrs }">
              <p class="subtitle-1 text-block-number" v-bind="attrs" v-on="on">
                <span
                  class="yellow lighten-5"
                  v-if="cond.highlight.finalized"
                  v-text="source.header.finalized.number"
                />
                <span v-else v-text="source.header.finalized.number"/>
                <v-btn icon x-small :href="`${sourceChain.explorer}/block/${source.header.finalized.number}`"
                       target="_blank">
                  <v-icon x-small>mdi-open-in-new</v-icon>
                </v-btn>
              </p>
            </template>
            <span>Finalized block</span>
          </v-tooltip>
        </v-col>
      </v-row>
    </v-col>

    <v-col cols="12" md="8">
      <slot/>
    </v-col>

    <v-col cols="12" md="2" class="pt-8" v-if="targetChain">
      <v-row>
        <v-col cols="12" class="d-flex flex-column align-center">
          <v-tooltip bottom>
            <template v-slot:activator="{ on, attrs }">
              <v-avatar v-bind="attrs" v-on="on" size="64" v-ripple>
                <img :src="targetChain.logo" :alt="targetChain.name" v-if="targetChain.logo">
                <v-icon v-if="!targetChain.logo" size="54">mdi-alpha-c</v-icon>
              </v-avatar>
              <h2 v-bind="attrs" v-on="on" class="text-h6 font-weight-light">{{ targetChain.name }}</h2>
            </template>
            <span>Target chain</span>
          </v-tooltip>
        </v-col>
      </v-row>
    </v-col>
  </v-row>
</template>

<script>

async function _initState(vm) {
  vm.client.rpc.chain.subscribeNewHeads(header => {
    vm.source.header.last = header;
    vm.cond.highlight.last = true;
    setTimeout(() => vm.cond.highlight.last = false, 500);
  });
  vm.client.rpc.chain.subscribeFinalizedHeads(header => {
    vm.source.header.finalized = header;
    vm.cond.highlight.finalized = true;
    setTimeout(() => vm.cond.highlight.finalized = false, 500);
  });
}

export default {
  props: {
    sourceClient: {
      type: Object,
    },
    sourceChain: {
      type: Object,
    },
    targetChain: {
      type: Object,
    },
  },
  data: () => ({
    source: {
      delaySourceClient: null,
      header: {
        last: null,
        finalized: null,
      }
    },
    cond: {
      highlight: {
        last: false,
        finalized: false,
      }
    }
  }),
  methods: {
    initState(sourceClient) {
      this.source.delaySourceClient = sourceClient;
      _initState(this);
    },
    tesv(v) {
      console.log(v);
    }
  },
  computed: {
    client() {
      return this.sourceClient || this.source.delaySourceClient;
    }
  },
  created() {
    // initState(this);
  }
}
</script>

<style scoped>

</style>
