<script>
  import { f7, Page, Navbar, NavTitle, Button, Block } from "framework7-svelte";
  import { onMount } from "svelte";
  import { ir_buttons } from "../../js/store.js";
  import { CapacitorHttp } from "@capacitor/core";
  import "@xterm/xterm/css/xterm.css";

  let _ip = $state("");

  onMount(async () => {
    _ip = localStorage.getItem("ip");
    console.log($ir_buttons);
  });

  async function send_signal(signal) {
    try {

      const options = {
        url: "http://" + _ip + "/ir_send",
        headers: { "Content-Type": "application/json" },
        data: signal,
      };

      const res = await CapacitorHttp.post(options);
      console.log(res);

      f7.notification
        .create({
          icon: '<i class="icon material-settings_remote"></i>',
          title: "IR Sender",
          subtitle: "Signal sent successfully",
          text: "",
          closeButton: true,
          closeOnClick: true,
          closeTimeout: 3000,
        })
        .open();
        
    } catch (error) {
      console.error(error);
      f7.dialog.alert("Error during sending signal");
    }
  }
</script>

<Page name="home">
  <!-- Top Navbar -->
  <Navbar sliding={false} backLink="Back">
    <NavTitle sliding>CapibaraZero</NavTitle>
  </Navbar>

  <Block>
    <div class="grid grid-cols-3 grid-gap">
      {#each $ir_buttons as button}
        <Button round fill on:click={() => send_signal(button)}
          >{button.btn_name}</Button
        >
      {/each}
    </div>
  </Block>
</Page>
