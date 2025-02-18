export function parse_parsed_signal(_signal, lines) {
  for (let index = 0; index < lines.length; index++) {
    const line = lines[index];
    if (line.includes("protocol")) {
      let flipper_protocol = line.split(": ")[1];
      // Missing protocols on Linux: NEC42, Samsung32, Kaseikyo, RCA
      if (flipper_protocol == "NEC" || flipper_protocol == "Pioneer") {
        _signal.protocol = "nec";
      } else if (flipper_protocol == "NECext") {
        _signal.protocol = "necx";
      } else if (flipper_protocol == "RC6") {
        _signal.protocol = "rc6_0";
      } else if (flipper_protocol == "RC5") {
        _signal.protocol = "rc5";
      } else if (flipper_protocol == "RC5X") {
        _signal.protocol = "rc5x_20";
      } else if (flipper_protocol == "SIRC") {
        _signal.protocol = "sony12";
      } else if (flipper_protocol == "SIRC15") {
        _signal.protocol = "sony15";
      } else if (flipper_protocol == "SIRC20") {
        _signal.protocol = "sony20";
      }
    } else if (line.includes("address")) {
      let flipper_addr = line.split(": ")[1].split(" ");

      // Some protocols require shorter message. More info here: https://www.kernel.org/doc/html/latest/userspace-api/media/rc/rc-protos.html
      // https://github.com/flipperdevices/flipperzero-firmware/tree/dev/applications/debug/unit_tests/resources/unit_tests/infrared
      if (
        (_signal.protocol.includes("SIRC") && _signal.protocol != "SIRC20") ||
        _signal.protocol == "NEC" ||
        _signal.protocol == "Pioneer" ||
        _signal.protocol.includes("RC")
      ) {
        _signal.address = flipper_addr[0];
      } else {
        _signal.address = `${flipper_addr[0]}${flipper_addr[1]}`;
      }
    } else if (line.includes("command")) {
      let flipper_cmd = line.split(": ")[1].split(" ");
      if (
        (_signal.protocol.includes("SIRC") && _signal.protocol != "SIRC20") ||
        _signal.protocol == "NEC" ||
        _signal.protocol == "Pioneer" ||
        _signal.protocol.includes("RC")
      ) {
        _signal.command = flipper_cmd[0];
      } else {
        _signal.command = `${flipper_cmd[0]}${flipper_cmd[1]}`;
      }
      return _signal;
    }
  }
  return _signal;
}

export function parse_raw_signal(lines, _signal) {
    _signal.raw_data = "";
    _signal.address = "";
    _signal.command = "";
    _signal.protocol = "";
    for (let index = 0; index < lines.length; index++) {
        const line = lines[index];
        if(line.includes("data: ")) {
            let raw_data = line.split(": ")[1].split(" ");
            raw_data.forEach((x, i) => {
                if(i%2 == 0) {
                    _signal.raw_data += `pulse ${x}\n`;
                } else {
                    _signal.raw_data += `space ${x}\n`;
                }
            }); 
            return _signal;
        } else if(line.includes("duty_cycle: ")) {
          _signal.duty_cycle = (+line.split(": ")[1]) * 100;  // Convert it in percent for LIRC usage
        } else if(line.includes("frequency: ")) {
          _signal.frequency = +line.split(": ")[1];
        }
    }
}