<template>
  <div>
    <h1>{{ device.device_name.name }}</h1>
    <h2 v-if="device.device_name.nickname !== null">
      {{ device.device_name.nickname }}
    </h2>
    <h2 v-else>no description</h2>
    <h2>{{ device.temperature / 10 }}°C</h2>
    <h2>{{ device.humidity }}%</h2>
    <h2>
      last update: {{ (Date.now() - new Date(device.time_stamp)) / 1000 }} s ago
    </h2>
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
