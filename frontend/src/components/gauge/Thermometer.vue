<template>
  <div id="wrapper">
    <div id="termometer">
      <div
        id="temperature"
        v-bind:style="{ height: temp_height }"
        :data-value="temp_reading"
      ></div>
      <div id="graduations"></div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import { ThermometerSettings } from "@/interfaces/device.interface";
export default defineComponent({
  name: "ThermometerDisplay",
  props: {
    value: Number,
    settings: Object as PropType<ThermometerSettings>,
  },
  setup(props, ctx) {
    const min_temp: number = props.settings?.min_val || (0 as number);
    const max_temp: number = props.settings?.max_val || (1 as number);

    const temp_reading: string = props.value
      ? `${props.value}${props.settings?.unit || "°C"}`
      : "No data";
    const temp_height: string = props.value
      ? `${(100 * (props.value - min_temp)) / (max_temp - min_temp)}%`
      : "0%";

    const color_bulb = props.settings?.color_bulb || "#2affd8";
    const color_top = props.settings?.color_top || "#ff0000";

    return { temp_reading, temp_height, color_bulb, color_top };
  },
});
</script>

<style scoped lang="scss">
// VARIABLES
$TM-mainTint: #3d3d44;
$TM-backgroundColor: darken($TM-mainTint, 0%);
$TM-borderSize: 5px;
$TM-borderColor: darken($TM-mainTint, 8%);
$TM-width: 15px;
$TM-height: 140px;
$TM-bulbSize: $TM-width * 2;
$TM-radius: 20px;
$TM-graduationsStyle: 2px solid rgba(0, 0, 0, 0.5);
$TM-bulbColor: v-bind(color_bulb);
$TM-mercuryColor: linear-gradient(v-bind(color_top), $TM-bulbColor) no-repeat
  bottom;

// Tooltip
$TM-tooltipColor: $TM-mainTint;
$TM-tooltipSize: 1.5em;
$TM-tooltipRadius: 5px;
$TM-tooltipTopShift: 5px;
$TM-tooltipVerticalPadding: 5px;
$TM-tooltipHorizontalPadding: $TM-tooltipVerticalPadding * 2;
$TM-tooltipLeftShift: 100%;
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
    bottom: -$TM-bulbSize + $TM-borderSize;
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
      background: $TM-tooltipColor;
      color: white;
      z-index: 2;
      width: 3em;
      padding: $TM-tooltipVerticalPadding $TM-tooltipHorizontalPadding;
      border-radius: $TM-tooltipRadius;
      font-size: $TM-tooltipSize;
      line-height: 1;
      transform: translateY(50%);
      left: calc(#{$TM-tooltipLeftShift} + 0.75em / #{$TM-tooltipArrowWidth});
      top: calc(
        -1em + #{$TM-tooltipTopShift} - #{$TM-tooltipVerticalPadding} * 2
      );
    }

    // Tooltip arrow
    &:after {
      content: "";
      border-top: $TM-tooltipSize / $TM-tooltipArrowHeight solid transparent;
      border-bottom: $TM-tooltipSize / $TM-tooltipArrowHeight solid transparent;
      border-right: $TM-tooltipSize / $TM-tooltipArrowWidth solid
        $TM-tooltipColor;
      left: $TM-tooltipLeftShift;
      top: calc(
        -#{$TM-tooltipSize} / #{$TM-tooltipArrowHeight} + #{$TM-tooltipTopShift}
      );
    }
  }
}
</style>
