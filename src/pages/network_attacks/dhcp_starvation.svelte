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

  let _ip = "";
  let ws;
  onMount(() => {
    _ip = localStorage.getItem("ip");
    ws = new WebSocket("ws://" + _ip + "/dhcp_starvation");
    ws.onmessage = function (msg) {
        console.log(msg);
        console.log($current_net_interface.name);
        ws.send($current_net_interface.name);
    }
    // ws.onopen = function () {
    // console.log($current_net_interface);
    //   ws.send($current_net_interface.name);
    // };
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
  <Button fill color="red" on:click={stop_attack}>Stop</Button>
</Page>
