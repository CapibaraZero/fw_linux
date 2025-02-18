<script>
  import {
    f7,
    Page,
    Navbar,
    NavLeft,
    NavTitle,
    NavTitleLarge,
    NavRight,
    Link,
    Toolbar,
    Block,
    BlockTitle,
    List,
    ListItem,
    ListButton,
    Button,
    Actions,
    ActionsGroup,
    ActionsLabel,
    ActionsButton,
  } from "framework7-svelte";
  import { onMount } from "svelte";
  import { ap } from "../../js/store.js";

  let _ip = "";
  // import { CapacitorHttp } from "@capacitor/core";
  // console.log($ap);

  // let selected_ap = {
  //   wps: false,
  // };
  let actionsOneGroupOpened = false;

  onMount(() => {
    _ip = localStorage.getItem("ip");
  });
  let ble_devices = [];
  const ws = new WebSocket("ws://" + _ip +  "/ble_scan");
  // const output = document.getElementById("output");
  // const input = document.getElementById("input");
  ws.onmessage = (event) => {
    ble_devices.push(JSON.parse(event.data));
    ble_devices = ble_devices;
    console.log(event.data);
    //{"name":"SMSL BT4.2","rssi":-128,"tx_power":-1,"icon":"audio-headphones"}
    // ws.send("napoli")
    // output.value += `\n${event.data}`;
  };

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
        onClick={() => {
        //   selected_ap = _ap;
        //   actionsOneGroupOpened = true;
        }}
      ></ListItem>
    {/each}
    <Button color="red" fill on:click={stop_scan}>Stop</Button>
  </List>

  <!-- One Group
    <Actions bind:opened={actionsOneGroupOpened}>
      <ActionsGroup>
        <ActionsLabel>Perform an attack</ActionsLabel>
        <ActionsButton>Sniff</ActionsButton>
        <ActionsButton>Replay</ActionsButton>
        <ActionsButton>DoS</ActionsButton>
        {#if selected_ap.wps}
          <ActionsButton>WPS Attack</ActionsButton>
        {/if}
        <ActionsButton color="red">Cancel</ActionsButton>
      </ActionsGroup>
    </Actions> -->
</Page>
