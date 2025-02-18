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
    import { Terminal } from "@xterm/xterm";
    import { FitAddon } from '@xterm/addon-fit';
    import "@xterm/xterm/css/xterm.css";
    
    export let f7router;
    export let f7route;
  
    let _ip = "";
    let ws;
  //   let ready = false;
    // let sent = false;
    let terminal;
  
    onMount(async () => {
      _ip = localStorage.getItem("ip");
      console.log(f7route)
      terminal = new Terminal();
      const fitAddon = new FitAddon();
      terminal.loadAddon(fitAddon);
      terminal.open(document.getElementById("terminal"));
      fitAddon.fit();
      ws = new WebSocket("ws://" + _ip + "/nfc_nested_attack");
      // ws.send($current_net_interface.name);
      ws.onmessage = function (msg) {
          console.log(msg);
        if (msg.data != "connected") {
            const len = msg.data.split("\n").length;
          terminal.clear();
          terminal.write(msg.data.split("\n").slice(len - 5, len).join("\n"));
        }
        // if (!sent) {
        //   ws.send(
        //     JSON.stringify({
        //       interface: $current_net_interface.name,
        //       bssid: f7route.params.bssid,
        //       wps_pixiedust: f7route.params.pixiedust
        //     })
        //   );
        //   sent = true;
        // }
      };
    });
  
    async function stop_attack() {
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
  