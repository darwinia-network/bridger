<template>
  <v-app app id="inspire">
    <drawer-wrapper/>
    <v-main>
      <router-view/>
    </v-main>
  </v-app>
</template>


<script>

import DrawerWrapper from '@/components/drawer/drawer-wrapper.vue';

export default {
  name: 'page-home',
  components: {
    DrawerWrapper,
  },
  data: vm => ({}),
}
</script>

<style lang="scss">
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
}

</style>
