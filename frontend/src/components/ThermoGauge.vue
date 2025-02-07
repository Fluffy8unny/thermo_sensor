<template>
  <div>
    <h1>{{ device.device_name.name }}</h1>
    <h2>{{ device.temperature / 10 }}°C</h2>
    <h2>{{ device.humidity }}%</h2>
  </div>
</template>

<script>
import { defineComponent } from "vue";
import Reading from "../interfaces/device.interface";
export default defineComponent({
  name: "ThermoGauge",
  props: {
    device: Reading,
  },
});
</script>
