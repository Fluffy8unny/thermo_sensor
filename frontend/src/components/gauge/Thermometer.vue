<template>
  <span id="wrapper">
    <span id="termometer">
      <span
        id="temperature"
        v-bind:style="{ height: temp_height }"
        :data-value="temp_reading"
      ></span>
      <span id="graduations"></span>
    </span>
    <span class="text-h6">{{ temp_unit }}</span>
  </span>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import { ThermometerSettings } from "@/interfaces/device.interface";
import { useTheme } from "vuetify";

export default defineComponent({
  name: "ThermometerDisplay",
  props: {
    value: Number,
    settings: Object as PropType<ThermometerSettings>,
  },
  setup(props) {
    const min_temp: number = props.settings?.min_val || (0 as number);
    const max_temp: number = props.settings?.max_val || (1 as number);

    const temp_reading = `${props?.value || 0}`;
    const temp_unit: string = props?.settings?.unit || "";

    const temp_height: string = props.value
      ? `${(100 * (props.value - min_temp)) / (max_temp - min_temp)}%`
      : "0%";
    const vTheme = useTheme();
    const color_border = vTheme.current.value.colors.background;
    const color_out = vTheme.current.value.colors.surface;
    const color_font = vTheme.current.value.colors["on-background"];

    const color_bulb = props.settings?.color_bulb || "#2affd8";
    const color_top = props.settings?.color_top || "#ff0000";

    return {
      temp_reading,
      temp_unit,
      temp_height,
      color_bulb,
      color_top,
      color_out,
      color_border,
      color_font,
    };
  },
});
</script>

<style scoped lang="scss">
@use "sass:math";

// VARIABLES
$TM-mainTint: v-bind(color_out);
$TM-backgroundColor: v-bind(color_out);
$TM-borderColor: v-bind(color_border);
$TM-height: 7vh;
$TM-width: 2vh;
$TM-bulbSize: $TM-width * 1.75;
$TM-borderSize: $TM-bulbSize * 0.1;

$TM-radius: math.div($TM-height, 4);
$TM-graduationsStyle: 2px solid rgba(0, 0, 0, 0.5);
$TM-bulbColor: v-bind(color_bulb);
$TM-mercuryColor: linear-gradient(v-bind(color_top), $TM-bulbColor) no-repeat
  bottom;

// Tooltip
$TM-tooltipColor: $TM-mainTint;
$TM-tooltipSize: 5vw;
$TM-tooltipRadius: 5px;
$TM-tooltipTopShift: 5px;
$TM-tooltipVerticalPadding: 5px;
$TM-tooltipHorizontalPadding: $TM-tooltipVerticalPadding * 2;
$TM-tooltipLeftShift: 80%;
$TM-tooltipArrowWidth: 1.5; // Higher numbers produce smaller width
$TM-tooltipArrowHeight: 2.2; // Higher numbers produce smaller height

@mixin border() {
  border: $TM-borderSize solid $TM-borderColor;
}

#wrapper {
  margin: auto;
  display: flex;
  flex-direction: column;
  align-items: center;
  position: relative;
  left: -5%;
}

#termometer {
  width: $TM-width;
  background: $TM-backgroundColor;
  height: $TM-height;
  position: relative;
  @include border;
  border-radius: $TM-radius;
  z-index: 1;
  margin-bottom: $TM-bulbSize;

  &:before,
  &:after {
    position: absolute;
    content: "";
    border-radius: 50%;
  }

  &:after {
    transform: translateX(-50%);
    width: $TM-bulbSize;
    height: $TM-bulbSize;
    background-color: $TM-bulbColor;
    bottom: -$TM-bulbSize * 0.8;
    @include border;
    z-index: -3;
    left: 50%;
  }

  #graduations {
    height: 59%;
    top: 20%;
    width: 50%;

    &,
    &:before {
      position: absolute;
      border-top: $TM-graduationsStyle;
      border-bottom: $TM-graduationsStyle;
    }

    &:before {
      content: "";
      height: 34%;
      width: 100%;
      left: -0.1em;
      top: 32%;
    }
  }

  #temperature {
    bottom: 0;
    background: $TM-mercuryColor;
    width: 100%;
    border-radius: $TM-radius;
    background-size: 100% $TM-height;
    transition: all 0.2s ease-in-out;

    &,
    &:before,
    &:after {
      position: absolute;
    }

    &:before {
      content: attr(data-value);
      color: v-bind(color_font);
      z-index: 2;
      width: $TM-tooltipSize 4;
      padding: $TM-tooltipVerticalPadding $TM-tooltipHorizontalPadding;
      border-radius: $TM-tooltipRadius;
      font-size: min(3vh, 3vw);
      line-height: 1;
      transform: translateY(50%);
      left: calc(#{$TM-tooltipLeftShift} + 0.45em / #{$TM-tooltipArrowWidth});
      top: calc(
        -1em + #{$TM-tooltipTopShift} - #{$TM-tooltipVerticalPadding} * 2
      );
    }

    // Tooltip arrow
    &:after {
      content: "";
      border-top: calc($TM-tooltipSize / $TM-tooltipArrowHeight) solid
        transparent;
      border-bottom: calc($TM-tooltipSize / $TM-tooltipArrowHeight) solid
        transparent;
      border-right: calc($TM-tooltipSize / $TM-tooltipArrowWidth) solid
        $TM-tooltipColor;
      left: $TM-tooltipLeftShift;
      top: calc(
        -#{$TM-tooltipSize} / #{$TM-tooltipArrowHeight} + #{$TM-tooltipTopShift}
      );
    }
  }
}
</style>
