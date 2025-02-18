extern crate cc;

fn main() {
    println!("cargo:rustc-link-search=/home/andreock/work/capibara_rpi/new_capibarazero/api/src/nfc");
    println!("cargo:rustc-link-lib=nfc_poll");
    println!("cargo:rustc-link-lib=cjson");
    println!("cargo:rustc-link-lib=usb");
    println!("cargo:rustc-link-lib=nfc");
    // cc::Build::new().file("src/nfc/nfc_poll.c").static_crt(true).compile("nfc_poll");
}
