import { writable } from 'svelte/store';

export const ap = writable([]);
export const current_net_interface = writable("");
export const devices = writable([]);
export const hotspot_uuid = writable("");
export const nfc_tag = writable({});
export const nfc_data = writable({});
export const ir_buttons = writable([]);