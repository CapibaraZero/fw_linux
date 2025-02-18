
//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;
use axum::{
    body::Bytes,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse
};
use axum_extra::TypedHeader;

use bluer::{Adapter, AdapterEvent, Address, DiscoveryFilter, DiscoveryTransport};
use futures::{SinkExt, StreamExt};
use std::{collections::HashSet, env};
use std::net::SocketAddr;
use serde::Serialize;
use serde::Deserialize;

mod fastpair;
mod apple;
mod swiftpair;

#[derive(Serialize, Deserialize)]
struct BLEDevice {
    name: String,
    rssi: i16,
    tx_power: i16,
    icon: String,
}

async fn query_device(adapter: &Adapter, addr: Address) -> BLEDevice {
    let device = adapter.device(addr).expect("Can't parse device");
    let parsed_device = BLEDevice {
        name: device
            .name()
            .await
            .expect("err")
            .unwrap_or("unknown".to_string()),
        rssi: device.rssi().await.expect("err").unwrap_or(-128),
        tx_power: device.tx_power().await.expect("err").unwrap_or(-1),
        icon: device
            .icon()
            .await
            .expect("err")
            .unwrap_or("unknown".to_string()),
    };

    parsed_device
}

pub async fn ble_scan_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("`{user_agent}` at {addr} connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| ble_scan(socket, addr))
}

/// Actual websocket statemachine (one will be spawned per connection)
pub async fn ble_scan(mut socket: WebSocket, who: SocketAddr) {
    // send a ping (unsupported by some browsers) just to kick things off and get a response
    if socket
        .send(Message::Ping(Bytes::from_static(&[1, 2, 3])))
        .await
        .is_ok()
    {
        println!("Pinged {who}...");
    } else {
        println!("Could not send ping {who}!");
        // no Error here since the only thing we can do is to close the connection.
        // If we can not send messages, there is no way to salvage the statemachine anyway.
        return;
    }

    // let with_changes = env::args().any(|arg| arg == "--changes");
    // let all_properties = env::args().any(|arg| arg == "--all-properties");
    let le_only = env::args().any(|arg| arg == "--le");
    let br_edr_only = env::args().any(|arg| arg == "--bredr");
    let filter_addr: HashSet<_> = env::args()
        .filter_map(|arg| arg.parse::<Address>().ok())
        .collect();

    let session = bluer::Session::new().await.expect("Err");
    let adapter = session.default_adapter().await.expect("Err");
    println!(
        "Discovering devices using Bluetooth adapter {}\n",
        adapter.name()
    );
    adapter.set_powered(true).await.expect("Err");

    let filter = DiscoveryFilter {
        transport: if le_only {
            DiscoveryTransport::Le
        } else if br_edr_only {
            DiscoveryTransport::BrEdr
        } else {
            DiscoveryTransport::Auto
        },
        ..Default::default()
    };
    adapter.set_discovery_filter(filter).await.expect("err");
    println!(
        "Using discovery filter:\n{:#?}\n\n",
        adapter.discovery_filter().await
    );

    let mut device_events = adapter.discover_devices().await.expect("err");
    // pin_mut!(device_events);

    // let mut all_change_events = SelectAll::new();
    let (mut sender, mut receiver) = socket.split();

    let mut recv_task = tokio::spawn(async move {
        loop {
            let msg = receiver.next().await.expect("error").expect("error2");
            if msg.to_text().expect("Can't parse to str") == "stop" {
                println!("Stopping clint");
                break;
            }
        }
    });
    let mut send_task = tokio::spawn(async move {
        loop {
            let device_event = device_events.next().await.expect("No next");
            match device_event {
                AdapterEvent::DeviceAdded(addr) => {
                    if !filter_addr.is_empty() && !filter_addr.contains(&addr) {
                        continue;
                    }
                    println!("Device added: {addr}");
                    let res = query_device(&adapter, addr).await;
                    sender
                        .send(Message::Text(
                            serde_json::to_string(&res)
                                .expect("Can't stringify struct")
                                .into(),
                        ))
                        .await
                        .expect("Can't send msg");

                    // if with_changes {
                    //     let device = adapter.device(addr).expect("Err");
                    //     // let change_events = device
                    //     //     .events()
                    //     //     .await
                    //     //     .expect("err")
                    //     //     .map(move |evt| (addr, evt));
                    //     // all_change_events.push(change_events);
                    // }
                }
                AdapterEvent::DeviceRemoved(addr) => {
                    sender
                        .send(Message::Text(std::format!("removed-{}", addr).into()))
                        .await
                        .expect("Can't send msg");
                    println!("Device removed: {addr}");
                }
                // AdapterEvent::PropertyChanged(property) => {
                // let change = all_change_events.next();
                // println!("Device changed");
                // println!("Device changed: {addr}");
                // println!("    {property:?}");
                // }
                _ => (),
            }
        }
    });
    // If any one of the tasks exit, abort the other.
    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(_a) => println!("Send killed"),
                Err(a) => println!("Error sending messages {a:?}")
            }
            recv_task.abort();
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(_b) => println!("Rec killed"),
                Err(b) => println!("Error receiving messages {b:?}")
            }
            send_task.abort();
        }
    }

    println!("Destroying socket");
}

pub async fn fastpair_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    fastpair::_fastpair_handler(ws, user_agent, ConnectInfo(addr))
}

pub async fn apple_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    apple::_apple_handler(ws, user_agent, ConnectInfo(addr))
}

pub async fn swiftpair_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    swiftpair::_swiftpair_handler(ws, user_agent, ConnectInfo(addr))
}