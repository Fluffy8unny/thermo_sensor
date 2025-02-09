<template>
  <v-card class="my-2">
    <v-card-item
      class="text-h5"
      title="Recent readings"
      prepend-icon="mdi-chart-bar"
    >
    </v-card-item>
    <v-card-text>
      <v-row>
        <v-col>
          <VuePlotly ref="plot_temp" :data="data_temp" :layout="layout_temp" />
        </v-col>
      </v-row>
      <v-row>
        <v-col class="d-flex justify-center">
          <ModeSelection @click="update_plot" ref="mode_ref" class="mx-2" />
          <TimeSelection
            @click="update_plot"
            ref="date_ref"
            class="mx-2"
            @redraw="update_plot"
          />
          <v-btn-toggle
            color="divided"
            v-model="impossible_ref"
            variant="outlined"
          >
            <v-btn size="large" icon="mdi-refresh" @click="update_plot"></v-btn>
          </v-btn-toggle>
        </v-col>
      </v-row>
    </v-card-text>
  </v-card>
</template>
<script lang="ts">
import { defineComponent, ref, Ref, onMounted, watch } from "vue";
import TimeSelection from "./plot/TimeSelection.vue";
import ModeSelection from "./plot/ModeSelection.vue";
import ThermoService from "../services/thermo.service";
import { Plot, Reading } from "../interfaces/device.interface";

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
const line_color = "#555";
const define_plot_ref = () => {
  return ref<Partial<Plotly.Layout>>({
    paper_bgcolor: "rgba(0,0,0,0)",
    plot_bgcolor: "rgba(0,0,0,0)",
    height: 450,
    font: {
      family: "Courier New, monospace",
      size: 14,
      color: "#fff",
    },
    margin: {
      l: 100,
      r: 100,
      b: 80,
      t: 20,
      pad: 4,
    },
    legend: {
      x: 1.05,
      xanchor: "left",
      y: 1.0,
    },
    xaxis: {
      gridcolor: line_color,
    },
    yaxis: {
      title: {
        text: "Temperature [°C]",
      },
      gridcolor: line_color,
    },
    yaxis2: {
      title: {
        text: "Humidity [%]",
      },
      overlaying: "y",
      gridcolor: line_color,
      side: "right",
    },
  });
};

const plot_colors = [
  "#1f77b4", // muted blue
  "#ff7f0e", // safety orange
  "#2ca02c", // cooked asparagus green
  "#d62728", // brick red
  "#9467bd", // muted purple
  "#8c564b", // chestnut brown
  "#e377c2", // raspberry yogurt pink
  "#7f7f7f", // middle gray
  "#bcbd22", // curry yellow-green
  "#17becf", // blue-teal
];

export default defineComponent({
  components: {
    VuePlotly: VuePlotly,
    TimeSelection: TimeSelection,
    ModeSelection: ModeSelection,
  },
  name: "ThermoPlot",
  props: {},
  setup(props, ctx) {
    const ref_to_time_selector = ref();
    const ref_to_mode_selector = ref();
    const impossible_ref = 27;
    const plot_temp = ref<typeof VuePlotly>();

    const layout_temp = define_plot_ref();

    const data_temp = ref<Plotly.Data[]>([]);
    const data_humidity = ref<Plotly.Data[]>([]);

    const update_plot = async () => {
      data_humidity.value = [];
      data_temp.value = [];
      var ctr = 0;
      for (const reading_promise of await get_device_readings(
        ref_to_time_selector.value.date_ref
      )) {
        const reading = await reading_promise;
        const temperature_readings = reading.data.map(
          (r: Reading) => r.temperature / 10
        );
        const humidity_readings = reading.data.map((r: Reading) => r.humidity);
        const timestamps = reading.data.map((r: Reading) => r.time_stamp);

        const generate_data = (
          y_coordinates: number[],
          name: string,
          type: Plot
        ): Plotly.Data => {
          const name_suffix =
            type === "TEMP" ? "temperature [°C]" : "humidity [%]";
          const axis = type === "TEMP" ? "y" : "y2";
          return {
            x: timestamps,
            y: y_coordinates,
            type: "scatter",
            mode: "lines",
            name: `${name}`,
            line: {
              color: plot_colors[ctr],
              dash: type === "HUMIDITY" ? "dot" : "solid",
            },
            yaxis: axis,
          };
        };

        ref_to_mode_selector.value.plot_ref.includes("TEMP") &&
          data_temp.value.push(
            generate_data(temperature_readings, reading.name, "TEMP")
          );
        ref_to_mode_selector.value.plot_ref.includes("HUMIDITY") &&
          data_temp.value.push(
            generate_data(humidity_readings, reading.name, "HUMIDITY")
          );
        ctr = ++ctr % plot_colors.length;
      }
    };

    onMounted(async () => {
      update_plot();
    });

    return {
      data_temp,
      data_humidity,
      layout_temp,
      plot_temp,
      date_ref: ref_to_time_selector,
      mode_ref: ref_to_mode_selector,
      impossible_ref,
      update_plot,
    };
  },
});
</script>
