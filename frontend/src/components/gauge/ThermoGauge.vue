<template>
  <Thermometer :value="device.temperature / 10" :settings="therm_settings" />
  <Thermometer :value="device.humidity" :settings="humidity_settings" />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { Reading } from "../interfaces/device.interface";
import ThermometerDisplay from "./Thermometer.vue";

export default defineComponent({
  name: "ThermoGauge",
  components: { Thermometer: ThermometerDisplay },
  props: {
    device: Object,
  },
  setup(props, ctx) {
    const therm_settings = {
      min_val: 15,
      max_val: 35,
      unit: "°C",
      color_bulb: "#32de84",
      color_top: "#ff0000",
    };
    const humidity_settings = {
      min_val: 0,
      max_val: 100,
      unit: "%",
      color_bulb: "#7CB9E8",
      color_top: "#7CB9E8",
    };
    return { therm_settings, humidity_settings };
  },
});
</script>
