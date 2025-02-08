<template>
  <input v-model="msg" placeholder="edit me" />
  <button :disabled="is_disabled" @click="update_description(msg)">
    update description
  </button>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import ThermoService from "../services/thermo.service";
export default defineComponent({
  name: "ThermoNameSetter",
  props: {
    device: Object,
  },
  setup(props, ctx) {
    const update_description = async (message: string) => {
      is_disabled.value = true;
      await ThermoService.update_device_description(
        props.device?.device_name.name,
        message
      );
      is_disabled.value = false;
      msg.value = "";
    };
    console.log(props.device);
    const msg = ref("");
    const is_disabled = ref(false);
    return { msg, is_disabled, update_description };
  },
});
</script>
