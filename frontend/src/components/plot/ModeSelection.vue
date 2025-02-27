<template>
  <v-btn-toggle
    :small="true"
    variant="outlined"
    v-model="plot_ref"
    color="primary"
    multiple
  >
    <v-tooltip text="toggle humidity curves" location="top">
      <template v-slot:activator="{ props }">
        <v-btn v-bind="props" value="HUMIDITY">
          <v-icon>mdi-water-percent</v-icon>
        </v-btn>
      </template>
    </v-tooltip>
    <v-tooltip text="toggle temperature curves" location="top">
      <template v-slot:activator="{ props }">
        <v-btn v-bind="props" value="TEMP">
          <v-icon>mdi-thermometer</v-icon>
        </v-btn>
      </template>
    </v-tooltip>
  </v-btn-toggle>
</template>

<script setup lang="ts">
import { ref } from "vue";
const plot_ref = ref(["TEMP"]);
defineExpose({ plot_ref: plot_ref });
</script>
