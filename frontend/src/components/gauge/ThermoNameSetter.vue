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
    <v-tooltip text="cancel editing description">
      <template v-slot:activator="{ props }">
        <v-btn
          variant="plain"
          size="x-large"
          icon="mdi-cancel"
          @click="set_disabled(false)"
          v-bind="props"
        ></v-btn>
      </template>
    </v-tooltip>
  </v-row>
  <v-row v-else>
    <v-col
      class="my-4 text-h6 text-lg-h4 text-md-h5"
      style="text-overflow: ellipsis; overflow: hidden; white-space: nowrap"
      :cols="mouse_ref == true ? 9 : 12"
      >{{
        device?.device_name.nickname ? device.device_name.nickname : "no name"
      }}
    </v-col>
    <v-col class="text-h5" cols="3" v-if="mouse_ref || is_disabled">
      <v-tooltip text="edit device description">
        <template v-slot:activator="{ props }">
          <v-btn
            variant="plain"
            size="x-large"
            icon="mdi-pencil"
            @click="set_disabled(true)"
            v-bind="props"
          ></v-btn>
        </template>
      </v-tooltip>
    </v-col>
  </v-row>
</template>

<script lang="ts">
import { defineComponent, ref, toRefs } from "vue";
import ThermoService from "../../services/thermo.service";

export default defineComponent({
  name: "ThermoNameSetter",
  props: {
    device: Object,
    mouse_state: Boolean,
  },
  setup(props) {
    const update_description = async (message: string) => {
      await ThermoService.update_device_description(
        props.device?.device_name.name,
        message
      );
      is_disabled.value = false;
      msg.value = "";
    };
    const { mouse_state } = toRefs(props);
    const mouse_ref = mouse_state;

    const msg = ref("");
    const is_disabled = ref(false);
    const set_disabled = (val: boolean) => {
      is_disabled.value = val;
    };
    return {
      msg,
      is_disabled,
      mouse_ref,
      update_description,
      set_disabled,
    };
  },
});
</script>
