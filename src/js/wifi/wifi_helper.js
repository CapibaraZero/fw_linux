import { CapacitorHttp } from "@capacitor/core";

export async function enable_monitor_mode(ip, _interface) {
    const options = {
      url:
        "http://" +
        ip +
        "/enable_monitor_mode?interface=" +
        _interface,
    };

    const response = await CapacitorHttp.get(options);
    console.log(response);
    // f7.dialog.close();
  }

export async function disable_monitor_mode(ip, _interface) {
    const options = {
      url:
        "http://" +
        ip +
        "/disable_monitor_mode?interface=" +
        _interface,
    };

    await CapacitorHttp.get(options);
  }
