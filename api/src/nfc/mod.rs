extern crate core;
use axum::extract::connect_info::ConnectInfo;
use axum::extract::Query;
use axum::{extract::ws::WebSocketUpgrade, extract::Multipart, response::IntoResponse};
use axum_extra::TypedHeader;
use chrono::{Datelike, Timelike};
use libc::c_char;
use rand::rngs::OsRng;
use rand::TryRngCore;
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
use std::fs;
use std::net::SocketAddr;
use std::path::Path;
use std::process::Command;

mod nested_attack;

#[link(name = "cjson")]
#[link(name = "nfc")]
#[link(name = "usb")]
extern "C" {
    fn nfc_poll_tag() -> *const c_char;
}

pub async fn nfc_poll() -> String {
    unsafe { CStr::from_ptr(nfc_poll_tag()).to_str().unwrap().to_string() }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct NFCKeyParams {
    key_type: Option<String>,
    ultralight: bool,
}

#[derive(Serialize, Deserialize)]
pub struct NFCReadResponse {
    stdout: String,
    filename: String,
    result: Vec<u8>,
}

pub async fn nfc_read(params: Query<NFCKeyParams>) -> String {
    let year = chrono::offset::Local::now().year();
    let month = chrono::offset::Local::now().month();
    let day = chrono::offset::Local::now().day();
    let hour = chrono::offset::Local::now().hour();
    let minutes = chrono::offset::Local::now().minute();
    let seconds = chrono::offset::Local::now().second();

    let path = std::format!(
        "~/nfc_captures/{}_{}_{}_{}_{}_{}.mfd",
        day,
        month,
        year,
        hour,
        minutes,
        seconds
    );
    let result = if !params.ultralight {
        Command::new("sh")
            .arg("-c")
            .arg(std::format!(
                "nfc-mfclassic r {} u {}",
                params.key_type.clone().expect("Missing key type params"),
                path.clone()
            ))
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(std::format!("nfc-mfultralight r {}", path.clone()))
            .output()
            .expect("failed to execute process")
    };

    let mut res = NFCReadResponse {
        stdout: String::from_utf8_lossy(&result.stdout).to_string(),
        filename: path.clone(),
        result: vec![],
    };
    if Path::new(path.as_str()).exists() {
        res.result = fs::read(path).expect("Can't read mfd");
    }
    serde_json::to_string(&res).expect("Can't serialize response")
}

pub async fn nested_attack_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    nested_attack::_nfc_nested_attack_handler(ws, user_agent, ConnectInfo(addr))
}

pub async fn nfc_write_data(mut multipart: Multipart) -> String {
    let mut path_name: String = String::new();
    let mut key: String = String::new();
    let mut ultralight: String = String::new();
    let mut uid_overwrite: String = String::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        if name == "tag" {
            let data = field.bytes().await.unwrap();
            let random_id = OsRng.try_next_u32().expect("Can't gen"); // Use OsRng since is thread-safe
            path_name = std::format!("/tmp/tag_{}.mfd", random_id);
            fs::write(path_name.as_str(), data).expect("Can't write nfc tag to be wrote");
        } else if name == "key" {
            key = field.text().await.unwrap();
        } else if name == "ultralight" {
            ultralight = field.text().await.unwrap();
        } else if name == "uid_overwrite" {
            if field.name().unwrap().to_string() == "true" {
                uid_overwrite = "--uid".to_string();
            }
        }
    }

    let res = if ultralight == "true" {
        Command::new("sh")
            .arg("-c")
            .arg(std::format!(
                "echo 'n' | nfc-mfultralight w {} --otp --lock --dynlock {}",
                path_name,
                uid_overwrite
            ))
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(std::format!(
                "nfc-mfclassic w {} u {}",
                key,
                path_name
            ))
            .output()
            .expect("failed to execute process")
    };

    let stdout = String::from_utf8_lossy(&res.stdout);

    fs::remove_file(path_name).expect("Can't remove nfc file");
    return stdout.into();
}
