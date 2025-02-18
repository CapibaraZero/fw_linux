
import HomePage from '../pages/home.svelte';
import WiFiScanResultPage from '../pages/wifi/scan_result.svelte';
import WiFiEvilPortalPage from '../pages/wifi/evil_portal.svelte';
import WiFiDoSPage from '../pages/wifi/wifi_dos.svelte';
import WiFiBeaconSpamPage from '../pages/wifi/beacon_spam.svelte';
import WiFiWPSBruteforcePage from '../pages/wifi/wps_bruteforce.svelte';

import BLEPage from '../pages/ble.svelte';
import BLEScanPage from '../pages/ble/ble_scan.svelte';
import BLESpamPage from '../pages/ble/ble_spam.svelte';

import NFCPage from '../pages/nfc.svelte';
import NFCDetailsPage from '../pages/nfc/nfc_tag_details.svelte';
import NFCReadPage from '../pages/nfc/nfc_read.svelte';
import NFCNestedAttack from '../pages/nfc/nfc_nested_attack.svelte';

import BCPage from '../pages/network_attacks/bc.svelte';
import NetworkAttacksPage from '../pages/network_attacks.svelte';
import ARPScannerPage from '../pages/network_attacks/arp_scanner.svelte';
import DHCPStarvationPage from '../pages/network_attacks/dhcp_starvation.svelte';
import NetworksDetailsPage from '../pages/network_attacks/networks_details.svelte';
import SnifferPage from '../pages/network_attacks/sniffer.svelte';

import SubGHZPage from '../pages/subghz.svelte';

import IRPage from '../pages/ir.svelte';
import IRRCPage from '../pages/ir/ir_rc.svelte';

import NotFoundPage from '../pages/404.svelte';

var routes = [
  {
    path: '/',
    component: HomePage,
  },
  {
    path: '/wifi_scan_result/',
    component: WiFiScanResultPage
  },
  {
    path: '/evil_portal/',
    component: WiFiEvilPortalPage
  },
  {
    path: '/wifi_dos/:channel/:bssid/',
    component: WiFiDoSPage
  },
  {
    path: '/wifi_beacon_spam/',
    component: WiFiBeaconSpamPage
  },
  {
    path: '/wifi_wps_bruteforce/:bssid/:pixiedust/',
    component: WiFiWPSBruteforcePage
  },
  {
    path: '/ble_scan_page/',
    component: BLEScanPage
  },
  {
    path: '/ble_spam_page/:type/',
    component: BLESpamPage
  },
  {
    path: '/ble/',
    component: BLEPage
  },
  {
    path: '/nfc/',
    component: NFCPage
  },
  {
    path: '/nfc_details/',
    component: NFCDetailsPage
  },
  {
    path: '/nfc_read/',
    component: NFCReadPage
  },
  {
    path: '/nfc_nested_attack/',
    component: NFCNestedAttack
  },
  {
    path: '/network_attacks/',
    component: NetworkAttacksPage
  },
  {
    path: '/arp_scanner/',
    component: ARPScannerPage
  },
  {
    path: '/dhcp_starvation/',
    component: DHCPStarvationPage
  },
  {
    path: '/networks_details/',
    component: NetworksDetailsPage
  },
  {
    path: '/bc/',
    component: BCPage
  },
  {
    path: '/sniffer/',
    component: SnifferPage
  },
  {
    path: '/subghz/',
    component: SubGHZPage
  },
  {
    path: '/ir/',
    component: IRPage
  },
  {
    path: '/ir_rc/',
    component: IRRCPage
  },
  {
    path: '(.*)',
    component: NotFoundPage,
  },
];

export default routes;
