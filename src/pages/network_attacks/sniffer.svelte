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
  import { current_net_interface } from "../../js/store.js";
  import { saveAs } from 'file-saver';

  export let f7router;

  let _ip = "";
  let ws;
  let path = "";

  onMount(async () => {
    _ip = localStorage.getItem("ip");
    ws = new WebSocket("ws://" + _ip + "/sniffer");
    ws.onmessage = function (msg) {
      if (msg.data == "connected") ws.send($current_net_interface.name);
      else {
        if(msg.data instanceof Blob) {
            saveAs(msg.data, path);
        } else {
            path = msg.data.replace("/home/capibarazero/wifi_captures/", "");
        }
        console.log(msg.data);
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
    <p>Sniffer is running...</p>
    <Button color="red" fill on:click={stop_attack}>Stop attack</Button>
  </Block>
</Page>
