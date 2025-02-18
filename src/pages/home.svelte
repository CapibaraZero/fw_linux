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
  import { onMount } from "svelte";

  // Router component will receive f7router prop with current Router instance
  export let f7router;
  let _ip = "";
  import { CapacitorHttp } from "@capacitor/core";
  import { ap, current_net_interface, hotspot_uuid } from "../js/store.js";
  import { handle_custom_evil_portal, start_evilportal } from '../js/wifi/evilportal.js';

  async function scan_wifi() {
    f7.dialog.progress();
    const options = {
      url: "http://" + _ip + "/wifi_scan?interface=".concat(selected_interface),
    };

    const response = await CapacitorHttp.get(options);
    console.log(response);
    if (response.data == "") {
      f7.dialog.close();
      f7.dialog.alert(
        "Can't scan for wireless networks. Error: " + response.data
      );
    } else {
      f7.dialog.close();
      try {
        $ap = JSON.parse(response.data);
        // setTimeout(() => {
        f7router.navigate("/wifi_scan_result/");
      } catch (error) {
        f7.dialog.alert(
          "Can't scan for wireless networks. Error: " + response.data
        );
      }
    }
  }

  function add_server() {
    f7.dialog.prompt(
      "Set API ip in form ip:port for example: 192.168.1.11:3000",
      (ip) => {
        localStorage.setItem("ip", ip);
        _ip = ip;
        console.log(ip);
        location.reload();
      }
    );
  }

  async function get_interfaces(select_new_if = true) {
    const options = {
      url: "http://" + _ip + "/get_interfaces",
    };

    const response = await CapacitorHttp.get(options);
    interfaces = JSON.parse(response.data);
    console.log(interfaces);
    if (select_new_if && $current_net_interface == "") {
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
  }

  let interfaces = [];
  onMount(() => {
    _ip = localStorage.getItem("ip");
    get_interfaces();
  });

  let selected_interface = "wlan0";
  function change_net_interface() {
    $current_net_interface = interfaces.filter(
      (val) => val.name == selected_interface
    )[0];
  }

  function launch_bettercap() {
    console.log("Launch bettercap");
    if ($current_net_interface.ips.length == 0) {
      f7.dialog.alert("This interface doesn't have IP. Can't start bettercap");
    } else {
      f7router.navigate("/bc/");
    }
  }

  async function enable_monitor_mode(show_alert = true) {
    return new Promise((resolve, reject) => {
      if (selected_interface == "") {
        f7.dialog.alert("Please, select an interface");
        resolve();
      } else {
        f7.dialog.confirm(
          "Do you wanna enable monitor mode on " + selected_interface,
          async () => {
            f7.dialog.preloader();
            // f7.dialog.alert('Great!');
            const options = {
              url:
                "http://" +
                _ip +
                "/enable_monitor_mode?interface=" +
                selected_interface,
            };

            const response = await CapacitorHttp.get(options);
            f7.dialog.close();
            if (show_alert) {
              // interfaces = JSON.parse(response.data);
              f7.dialog.alert(response.data.stdout, () => {
                f7.dialog.alert(
                  "If airmon worked, you should be able to select new interface from home. It's usually called (interface)mon"
                );
              });
            }
            console.log(response);
            await get_interfaces(show_alert);
            resolve();
          }
        );
      }
    });
  }

  function disable_monitor_mode() {
    if (selected_interface == "") {
      f7.dialog.alert("Please, select an interface");
    } else {
      f7.dialog.confirm(
        "Do you wanna disable monitor mode on " + selected_interface,
        async () => {
          f7.dialog.preloader();
          // f7.dialog.alert('Great!');
          const options = {
            url:
              "http://" +
              _ip +
              "/disable_monitor_mode?interface=" +
              selected_interface,
          };

          const response = await CapacitorHttp.get(options);
          f7.dialog.close();
          // interfaces = JSON.parse(response.data);
          f7.dialog.alert(response.data.stdout);
          console.log(response);
          get_interfaces();
        }
      );
    }
  }

  async function wifi_sniffer() {
    await enable_monitor_mode(false);
    selected_interface += "mon";
    $current_net_interface.name = selected_interface;
    f7router.navigate("/sniffer/");
  }

  async function beacon_spam() {
    if (selected_interface == "") {
      f7.dialog.alert("Please, select an interface");
    } else {
      f7router.navigate("/wifi_beacon_spam/");
    }
  }

  let fileInput;

</script>

<Page name="home">
  <!-- Top Navbar -->
  <Navbar sliding={false}>
    <!-- <NavLeft>
      <Link iconIos="f7:menu" iconMd="material:menu" panelOpen="left" />
    </NavLeft> -->
    <NavTitle sliding>CapibaraZero</NavTitle>
    <NavRight>
      <Link iconIos="f7:add" iconMd="material:add" on:click={add_server} />
    </NavRight>
    <!-- <NavTitleLarge>CapibaraZero</NavTitleLarge> -->
  </Navbar>
  <input
    type="file"
    bind:this={fileInput}
    style="display: none"
    accept=".zip"
    on:input={handle_custom_evil_portal}
  />
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
  <!-- Page content -->
  <Block>
    {#if _ip == null}
      <p>Please add IP to your SBC</p>
    {/if}
    <p>Wi-Fi section of capibaraZero</p>
  </Block>

  <List inset strong>
    <ListButton title="Scan" on:click={scan_wifi} />
    <ListButton title="Sniff" on:click={wifi_sniffer} />
    <ListButton title="Bettercap" on:click={launch_bettercap} />
    <ListButton title="EvilPortal" on:click={() => start_evilportal(fileInput, selected_interface, _ip, f7router)} />
    <ListButton title="Beacon Spam" on:click={beacon_spam} />
    <ListButton title="Enable Monitor Mode" on:click={enable_monitor_mode} />
    <ListButton title="Disable Monitor Mode" on:click={disable_monitor_mode} />
  </List>
</Page>
