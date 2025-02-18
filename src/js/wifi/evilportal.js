import { hotspot_uuid } from "../store.js";
import { f7 } from "framework7-svelte";
import { CapacitorHttp } from "@capacitor/core";

let _selected_interface = "";
let _ip = "";
let _f7router;

async function evil_portal_request(portal_type, ssid, password, blob) {
  // await choise_captive_portal();
  f7.dialog.preloader();
  const options = {
    url: "http://" + _ip + "/wifi_ap",
    headers: { "content-type": "application/json" },
    data: {
      interface: _selected_interface,
      ssid: ssid,
      password: password,
      google: portal_type == "google",
      custom: blob == null ? [] : blob,
    },
  };
  try {
    const res = await CapacitorHttp.post(options);
    console.log(res);
    hotspot_uuid.set(res.data);
    f7.dialog.close();
    _f7router.navigate("/evil_portal/");
  } catch (error) {
    console.error(error);
    f7.dialog.close();
    f7.dialog.alert("Can't start EvilPortal");
  }
}

let _ssid = "";
let _password = "";

export function start_evilportal(file_container, selected_interface, ip, f7router) {
  //   if (selected_interface == "") {
  //     f7.dialog.alert("Please, select an interface");
  //   } else {
  f7.dialog.login("Enter your SSID and password", async (ssid, password) => {
    if (password.length < 8) {
      f7.dialog.alert("Password is too short");
    } else {
        _ssid = ssid;
        _password = password;
        _selected_interface = selected_interface;
        _ip = ip;
        _f7router = f7router;
      f7.dialog
        .create({
          title: "Vertical Buttons",
          buttons: [
            {
              text: "Google",
              onClick: async function () {
                await evil_portal_request("google", ssid, password, null);
              },
            },
            {
              text: "Custom zip",
              onClick: function () {
                file_container.click();
              },
            },
          ],
          verticalButtons: true,
        })
        .open();
    }
  });
}

function readFile(file) {
    return new Promise((resolve, reject) => {
      // Create file reader
      let reader = new FileReader()
  
      // Register event listeners
      reader.addEventListener("loadend", e => resolve(e.target.result))
      reader.addEventListener("error", reject)
  
      // Read file
      reader.readAsArrayBuffer(file)
    })
}

export async function handle_custom_evil_portal(evt) {
  const dump = evt.target.files[0];
  console.log(dump);
  const array = new Uint8Array(await readFile(dump));
  await evil_portal_request("custom", _ssid, _password, Array.from(array));
}
