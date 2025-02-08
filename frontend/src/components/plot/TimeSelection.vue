<template>
  <div>
    <button @click="setTimeOffset(1)">1 day</button>
    <button @click="setTimeOffset(7)">1 week</button>
    <button @click="setTimeOffset(30)">1 month</button>
    <button @click="setTimeOffset(365)">1 year</button>
    <button @click="setTime">from beginning</button>
  </div>
</template>

<script setup lang="ts">
import { defineExpose, ref } from "vue";

const date_ref = ref(new Date());
const setTime = () => {
  date_ref.value = new Date(+0);
};

const setTimeOffset = (offset: number) => {
  const date = new Date();
  date.setDate(date.getDate() - offset);
  date_ref.value = date;
};

setTimeOffset(7);
defineExpose({ date_ref: date_ref });
</script>
