<template>
  <v-btn-toggle v-model="plot_ref" color="primary" multiple>
    <v-btn value="HUMIDITY">
      <v-icon>mdi-water-percent</v-icon>
    </v-btn>

    <v-btn value="TEMP">
      <v-icon>mdi-thermometer</v-icon>
    </v-btn>
  </v-btn-toggle>
</template>

<script setup lang="ts">
import { defineExpose, ref } from "vue";
import Plot from "../../interfaces/device.interface.ts";
const plot_ref = ref(["HUMIDITY", "TEMP"]);

defineExpose({ plot_ref: plot_ref });
</script>
