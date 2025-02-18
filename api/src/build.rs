extern crate cc;

fn main() {
    // YOUR_PROJECT_PATH = $(pwd) command from the directory
    println!("cargo:rustc-link-search=YOUR_PROJECT_PATH/api/src/nfc");
    println!("cargo:rustc-link-lib=nfc_poll");
    println!("cargo:rustc-link-lib=cjson");
    println!("cargo:rustc-link-lib=usb");
    println!("cargo:rustc-link-lib=nfc");
     //cc::Build::new().file("src/nfc/nfc_poll.c").flag("-lcjson").compile("nfc_poll"); // For x64 build
}
