export function mode2_to_flipper(mode2_file) {
  const lines = [
    "Filetype: IR signals file",
    "Version: 1",
    "#",
    "# Dumped with CapibaraZero",
    "#",
    "name: capibarazero_dump",
    "type: raw",
    "frequency: 38000",
    "duty_cycle: 0.33",
    `data: ${mode2_file
      .replaceAll("pulse ", "")
      .replaceAll("space ", "")
      .replaceAll("\n", " ")}`,
  ];
  return lines.join("\n");
}
