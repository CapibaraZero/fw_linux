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
  import "@xterm/xterm/css/xterm.css";
  
  export let f7router;

  let _ip = "";
  let ws;
  let ready = false;
  let sent = false;
  let terminal;

  onMount(async () => {
    _ip = localStorage.getItem("ip");
    console.log($current_net_interface)
    terminal = new Terminal();
    const fitAddon = new FitAddon();
    terminal.loadAddon(fitAddon);
    terminal.open(document.getElementById("terminal"));
    fitAddon.fit();
    ws = new WebSocket("ws://" + _ip + "/launch_bettercap");
    // ws.send($current_net_interface.name);
    ws.onmessage = function (msg) {
      if(msg.data != "connected")
        terminal.write(msg.data);
      if(!sent) {
        ws.send($current_net_interface.name);
        sent = true;
        setTimeout(() => {
        ready = true;
        console.log("Open popup");
        window.open("http://" + $current_net_interface.ips[0] + ":8080", '_blank');
      }, 5000);
      }
    };
  });

  function stop_attack() {
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
    {#if ready}
      <p>Ettercap launched at {$current_net_interface.ips[0]}:8080</p>
      <Button fill on:click={stop_attack}>Stop attack</Button>
    {:else}
      <p>Waiting for ettercap to be launched...</p>
    {/if}
    <div id="terminal"></div>
  </Block>
</Page>
