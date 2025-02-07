export default interface Device {
  name: string;
  nickname: string | null;
}

export default interface Reading {
  device_name: Device;
  temperature: number;
  humidity: number;
  time_stamp: string;
}
