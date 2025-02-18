<script>
  import {
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
    f7,
  } from "framework7-svelte";

  import { onMount } from "svelte";
  import { CapacitorHttp } from "@capacitor/core";
  import { ir_buttons } from "../js/store.js";
  import { parse_parsed_signal, parse_raw_signal } from "../js/ir_parser.js";
  import { mode2_to_flipper } from "../js/ir_writer.js";

  let _ip = "";

  // Router component will receive f7router prop with current Router instance
  export let f7router;

  let fileInput;

  let signal = {
    raw_data: "",
    frequency: 38000,
    duty_cycle: 33,
  };
  let signals = [];

  async function send_ir_data_handler(evt) {
    const dump = evt.target.files[0];
    console.log(dump);
    const reader = new FileReader(); // create object

    reader.onload = function (e) {
      const text_data = e.target.result;
      const lines = text_data.split("\n");

      for (let index = 0; index < lines.length; index++) {
        const line = lines[index];
        //   console.log(line);
        if (line.includes("name")) {
          signal.btn_name = line.split(": ")[1];
        } else if (line == "type: parsed") {
          // Orrible workaround to push the actual object in array
          signals.push(
            JSON.parse(
              JSON.stringify(parse_parsed_signal(signal, lines.slice(index)))
            )
          );
          signal.btn_name = "";
          signal.address = "";
          signal.command = "";
          signal.protocol = "";
          signal.raw_data = "";
          index += 4; // Optimize the cycle jumping at the next button
        } else if (line == "type: raw") {
          console.log("raw");
          signals.push(
            JSON.parse(
              JSON.stringify(parse_raw_signal(lines.slice(index), signal))
            )
          );
          signal.btn_name = "";
          signal.address = "";
          signal.command = "";
          signal.protocol = "";
          signal.raw_data = "";
          index += 4; // Optimize the cycle jumping at the next button
        }
      }
      $ir_buttons = signals;
      console.log(signals);
      f7router.navigate("/ir_rc/");
      // console.log();
      //   output.textContent = e.target.result; // Display the file content
    };

    reader.readAsText(dump); // Read the file as text
  }

  onMount(() => {
    _ip = localStorage.getItem("ip");
  });

  async function read_signal() {
    f7.dialog.preloader("Waiting for signal...");
    const options = {
      url: "http://" + _ip + "/ir_receive",
    };

    const response = await CapacitorHttp.get(options);
    console.log(response);
    f7.dialog.close();

    let today = new Date();
    let dd = String(today.getDate()).padStart(2, "0");
    let mm = String(today.getMonth() + 1).padStart(2, "0"); //January is 0!
    let yyyy = today.getFullYear();

    today = mm + "/" + dd + "/" + yyyy;
    console.log(today);
    
    const blob = new Blob([mode2_to_flipper(response.data)], {
      type: "text/plain;charset=utf-8",
    });

    saveAs(blob, `${dd}_${mm}_${yyyy}.ir`);
  }
</script>

<Page name="home">
  <!-- Top Navbar -->
  <Navbar sliding={false}>
    <NavTitle sliding>CapibaraZero</NavTitle>
    <!-- <NavTitleLarge>CapibaraZero</NavTitleLarge> -->
  </Navbar>
  <input
    type="file"
    bind:this={fileInput}
    style="display: none"
    on:input={send_ir_data_handler}
  />
  <!-- Page content -->
  <Block>
    <p>IR section of capibaraZero</p>
  </Block>

  <List inset strong>
    <ListButton title="Send" on:click={() => fileInput.click()} />
    <ListButton title="Read" on:click={read_signal} />
  </List>
</Page>
