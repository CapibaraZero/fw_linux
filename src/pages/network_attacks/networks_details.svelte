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
  let _ip = "";
  let response = [];
  let interfaces = [];
  onMount(async () => {
    _ip = localStorage.getItem("ip");
    let options = {
      url: "http://" + _ip + "/get_routes",
    };
    console.log(options);
    try {
      const res = await CapacitorHttp.get(options);
      response = res.data.split("\n");
      console.log(res.data);

      options = {
        url: "http://" + _ip + "/get_interfaces",
      };

      const if_resp = await CapacitorHttp.get(options);
      interfaces = JSON.parse(if_resp.data);
      console.log(interfaces);
      // sent_packets++;
    } catch (error) {
      console.log(error);
      // clearInterval(arp_task);
      f7.dialog.alert("Error. Can't get networks details");
    }
  });
</script>

<Page name="home">
  <!-- Top Navbar -->
  <Navbar sliding={false} backLink="Back">
    <NavTitle sliding>CapibaraZero</NavTitle>
  </Navbar>

  <Block>
    <h1>Interfaces:</h1>
    <List dividersIos mediaList outlineIos strongIos>
      {#each interfaces as _interface}
        <ListItem
          title={_interface.name}
          subtitle={_interface.mac}
          text={_interface.ips.join(" ")}
        ></ListItem>
      {/each}
    </List>
  </Block>

  <Block>
    <h1>Routing table:</h1>
    {#each response as line}
      <p>{line}</p>
    {/each}
  </Block>
</Page>
