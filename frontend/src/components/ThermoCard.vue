<template>
  <v-card
    class="my-auto mx-auto"
    variant="elevated"
    @mouseover="set_state(true)"
    @mouseleave="set_state(false)"
  >
    <v-row align="center" no-gutters>
      <v-col>
        <v-card-item prepend-icon="mdi-thermometer" elevation="16">
          <template v-slot:title>
            <p class="text-subtitle-2 text-lg-h6 text-md-subtitle-1">
              {{ device?.device_name.name }}
            </p>
          </template>
          <template v-slot:subtitle>
            <v-tooltip :text="get_text()">
              <template v-slot:activator="{ props }">
                <v-icon
                  v-bind="props"
                  :color="get_color()"
                  class="me-1 pb-1"
                  icon="mdi-update"
                  size="18"
                ></v-icon>
              </template>
            </v-tooltip>
            updated:
            {{ cur_time_s() }}
            s ago.
          </template>
        </v-card-item>
      </v-col>
      <v-col>
        <ThermoNameSetter
          :device="device"
          :mouse_state="mouse_state.valueOf()"
        />
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
import { defineComponent, ref, toRefs } from "vue";
import ThermoGauge from "./gauge/ThermoGauge.vue";
import ThermoNameSetter from "./gauge/ThermoNameSetter.vue";
export default defineComponent({
  name: "ThermoCard",
  components: { ThermoGauge, ThermoNameSetter },
  props: {
    device: Object,
  },
  setup(props) {
    const mouse_state = ref(false);
    let set_state = (val: boolean) => {
      mouse_state.value = val;
    };
    const { device: dev_ref } = toRefs(props);
    const cur_time_s = () =>
      Math.floor(
        (Date.now() - new Date(dev_ref.value?.time_stamp).getTime()) / 1000
      );
    const get_color = () => {
      const time_s = cur_time_s();
      if (time_s < 11) return "success";
      if (time_s < 120) return "warning";
      return "error";
    };
    const get_text = () => {
      const time_s = cur_time_s();
      if (time_s < 11) return "device online";
      if (time_s < 120) return "device online?";
      return "device offline";
    };
    return { mouse_state, set_state, cur_time_s, get_color, get_text };
  },
});
</script>
