<template>
  <v-app>
    <v-main>
      <v-container>
        <v-row cols="12">
          <v-col
            cols="12"
            sm="6"
            md="6"
            lg="6"
            xl="3"
            xxl="3"
            v-for="device in devices"
            :key="device.device_name.name"
          >
            <ThermoCard :device="device" />
          </v-col>
        </v-row>
        <ThermoPlot />
      </v-container>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
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
  setup() {
    const devices = ref<Reading[]>([]);
    useIntervalFn(async () => {
      let default_date = new Date();
      default_date.setDate(default_date.getDate() - 7);
      const devices_result = await ThermoService.get_newest_readings(
        default_date
      );
      devices.value = devices_result;
    }, 1000);
    console.log(devices);
    return { devices };
  },
});
</script>
