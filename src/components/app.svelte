<script>
  import { onMount } from "svelte";
  import { getDevice } from "framework7/lite-bundle";
  import {
    f7,
    f7ready,
    App,
    Panel,
    Views,
    View,
    Popup,
    Page,
    Navbar,
    Toolbar,
    NavRight,
    Link,
    Block,
    BlockTitle,
    LoginScreen,
    LoginScreenTitle,
    List,
    ListItem,
    ListInput,
    ListButton,
    BlockFooter,
  } from "framework7-svelte";

  import capacitorApp from "../js/capacitor-app";
  import routes from "../js/routes";

  const device = getDevice();
  // Framework7 Parameters
  let f7params = {
    name: "CapibaraZero", // App name
    theme: "auto", // Automatic theme detection
    colors: {
      primary: "#8e7559",
    },
    darkMode: true,
    
    // App routes
    routes: routes,

    // Register service worker (only on production build)
    serviceWorker:
      process.env.NODE_ENV === "production"
        ? {
            path: "/service-worker.js",
          }
        : {},
    // Input settings
    input: {
      scrollIntoViewOnFocus: device.capacitor,
      scrollIntoViewCentered: device.capacitor,
    },
    // Capacitor Statusbar settings
    statusbar: {
      iosOverlaysWebView: true,
      androidOverlaysWebView: false,
    },
  };
  onMount(() => {
    f7ready(() => {
      // Init capacitor APIs (see capacitor-app.js)
      if (f7.device.capacitor) {
        capacitorApp.init(f7);
      }
      // Call F7 APIs here
    });
  });
</script>

<App {...f7params}>
  <!-- Left panel with cover effect-->
  <Panel left cover dark>
    <View>
      <Page>
        <Navbar title="Left Panel" />
        <Block>Left panel content goes here</Block>
      </Page>
    </View>
  </Panel>

  <!-- Right panel with reveal effect-->
  <Panel right reveal dark>
    <View>
      <Page>
        <Navbar title="Right Panel" />
        <Block>Right panel content goes here</Block>
      </Page>
    </View>
  </Panel>

  <!-- Views/Tabs container -->
  <Views tabs class="safe-areas">
    <!-- Tabbar for switching views-tabs -->
    <Toolbar tabbar icons bottom>
      <Link
        tabLink="#view-wifi"
        tabLinkActive
        iconIos="f7:wifi"
        iconMd="material:wifi"
        text="Wi-Fi"
      />
      <Link
        tabLink="#view-ble"
        iconIos="f7:bluetooth"
        iconMd="material:bluetooth"
        text="BLE"
      />
      <Link
        tabLink="#view-badusb"
        iconIos="f7:usb"
        iconMd="material:usb"
        text="BadUSB"
      />
      <Link
        tabLink="#view-subghz"
        iconIos="f7:radio"
        iconMd="material:radio"
        text="SubGHZ"
      />
      <Link
        tabLink="#view-nfc"
        iconIos="f7:nfc"
        iconMd="material:nfc"
        text="NFC"
      />
      <Link
        tabLink="#view-ir"
        iconIos="material:settings_remote"
        iconMd="material:settings_remote"
        text="IR"
      />
      <Link
        tabLink="#view-net_at"
        iconIos="material:settings_ethernet"
        iconMd="material:settings_ethernet"
        text="Net.At"
      />
    </Toolbar>

    <!-- Your main view/tab, should have "view-main" class. It also has "tabActive" prop -->
    <View id="view-wifi" main tab tabActive url="/" />

    <!-- ble View -->
    <View id="view-ble" name="ble" tab url="/ble/" />

    <!-- Settings View -->
    <View id="view-badusb" name="badusb" tab url="/badusb/" />

    <!-- Settings View -->
    <View id="view-subghz" name="subghz" tab url="/subghz/" />

    <!-- Settings View -->
    <View id="view-nfc" name="nfc" tab url="/nfc/" />

        <!-- Settings View -->
        <View id="view-ir" name="ir" tab url="/ir/" />

    <!-- Settings View -->
    <View id="view-net_at" name="net_at" tab url="/network_attacks/" />
  </Views>

  <!-- Popup -->
  <Popup id="my-popup">
    <View>
      <Page>
        <Navbar title="Popup">
          <NavRight>
            <Link popupClose>Close</Link>
          </NavRight>
        </Navbar>
        <Block>
          <p>Popup content goes here.</p>
        </Block>
      </Page>
    </View>
  </Popup>
</App>
