<template>
  <v-app>
    <v-main>
      <v-container>
        <v-row>
          <v-col v-for="device in devices" :key="device.device_name.name">
            <ThermoCard :device="device" />
          </v-col>
        </v-row>
        <ThermoPlot />
      </v-container>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted } from "vue";
import { useIntervalFn } from "@vueuse/core";

import ThermoCard from "./components/ThermoCard.vue";
import ThermoPlot from "./components/ThermoPlot.vue";

import ThermoService from ".//services/thermo.service";
import { Reading } from "./interfaces/device.interface";

export default defineComponent({
  name: "App",
  components: {
    ThermoCard,
    ThermoPlot,
  },
  setup(props, ctx) {
    const devices = ref<Reading[]>([]);
    const { pause, resume, isActive } = useIntervalFn(async () => {
      let default_date = new Date();
      default_date.setDate(default_date.getDate() - 7);
      const devices_result = await ThermoService.get_newest_readings(
        default_date
      );
      devices.value = devices_result;
    }, 1000);
    return { devices };
  },
});
</script>
