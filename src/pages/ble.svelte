<Page name="home">
    <!-- Top Navbar -->
    <Navbar sliding={false}>
      <NavTitle sliding>CapibaraZero</NavTitle>
      <!-- <NavTitleLarge>CapibaraZero</NavTitleLarge> -->
    </Navbar>
  
    <!-- Page content -->
    <Block>
      <p>BLE section of capibaraZero</p>
    </Block>
  
    <List inset strong>
      <ListButton title="Scan" on:click={() => f7router.navigate("/ble_scan_page/")}/>
      <!-- <ListButton title="Sniff" /> -->
      <ListButton title="AppleJuice" on:click={() => f7router.navigate("/ble_spam_page/applejuice/")}/>
      <!-- <ListButton title="Samsung BLE Spam"/> -->
      <ListButton title="Fast Pair Spam"  on:click={() => f7router.navigate("/ble_spam_page/fastpair/")}/>
      <ListButton title="Swift Pair Spam" on:click={() => f7router.navigate("/ble_spam_page/swiftpair/")}/>
      <ListButton title="Launch Bettercap" on:click={launch_bettercap} />
    </List>
  </Page>
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
      Button
    } from 'framework7-svelte';
        // Router component will receive f7router prop with current Router instance
        export let f7router;
  // Router component will receive f7route prop with current route data
  export let f7route;

  function launch_bettercap() {
    console.log("Launch bettercap");
    // console.log(f7router);
    // f7.views.main.router.navigate("/bettercap_launcher/")
    f7router.navigate("/bc/");
    // f7router.navigate("/wifi_scan_result/");
  }
  </script>