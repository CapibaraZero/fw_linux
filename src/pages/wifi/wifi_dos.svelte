<script>
  import {
    f7,
    Page,
    Navbar,
    NavTitle,
    Button,
    Block
  } from "framework7-svelte";
  import { onMount } from "svelte";
  import { CapacitorHttp } from "@capacitor/core";
  import { current_net_interface } from "../../js/store.js";
  export let f7router;
  export let f7route;

  let _ip = "";
  let ws;
  let ready = false;
  let sent = false;

  async function enable_monitor_mode(show_alert = true) {
    const options = {
      url:
        "http://" +
        _ip +
        "/enable_monitor_mode?interface=" +
        $current_net_interface.name,
    };

    const response = await CapacitorHttp.get(options);
    console.log(response);
    f7.dialog.close();
  }

  async function disable_monitor_mode() {
    const options = {
      url:
        "http://" +
        _ip +
        "/disable_monitor_mode?interface=" +
        $current_net_interface.name,
    };

    await CapacitorHttp.get(options);
    $current_net_interface.name.replace("mon", "");
  }

  let closed = false;
  onMount(async () => {
    _ip = localStorage.getItem("ip");
    await enable_monitor_mode();
    console.log(f7route);
    $current_net_interface.name += "mon";
    console.log($current_net_interface);
    ws = new WebSocket("ws://" + _ip + "/wifi_dos");
    // ws.send($current_net_interface.name);
    ws.onmessage = function (msg) {
      console.log(msg);
      //   if (msg.data != "connected") terminal.write(msg.data);
      if (!sent) {
        ws.send(
          JSON.stringify({
            interface: $current_net_interface.name,
            channel: +f7route.params.channel,
            bssid: f7route.params.bssid,
          })
        );
        sent = true;
      }
    };
    ws.onclose = async function () {
      if (!closed) {
        f7.dialog.alert("Unexpected error. Please check server log.", () =>
          f7router.back()
        );
      }
      await disable_monitor_mode();
    };
  });

  function stop_attack() {
    closed = true;
    ws.send("stop");
    f7router.back();
  }
</script>

<Page name="home">
  <!-- Top Navbar -->
  <Navbar sliding={false} backLink="Back">
    <NavTitle sliding>CapibaraZero</NavTitle>
  </Navbar>

  <Block>
    <Button color="red" on:click={stop_attack} fill>Stop DoS</Button>
  </Block>
</Page>
