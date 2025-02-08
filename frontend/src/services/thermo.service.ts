import axios from "axios";
import Device from "@/interfaces/device.interface";
import Reading from "@/interfaces/device.interface";

const backend_url = "http://localhost:8081";

const convert_date = (date: Date): string => date.toISOString();

const get_devices = async (start_date: Date): Promise<Device[]> => {
  const date_string = convert_date(start_date);
  const response = await axios.get(
    `${backend_url}/all_device_names/${date_string}`
  );
  return response.data;
};

const get_newest_readings = async (start_date: Date): Promise<Reading[]> => {
  const date_string = convert_date(start_date);
  const response = await axios.get(
    `${backend_url}/current_reading/${date_string}`
  );
  const device = response.data;
  return device;
};

const get_all_readings_for_device = async (
  device_name: Device,
  start_date: Date
): Promise<Reading[]> => {
  const date_string = convert_date(start_date);
  const response = await axios.get(
    `${backend_url}/device/${device_name.name}/${date_string}`
  );
  const readings = response.data.map((reading: any) => {
    return {
      device_name: reading.device_name,
      temperature: reading.temperature,
      humidity: reading.humidity,
      time_stamp: reading.time_stamp,
    };
  });
  return readings;
};

const update_device_description = async (name: string, nickname: string) => {
  return await axios.post(`${backend_url}/nickname/${name}/${nickname}`, {});
};

const ThermoService = {
  get_devices: get_devices,
  get_newest_readings: get_newest_readings,
  get_all_readings_for_device,
  update_device_description,
};
export default ThermoService;
