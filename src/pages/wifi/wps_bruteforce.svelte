<script>
  import {
    f7,
    Page,
    Navbar,
    NavTitle,
    Button,
    Block,
    List,
    ListItem,
    Actions,
    ActionsGroup,
    ActionsLabel,
    ActionsButton,
    Popup,
    NavRight,
  } from "framework7-svelte";
  import { onMount } from "svelte";
  import { CapacitorHttp } from "@capacitor/core";
  import { current_net_interface } from "../../js/store.js";
  import { Terminal } from "@xterm/xterm";
  import { FitAddon } from '@xterm/addon-fit';
  import { disable_monitor_mode, enable_monitor_mode } from '../../js/wifi/wifi_helper.js';
  import "@xterm/xterm/css/xterm.css";
  
  export let f7router;
  export let f7route;

  let _ip = "";
  let ws;
//   let ready = false;
  let sent = false;
  let terminal;

  onMount(async () => {
    _ip = localStorage.getItem("ip");
    console.log($current_net_interface);
    await enable_monitor_mode(_ip, $current_net_interface.name);
    $current_net_interface.name += "mon";
    console.log(f7route)
    terminal = new Terminal();
    const fitAddon = new FitAddon();
    terminal.loadAddon(fitAddon);
    terminal.open(document.getElementById("terminal"));
    fitAddon.fit();
    ws = new WebSocket("ws://" + _ip + "/wps_bruteforce");
    // ws.send($current_net_interface.name);
    ws.onmessage = function (msg) {
        console.log(msg);
      if (msg.data != "connected") {
        terminal.clear();
        terminal.write(msg.data);
      }
      if (!sent) {
        ws.send(
          JSON.stringify({
            interface: $current_net_interface.name,
            bssid: f7route.params.bssid,
            wps_pixiedust: f7route.params.pixiedust
          })
        );
        sent = true;
      }
    };
  });

  async function stop_attack() {
    await disable_monitor_mode(_ip, $current_net_interface.name);
    $current_net_interface.name.replace("mon", "");
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
    <Button color="red" fill on:click={stop_attack}>Stop attack</Button>
    <div id="terminal"></div>
  </Block>
</Page>
