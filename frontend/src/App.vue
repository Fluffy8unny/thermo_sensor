<template>
  <div class="wide" style="display: flex">
    <div v-for="device in devices" :key="device.device_name.name">
      <ThermoGauge :device="device" />
      <ThermoNameSetter :device="device" />
    </div>
  </div>
  <ThermoPlot />
</template>

<script lang="ts">
import { defineComponent, ref, onMounted } from "vue";
import { useIntervalFn } from "@vueuse/core";

import ThermoGauge from "./components/ThermoGauge.vue";
import ThermoPlot from "./components/ThermoPlot.vue";
import ThermoNameSetter from "./components/ThermoNameSetter.vue";

import ThermoService from ".//services/thermo.service";
import { Reading } from "./interfaces/device.interface";

export default defineComponent({
  name: "App",
  components: {
    ThermoGauge,
    ThermoNameSetter,
    ThermoPlot,
  },
  setup(props, ctx) {
    const devices = ref<Reading[]>([]);
    onMounted(async () => {
      const { pause, resume, isActive } = useIntervalFn(async () => {
        let default_date = new Date();
        default_date.setDate(default_date.getDate() - 7);
        const devices_result = await ThermoService.get_newest_readings(
          default_date
        );
        devices.value = devices_result;
      }, 1000);
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
