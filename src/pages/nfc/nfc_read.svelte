<script>
  import {
    Page,
    Navbar,
    NavTitle,
    Button,
    Block,
  } from "framework7-svelte";
  import { onMount } from "svelte";
  import { nfc_data } from "../../js/store.js";
  import { Terminal } from "@xterm/xterm";
  import { FitAddon } from '@xterm/addon-fit';
  import { saveAs } from "file-saver";
  import "@xterm/xterm/css/xterm.css";

  let _ip = "";
  let terminal;

  onMount(async () => {
    _ip = localStorage.getItem("ip");
    terminal = new Terminal();
    const fitAddon = new FitAddon();
    terminal.loadAddon(fitAddon);
    terminal.open(document.getElementById("terminal"));
    terminal.write($nfc_data.stdout);
    fitAddon.fit();
  });

  function save_data() {
    const arrUint8 = new Uint8Array($nfc_data.result);

    const blob = new Blob([arrUint8.buffer], {
      type: "application/octet-stream",
    });
    saveAs(blob, $nfc_data.filename.replace("~/nfc_captures/", ""));
  }
</script>

<Page name="home">
  <!-- Top Navbar -->
  <Navbar sliding={false} backLink="Back">
    <NavTitle sliding>CapibaraZero</NavTitle>
  </Navbar>

  <div id="terminal"></div>

  <Block>
    <Button fill on:click={save_data}>Download data</Button>
  </Block>
</Page>
