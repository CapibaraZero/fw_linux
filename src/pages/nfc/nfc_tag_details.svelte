<script>
  import {
    Page,
    Navbar,
    NavTitle,
    Block,
    List,
    ListButton,
    f7,
  } from "framework7-svelte";
  import { nfc_data, nfc_tag } from "../../js/store";
  export let f7router;
  console.log($nfc_tag);
  import { saveAs } from "file-saver";
  import { onMount } from "svelte";
  import { CapacitorHttp } from "@capacitor/core";

  let _ip = "";
  onMount(() => {
    _ip = localStorage.getItem("ip");
  });
  function download_polled_tag() {
    let today = new Date();
    let dd = String(today.getDate()).padStart(2, "0");
    let mm = String(today.getMonth() + 1).padStart(2, "0"); //January is 0!
    let yyyy = today.getFullYear();

    today = mm + "/" + dd + "/" + yyyy;
    console.log(today);
    const blob = new Blob([JSON.stringify($nfc_tag)], {
      type: "text/plain;charset=utf-8",
    });

    saveAs(
      blob,
      `${dd}_${mm}_${yyyy}_${$nfc_tag.uid
        .map((bit) => bit.toString(16))
        .join("_")
        .toUpperCase()}.json`
    );
  }

  async function nfc_data_request(key_type, ultralight) {
    f7.dialog.preloader();

    const options = {
      // url: "http://" + _ip + "/nfc_read?key_type=".concat(key_type),
      url: `http://${_ip}/nfc_read?key_type=${key_type}&ultralight=${ultralight}`,
    };

    const response = await CapacitorHttp.get(options);
    console.log(response);
    $nfc_data = JSON.parse(response.data);
    f7.dialog.close();
    f7router.navigate("/nfc_read/");
  }

  let fileInput;

  async function read_nfc_data() {
    if ($nfc_tag.atqa.join("") == "04" && $nfc_tag.sak == 8) {
      f7.dialog
        .create({
          title: "Which key do you wanna use?",
          buttons: [
            {
              text: "Key A",
              onClick: async function () {
                await nfc_data_request("A", false);
              },
            },
            {
              text: "Key B",
              onClick: async function () {
                await nfc_data_request("B", false);
              },
            },
          ],
          verticalButtons: true,
        })
        .open();
    } else {
      console.log("Ultralight");
      await nfc_data_request("A", true);
      // Read ultralight
    }
  }

  function write_nfc_tag() {
    f7.dialog.alert(
      "Please select the file of the dump that you wanna write on the tag",
      () => {
        fileInput.click();
      }
    );
  }

  async function write_nfc_request(formdata) {
    f7.dialog.preloader();
    const options = {
      url: "http://" + _ip + "/nfc_write",
      headers: { "Content-Type": "multipart/form-data" },
      data: formdata,
    };
    console.log(options);
    const res = await CapacitorHttp.post(options);
    f7.dialog.close();
    f7.dialog.alert(res.data);
    console.log(res);
  }

  async function write_nfc_handle_file(evt) {
    const dump = evt.target.files[0];
    console.log(dump);
    const formData = new FormData();
    formData.append("tag", dump);
    if ($nfc_tag.atqa.join("") == "04" && $nfc_tag.sak == 8) {
      f7.dialog
        .create({
          title: "Which key do you wanna use?",
          buttons: [
            {
              text: "Key A",
              onClick: async function () {
                formData.append("key", "A");
                await write_nfc_request(formData);
              },
            },
            {
              text: "Key B",
              onClick: async function () {
                formData.append("key", "B");
                await write_nfc_request(formData);
              },
            },
          ],
          verticalButtons: true,
        })
        .open();
    } else {
      formData.append("ultralight", "true");
      f7.dialog.confirm(
        "Do you wanna override UID?",
        async () => {
          formData.append("uid_override", "true");
          await write_nfc_request(formData);
        },
        async () => {
          await write_nfc_request(formData);
        }
      );
    }
  }
</script>

<Page name="home">
  <!-- Top Navbar -->
  <Navbar sliding={false} backLink="Back">
    <NavTitle sliding>CapibaraZero</NavTitle>
  </Navbar>

  <input
    type="file"
    bind:this={fileInput}
    style="display: none"
    on:input={write_nfc_handle_file}
  />
  <!-- <button on:click={fileInput.click()}>Open File...</button> -->

  <Block>
    <h2>Card polled successfully!</h2>
    <div class="grid grid-rows-5">
      <h4>Type: {$nfc_tag.type}</h4>
      {#if $nfc_tag.type == "ISO14443A"}
        <h4>ATQA: {$nfc_tag.atqa[0]}{$nfc_tag.atqa[1]}</h4>
        <h4>SAK: {$nfc_tag.sak}</h4>
        <h4>
          UID: {$nfc_tag.uid
            .map((bit) => bit.toString(16))
            .join(" ")
            .toUpperCase()}
        </h4>
        {#if $nfc_tag.atqa.join("") == "04" && $nfc_tag.sak == 8}
          <h4>Card model: MIFARE Classic</h4>
        {:else if $nfc_tag.uid.length == 7}
          <h4>Card model(estimated): MIFARE Ultralight/NTAG</h4>
        {/if}
      {:else if $nfc_tag.type == "FeliCa"}
        <h4>
          Sys codes: {$nfc_tag.sys_code
            .map((bit) => bit.toString(16))
            .join(" ")
            .toUpperCase()}
        </h4>
        <h4>
          PAD: {$nfc_tag.pad
            .map((bit) => bit.toString(16))
            .join(" ")
            .toUpperCase()}{$nfc_tag.sak}
        </h4>
        <h4>
          UID: {$nfc_tag.uid
            .map((bit) => bit.toString(16))
            .join(" ")
            .toUpperCase()}
        </h4>
      {/if}
    </div>
  </Block>
  <List inset strong>
    <ListButton title="Download polled data" on:click={download_polled_tag} />
    {#if $nfc_tag.type == "ISO14443A"}
      <ListButton title="Read Tag" on:click={read_nfc_data} />
      <ListButton title="Write Tag" on:click={write_nfc_tag} />
      <!-- <ListButton title="Darkside attack" /> -->
      <!-- <ListButton title="Hardened attack" /> -->
    {/if}
    {#if $nfc_tag.atqa.join("") == "04" && $nfc_tag.sak == 8}
      <ListButton
        title="Nested attack"
        on:click={() => f7router.navigate("/nfc_nested_attack/")}
      />
    {/if}
  </List>
</Page>
