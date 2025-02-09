<template>
  <v-card class="my-4 mx-auto" variant="elevated">
    <v-row align="center" no-gutters>
      <v-col class="text-h4">
        <v-card-item
          :title="device?.device_name.name"
          prepend-icon="mdi-thermometer"
        >
          <template v-slot:subtitle>
            <v-icon class="me-1 pb-1" icon="mdi-update" size="18"></v-icon>
            updated:
            {{
              (Date.now() -
                new Date((device as Reading).time_stamp).getTime()) /
              1000
            }}
            s ago.
          </template>
        </v-card-item>
      </v-col>
      <v-col>
        <ThermoNameSetter :device="device" />
      </v-col>
    </v-row>
    <v-divider></v-divider>
    <v-card-text>
      <v-row align="center" no-gutters>
        <ThermoGauge :device="device" />
      </v-row>
    </v-card-text>
  </v-card>
</template>

<script lang="ts">
import { defineComponent, onMounted } from "vue";
import { Reading } from "../interfaces/device.interface";
import ThermoGauge from "./gauge/ThermoGauge.vue";
import ThermoNameSetter from "./gauge/ThermoNameSetter.vue";

export default defineComponent({
  name: "ThermoCard",
  components: { ThermoGauge, ThermoNameSetter },
  props: {
    device: Object,
  },
});
</script>
