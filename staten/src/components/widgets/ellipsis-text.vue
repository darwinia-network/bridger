<template>
  <v-row>
    <v-col cols="5" >
      <span class="ellipsis-text">{{ text }}</span>
      <v-tooltip activator="parent" location="bottom">
        {{ text }}
      </v-tooltip>
    </v-col>
    <v-col cols="3">
      <!--      v-clipboard:copy="text" v-clipboard:success="onCopy"-->
      <v-btn variant="plain" size="x-small" icon="mdi-content-copy"></v-btn>
    </v-col>
  </v-row>
</template>

<script lang="ts" setup>

import {defineProps} from 'vue'


const props = defineProps({
  text: {
    type: String,
  }
});

</script>
<style>
.ellipsis-text {
  width: 100%;
  display: block;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}
</style>
