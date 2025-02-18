<script>
  import { f7, Page, Navbar, NavTitle, Button, Block } from "framework7-svelte";
  import { onMount } from "svelte";
  import { current_net_interface } from "../../js/store.js";
  export let f7router;
  export let f7route;

  let _ip = "";
  let ws;
  let sent = false;

  let closed = false;
  onMount(async () => {
    _ip = localStorage.getItem("ip");
    // await enable_monitor_mode();
    console.log(f7route);
    // $current_net_interface.name += "mon";
    console.log($current_net_interface);
    ws = new WebSocket("ws://" + _ip + "/beacon_spam");
    // ws.send($current_net_interface.name);
    ws.onmessage = function (msg) {
      console.log(msg);
      //   if (msg.data != "connected") terminal.write(msg.data);
      if (!sent) {
        ws.send($current_net_interface.name);
        sent = true;
      }
    };
    ws.onclose = async function () {
      if (!closed) {
        f7.dialog.alert("Unexpected error. Please check server log.", () =>
          f7router.back()
        );
      }
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
    <Button color="red" on:click={stop_attack} fill>Stop Beacon Spam</Button>
  </Block>
</Page>
