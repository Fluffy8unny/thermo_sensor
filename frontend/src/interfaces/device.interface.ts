export type Device = {
  name: string;
  nickname: string | null;
}

export type Reading = {
  device_name: Device;
  temperature: number;
  humidity: number;
  time_stamp: string;
}

export type Plot = "TEMP" | "HUMIDITY";

export type ThermometerSettings = {
  min_val: number;
  max_val: number;
  unit: string;
  color_bulb: string;
  color_top : string;
};
