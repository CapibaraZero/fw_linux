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

      f7

    } from 'framework7-svelte';
  import { nfc_tag } from '../js/store';
  import { onMount } from 'svelte';
  import { CapacitorHttp } from "@capacitor/core";
  let _ip = "";
        // Router component will receive f7router prop with current Router instance
        export let f7router;
  // Router component will receive f7route prop with current route data
  export let f7route;

  async function poll_nfc_tag() {
    f7.dialog.preloader();

    const options = {
      url: "http://" + _ip + "/nfc_poll",
    };

    const response = await CapacitorHttp.get(options);
    $nfc_tag = JSON.parse(response.data);

    f7.dialog.close();
    f7router.navigate("/nfc_details/");
  }

  onMount(() => {
    _ip = localStorage.getItem("ip");
  });
</script>

<Page name="home">
  <!-- Top Navbar -->
  <Navbar sliding={false}>
    <NavTitle sliding>CapibaraZero</NavTitle>
    <!-- <NavTitleLarge>CapibaraZero</NavTitleLarge> -->
  </Navbar>

  <!-- Page content -->
  <Block>
    <p>NFC section of capibaraZero</p>
  </Block>

  <List inset strong>
    <ListButton
      title="Poll"
      on:click={poll_nfc_tag}
    />
  </List>
</Page>
