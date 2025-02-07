<template>
  <div class="wide">
    <VuePlotly
      ref="plot_temp"
      @plotly_click="onPlotlyClick"
      :data="data_humidity"
      :layout="layout_temp"
    />
  </div>
  <div class="wide">
    <VuePlotly
      ref="plot_humidity"
      @plotly_click="onPlotlyClick"
      :data="data_temp"
      :layout="layout_humidity"
    />
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, h } from "vue";
import ThermoService from "..//services/thermo";
import Reading from "../interfaces/device.interface";
import Device from "../interfaces/device.interface";

import Plotly from "plotly.js";
import VuePlotly from "vue3-plotly-ts";

export default defineComponent({
  components: { VuePlotly: VuePlotly },
  name: "ThermoPlot",
  props: {},
  setup(props, ctx) {
    const plot_temp = ref<typeof VuePlotly>();
    const plot_humidity = ref<typeof VuePlotly>();

    const layout_temp = ref<Partial<Plotly.Layout>>({
      height: 500,
      title: "temperature",
    });
    const layout_humidity = ref<Partial<Plotly.Layout>>({
      height: 500,
      title: "humidity",
    });

    const data_temp = ref<Plotly.Data[]>([]);
    const data_humidity = ref<Plotly.Data[]>([]);

    onMounted(async () => {
      const device_result = await ThermoService.get_devices();
      const readings = device_result.map(async (device) => {
        return {
          data: await ThermoService.get_all_readings_for_device(device),
          name: device.name,
        };
      });

      for (const i of readings) {
        const reading = await i;

        const temperature_readings = reading.data.map(
          (r) => r.temperature / 10 + Math.random()
        );
        const humidity_readings = reading.data.map(
          (r) => r.humidity + Math.random()
        );
        const timestamps = reading.data.map((r) => r.time_stamp);
        data_humidity.value.push({
          x: timestamps,
          y: humidity_readings,
          type: "scatter",
          mode: "lines",
          name: reading.name,
        });
        data_temp.value.push({
          x: timestamps,
          y: temperature_readings,
          type: "scatter",
          mode: "lines",
          name: reading.name,
        });
      }
    });

    return {
      data_temp,
      data_humidity,
      layout_temp,
      layout_humidity,
      plot_temp,
      plot_humidity,
    };
  },
});
</script>
