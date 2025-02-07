<template>
  <div class="wide" style="display: flex">
    <ThermoGauge
      v-for="device in devices"
      :key="device.device_name"
      :device="device"
    />
  </div>
  <ThermoPlot />
</template>

<script lang="ts">
import { defineComponent, ref, onMounted } from "vue";

import ThermoGauge from "./components/ThermoGauge.vue";
import ThermoPlot from "./components/ThermoPlot.vue";
import ThermoService from ".//services/thermo";
import Reading from "./interfaces/device.interface";
import VuePlotly from "vue3-plotly-ts";

export default defineComponent({
  name: "App",
  components: {
    ThermoGauge,
    ThermoPlot,
  },
  setup(props, ctx) {
    const devices = ref<Reading[]>([]);
    onMounted(async () => {
      const devices_result = await ThermoService.get_newest_readings();
      devices.value = devices_result;
    });
    return { devices };
  },
});
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
