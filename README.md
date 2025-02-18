# CapibaraZero

CapibaraZero aim to be a cheap alternative to FlipperZero™. It was originally based on ESP32 boards especially on ESP32-S3 but now supports also SBC(linux) boards.

The installation script is tested only on Raspberry PI OS Lite running on a Raspberry PI Zero 2W

## Features

- WiFi support
  - Scan
  - Sniffer
  - EvilPortal
  - Beacon Spam
- BLE support
  - Scan
  - AppleJuice
  - Fast pair spam(Android devices)
  - Swift pair spam(Windows devices)
- ~~BadUSB~~(not ready)
- SubGHZ
  - SDR receiver support
  - ~~CC1101~~(not ready)
- NFC
  - Poll ISO14443A/B card
  - Read MIFARE Classic and Ultralight
  - Write MIFARE Classic and Ultralight(using .mfd dump format)
  - MIFARE nested attack
- IR
  - Send IR codes(using FlipperZero's file format)
  - Receive IR codes(will be saved in RAW FlipperZero's decoding)
- Network attacks
  - ARP Scanner
  - DHCP starvation and DoS
  - Sniffer
- Bettercap support(WiFi, BLE and Layer 2/3 attacks)

## Installation

Check [INSTALLATION.md](https://github.com/CapibaraZero/fw_linux/blob/main/INSTALLATION.md)

## Build from source

To build from source, check [build.sh](https://github.com/CapibaraZero/fw_linux/blob/main/build.sh). It's tested on the latest Debian 12 container

## Update to latest release

If you wanna update capibaraZero installed on your SBC to latest version you can perform the following command:

```bash
curl https://github.com/CapibaraZero/fw_linux/raw/refs/heads/main/update.sh | sh
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

## Social

- [Matrix server](https://matrix.to/#/#capibarazero:capibarazero.com)
- [Discord-Matrix bridge](https://discord.gg/77f3BHvnhf)

## License

[GPL-3](https://www.gnu.org/licenses/gpl-3.0.html)
<!-- TODO: 
- Custom key NFC(you must edit dict in /home/capibarazero atm)
- Fix command execute from query string api(AVOID SH -C)
- Create global function to do pkill -9 PROCESS_NAME
-->