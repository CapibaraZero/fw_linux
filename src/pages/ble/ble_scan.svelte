<script>
  import {
    Page,
    Navbar,
    NavTitle,
    List,
    ListItem,
    Button
  } from "framework7-svelte";
  import { onMount } from "svelte";

  let _ip = "";
  let ble_devices = [];
  let ws;

  onMount(() => {
    _ip = localStorage.getItem("ip");
    ws = new WebSocket("ws://" + _ip + "/ble_scan");
    ws.onmessage = (event) => {
      ble_devices.push(JSON.parse(event.data));
      ble_devices = ble_devices;
    };
  });

  export let f7router;

  function stop_scan() {
    ws.send("stop");
    f7router.back();
  }
</script>

<Page name="home">
  <!-- Top Navbar -->
  <Navbar sliding={false} backLink="Back">
    <NavTitle sliding>CapibaraZero</NavTitle>
  </Navbar>

  <List dividersIos mediaList outlineIos strongIos>
    {#each ble_devices as device}
      <ListItem
        link="#"
        title={"Name: " + device.name}
        after={"Class: " + device.icon}
        subtitle={"RSSI: " + device.rssi}
        text={"TX Power: " + device.tx_power}
      ></ListItem>
    {/each}
    <Button color="red" fill on:click={stop_scan}>Stop</Button>
  </List>
</Page>
