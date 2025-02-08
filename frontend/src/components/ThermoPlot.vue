<template>
  <TimeSelection ref="date_ref" />
  <div class="wide">
    <VuePlotly ref="plot_temp" :data="data_humidity" :layout="layout_temp" />
  </div>
  <div class="wide">
    <VuePlotly
      ref="plot_humidity"
      :data="data_temp"
      :layout="layout_humidity"
    />
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, Ref, onMounted, h } from "vue";
import TimeSelection from "./plot/TimeSelection.vue";
import ThermoService from "../services/thermo.service";
import Reading from "../interfaces/device.interface";

import Plotly from "plotly.js";
import VuePlotly from "vue3-plotly-ts";
import { useIntervalFn } from "@vueuse/core";

const get_device_readings = async (start_date: Date) => {
  const device_result = await ThermoService.get_devices(start_date);
  const readings = device_result.map(async (device) => {
    return {
      data: await ThermoService.get_all_readings_for_device(device, start_date),
      name: device.name,
    };
  });
  return readings;
};

const define_plot_ref = (name: string, axis_title: string) => {
  return ref<Partial<Plotly.Layout>>({
    height: 500,
    title: { text: name },

    yaxis: {
      title: {
        text: axis_title,
      },
    },
  });
};
export default defineComponent({
  components: { VuePlotly: VuePlotly, TimeSelection: TimeSelection },
  name: "ThermoPlot",
  props: {},
  setup(props, ctx) {
    const ref_to_time_selector = ref();

    const plot_temp = ref<typeof VuePlotly>();
    const plot_humidity = ref<typeof VuePlotly>();

    const layout_temp = define_plot_ref("temperature", "°C");
    const layout_humidity = define_plot_ref("humidity", "%");

    const data_temp = ref<Plotly.Data[]>([]);
    const data_humidity = ref<Plotly.Data[]>([]);

    const update_plot = async () => {
      data_humidity.value = [];
      data_temp.value = [];
      for (const reading_promise of await get_device_readings(
        ref_to_time_selector.value.date_ref
      )) {
        const reading = await reading_promise;
        const temperature_readings = reading.data.map(
          (r: Reading) => r.temperature / 10 + Math.random()
        );
        const humidity_readings = reading.data.map(
          (r: Reading) => r.humidity + Math.random()
        );
        const timestamps = reading.data.map((r) => r.time_stamp);

        const generate_data = (
          y_coordinates: number[],
          name: string
        ): Plotly.Data => {
          return {
            x: timestamps,
            y: y_coordinates,
            type: "scatter",
            mode: "lines",
            name: name,
          };
        };

        data_humidity.value.push(
          generate_data(humidity_readings, reading.name)
        );
        data_temp.value.push(generate_data(temperature_readings, reading.name));
      }
    };

    onMounted(async () => {
      update_plot();
      const { pause, resume, isActive } = useIntervalFn(async () => {
        update_plot();
      }, 10000);
    });

    return {
      data_temp,
      data_humidity,
      layout_temp,
      layout_humidity,
      plot_temp,
      plot_humidity,
      date_ref: ref_to_time_selector,
    };
  },
});
</script>
