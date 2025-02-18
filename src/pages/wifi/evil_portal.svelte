<script>
  import {
    f7,
    Page,
    Navbar,
    NavTitle,
    Button,
    Block,
    ListButton,
    List
  } from "framework7-svelte";
  import { onMount } from "svelte";
  import { CapacitorHttp } from "@capacitor/core";
  import { hotspot_uuid } from "../../js/store.js";
  import { Terminal } from "@xterm/xterm";
  import { FitAddon } from "@xterm/addon-fit";

  export let f7router;
  export let f7route;

  let _ip = "";
  let terminal;
  let polling_task;

  console.log(f7route);

  onMount(() => {
    _ip = localStorage.getItem("ip");
    console.log($hotspot_uuid);
    terminal = new Terminal();
    const fitAddon = new FitAddon();
    terminal.loadAddon(fitAddon);
    terminal.open(document.getElementById("terminal"));
    fitAddon.fit();

    polling_task = setInterval(async function () {
      const options = {
        url: "http://" + _ip + "/read_evilportal_log",
      };
      const res = await CapacitorHttp.get(options);
      // console.log(res);
      terminal.clear();
      terminal.write(res.data);
    }, 2000);
  });

  async function stop_attack() {
    console.log("Stopping ap");
    clearInterval(polling_task);
    const options = {
      url: "http://" + _ip + "/stop_ap?uuid=".concat($hotspot_uuid),
    };
    try {
      const res = await CapacitorHttp.get(options);
      console.log(res);
    } catch (error) {
      console.error(error);
      f7.dialog.alert("Can't stop EvilPortal");
    }
    f7router.back();
  }

  async function download_log() {
    let today = new Date();
    let dd = String(today.getDate()).padStart(2, "0");
    let mm = String(today.getMonth() + 1).padStart(2, "0"); //January is 0!
    let yyyy = today.getFullYear();

    today = mm + "/" + dd + "/" + yyyy;
    console.log(today);
    const options = {
      url: "http://" + _ip + "/read_evilportal_log",
    };
    const res = await CapacitorHttp.get(options);
    const blob = new Blob([JSON.stringify(res.data)], {
      type: "text/plain;charset=utf-8",
    });

    saveAs(blob, `${dd}_${mm}_${yyyy}_evilportal_log.txt`);
  }
</script>

<Page name="home">
  <!-- Top Navbar -->
  <Navbar sliding={false} backLink="Back">
    <NavTitle sliding>CapibaraZero</NavTitle>
  </Navbar>

  <Block>
    <p>EvilPortal attack in progress...</p>
  </Block>
  <List inset strong>
    <ListButton on:click={download_log} fill title="Download the log"
    ></ListButton>

    <ListButton on:click={stop_attack} fill color="red" title="Stop attack"
    ></ListButton>
  </List>
  <div id="terminal"></div>
</Page>
