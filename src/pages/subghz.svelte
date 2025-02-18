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
  import { onMount } from 'svelte';
  let _ip = "";
        // Router component will receive f7router prop with current Router instance
        export let f7router;

  onMount(() => {
    _ip = localStorage.getItem("ip");
  });

  function open_openwebrx() {
    window.open(`http://${_ip.split(":")[0]}:8073`, "_blank");
  }
</script>

<Page name="home">
  <!-- Top Navbar -->
  <Navbar sliding={false}>
    <NavTitle sliding>CapibaraZero</NavTitle>
    <!-- <NavTitleLarge>CapibaraZero</NavTitleLarge> -->
  </Navbar>

  <!-- Page content -->
  <Block>
    <p>SubGHZ section of capibaraZero</p>
  </Block>

  <List inset strong>
    <ListButton
      title="Launch OpenWebRX(SDR)"
      on:click={open_openwebrx}
    />
  </List>
</Page>
