<template>
  <v-btn-toggle v-model="time_ref" variant="outlined">
    <v-tooltip text="select date range" location="top">
      <template v-slot:activator="{ props }">
        <v-btn value="1" @click="setTimeOffset(1)" v-bind="props">1 day</v-btn>
        <v-btn value="7" @click="setTimeOffset(7)" v-bind="props">1 week</v-btn>
        <v-btn value="30" @click="setTimeOffset(30)" v-bind="props"
          >1 month</v-btn
        >
        <v-btn value="365" @click="setTimeOffset(365)" v-bind="props"
          >1 year</v-btn
        >
        <v-btn value="null" @click="setTime" v-bind="props"
          >from beginning</v-btn
        >
      </template>
    </v-tooltip>
  </v-btn-toggle>
</template>

<script setup lang="ts">
import { ref } from "vue";
const date_ref = ref(new Date());
const time_ref = ref("30");
const setTime = () => {
  date_ref.value = new Date(+0);
};
const setTimeOffset = (offset: number) => {
  time_ref.value = offset.toString();
  const date = new Date();
  date.setDate(date.getDate() - offset);
  date_ref.value = date;
};

setTimeOffset(7);
defineExpose({ date_ref: date_ref });
</script>
