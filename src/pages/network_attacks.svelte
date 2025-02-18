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
  } from "framework7-svelte";
  import { devices, current_net_interface } from "../js/store.js";
  // Router component will receive f7router prop with current Router instance
  export let f7router;
  // Router component will receive f7route prop with current route data
  export let f7route;
  import { onMount } from "svelte";
  import { CapacitorHttp } from "@capacitor/core";
  let _ip = "";
  let selected_interface = "wlan0";

  async function start_arp_scan() {
    if (selected_interface == "") {
      f7.dialog.alert("Please, select an interface");
    } else {
      f7.dialog.progress();
      const options = {
        url: "http://" + _ip + "/arp_scan?interface=" + selected_interface,
      };

      try {
        const response = await CapacitorHttp.get(options);
        console.log(response);
        $devices = JSON.parse(response.data);
        f7.dialog.close();
        f7router.navigate("/arp_scanner/");
      } catch (error) {
        f7.dialog.close();
        f7.dialog.alert(
          "Can't find any device. For further information, check server logs"
        );
      }
    }
  }

  let interfaces = [];
  async function get_interfaces() {
    const options = {
      url: "http://" + _ip + "/get_interfaces",
    };

    const response = await CapacitorHttp.get(options);
    interfaces = JSON.parse(response.data);
    console.log(interfaces);
    $current_net_interface = interfaces.filter(
      (val) => val.name == selected_interface
    )[0];
    console.log($current_net_interface);
    if ($current_net_interface != null)
      selected_interface = $current_net_interface.name; // Fallback if selected_interface doesn't exists
    else {
      selected_interface = "";
    }
  }
  onMount(() => {
    _ip = localStorage.getItem("ip");
    get_interfaces();
  });

  function change_net_interface() {
    $current_net_interface = interfaces.filter(
      (val) => val.name == selected_interface
    )[0];
    console.log($current_net_interface)
  }

  function start_dhcp_starvation() {
    if (selected_interface == "") {
      f7.dialog.alert("Please, select an interface");
    } else {
      f7router.navigate("/dhcp_starvation/");
    }
  }
</script>

<Page name="home">
  <!-- Top Navbar -->
  <Navbar sliding={false}>
    <NavTitle sliding>CapibaraZero</NavTitle>
  </Navbar>

  <!-- Page content -->
  <Block>
    <p>Network Attacks section of capibaraZero</p>
  </Block>

  <List strongIos outlineIos dividersIos>
    <ListItem
      title="Select an interface: {selected_interface}"
      smartSelect
      smartSelectParams={{ openIn: "sheet", setValueText: false }}
    >
      <select
        name="net_attacks_interface"
        bind:value={selected_interface}
        on:change={change_net_interface}
      >
        {#each interfaces as _interface}
          <option value={_interface.name}>{_interface.name}</option>
        {/each}
      </select>
    </ListItem>
  </List>
  <List inset strong>
    <ListButton title="ARP Scanner" on:click={start_arp_scan} />
    <ListButton title="DHCP starvation" on:click={start_dhcp_starvation} />
    <ListButton
      title="Get network details"
      on:click={() => f7router.navigate("/networks_details/")}
    />
    <ListButton title="Sniffer" on:click={() => f7router.navigate("/sniffer/")} />
  </List>
</Page>
