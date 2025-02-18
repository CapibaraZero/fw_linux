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

  export let f7router;

  let _ip = "";
  import { CapacitorHttp } from "@capacitor/core";
  console.log($ap);

  let selected_ap = {
    wps: false,
  };
  let actionsOneGroupOpened = false;

  onMount(() => {
    _ip = localStorage.getItem("ip");
  });

  function connect_to_wifi_ap() {
    f7.dialog.password(
      "Enter password for " + selected_ap.ssid,
      async (password) => {
        f7.dialog.progress();
        const options = {
          url: "http://"+ _ip + "/wifi_connect",
          headers: { "content-type": "application/json"},
          data: { ssid: selected_ap.ssid.replace("SSID: ", ""), password: password},
        };
        try {
          await CapacitorHttp.post(options);
          f7.dialog.close();
          f7.dialog.alert("Connected successfully"); 
        } catch (error) {
          f7.dialog.close();
          f7.dialog.alert("Can't connect to AP"); 
        }
        // f7.dialog.alert(`Thank you!<br>Password:${password}`);
      }
    );
  }

  function start_wifi_dos() {
    f7router.navigate(`/wifi_dos/${selected_ap.primary_channel}/${selected_ap.bssid}/`);
  }

  function start_wps_bruteforce() {
    console.log(selected_ap);
    console.log(selected_ap.bssid);
    f7router.navigate(`/wifi_wps_bruteforce/${selected_ap.bssid}/${pixiedust}/`);
  }
</script>

<Page name="home">
  <!-- Top Navbar -->
  <Navbar sliding={false} backLink="Back">
    <NavTitle sliding>CapibaraZero</NavTitle>
  </Navbar>

  <List dividersIos mediaList outlineIos strongIos>
    {#each $ap as _ap}
      <ListItem
        link="#"
        title={_ap.ssid == "" ? "SSID: Hidden" : _ap.ssid}
        after={_ap.freq + "MHz"}
        subtitle={"Signal: " + _ap.signal_dbm + "dBm" + " Cipher: " +  _ap.pairwise_ciphers + " " + _ap.authentication_suites}
        text="WPS: {_ap.wps}"
        onClick={() => {
          selected_ap = _ap;
          actionsOneGroupOpened = true;
        }}
      ></ListItem>
    {/each}
  </List>

  <!-- One Group -->
  <Actions bind:opened={actionsOneGroupOpened}>
    <ActionsGroup>
      <ActionsLabel>Perform an action</ActionsLabel>
      <ActionsButton on:click={connect_to_wifi_ap}>Connect</ActionsButton>
      <!-- <ActionsButton>Sniff</ActionsButton> -->
      <!-- <ActionsButton>Replay</ActionsButton> -->
      <ActionsButton on:click={start_wifi_dos}>DoS</ActionsButton>
      {#if selected_ap.wps != null}
        <ActionsButton on:click={() => start_wps_bruteforce(false)}>WPS Bruteforce</ActionsButton>
        <ActionsButton on:click={() => start_wps_bruteforce(true)}>WPS Pixiedust</ActionsButton>
      {/if}
      <ActionsButton color="red">Cancel</ActionsButton>
    </ActionsGroup>
  </Actions>
</Page>
