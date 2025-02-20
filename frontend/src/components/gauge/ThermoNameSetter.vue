<template>
  <v-row v-if="is_disabled">
    <v-col>
      <v-text-field
        v-model="msg"
        append-inner-icon="mdi-pencil"
        density="compact"
        label="input description"
        variant="plain"
        single-line
        @click:append-inner="update_description(msg)"
      >
      </v-text-field>
    </v-col>
    <v-btn
      variant="plain"
      size="x-large"
      icon="mdi-cancel"
      @click="set_disabled(false)"
    ></v-btn>
  </v-row>
  <v-row v-else>
    <v-col
      class="my-4 text-h4"
      style="text-overflow: ellipsis; overflow: hidden; white-space: nowrap"
      cols="9"
      >{{
        device?.device_name.nickname ? device.device_name.nickname : "no name"
      }}
    </v-col>
    <v-col class="text-h5" cols="2">
      <v-btn
        variant="plain"
        size="x-large"
        icon="mdi-pencil"
        @click="set_disabled(true)"
      ></v-btn>
    </v-col>
  </v-row>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import ThermoService from "../../services/thermo.service";
export default defineComponent({
  name: "ThermoNameSetter",
  props: {
    device: Object,
  },
  setup(props, ctx) {
    const update_description = async (message: string) => {
      await ThermoService.update_device_description(
        props.device?.device_name.name,
        message
      );
      is_disabled.value = false;
      msg.value = "";
    };
    const msg = ref("");
    const is_disabled = ref(false);
    const set_disabled = (val: boolean) => {
      console.log(val);
      is_disabled.value = val;
    };
    return { msg, is_disabled, update_description, set_disabled };
  },
});
</script>
