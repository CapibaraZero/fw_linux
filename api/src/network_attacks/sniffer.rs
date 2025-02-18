use axum::extract::connect_info::ConnectInfo;
use axum::{
    body::Bytes,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use axum_extra::TypedHeader;
use chrono::{Datelike, Timelike};
use std::fs;
use std::process::Command;

use futures::{SinkExt, StreamExt};
use pcap::{Capture, Device};
use std::net::SocketAddr;

pub fn _sniffer_handler(
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
    ws.on_upgrade(move |socket| start_sniffer(socket, addr))
}

pub async fn start_sniffer(mut socket: WebSocket, who: SocketAddr) {
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

    let year = chrono::offset::Local::now().year();
    let month = chrono::offset::Local::now().month();
    let day = chrono::offset::Local::now().day();
    let hour = chrono::offset::Local::now().hour();
    let minutes = chrono::offset::Local::now().minute();
    let seconds = chrono::offset::Local::now().second();
    // println!();
    let devices = Device::list().unwrap();
    let mut device: Option<Device> = None;
    for dev in devices {
        if dev.name == interface.to_text().unwrap() {
            device = Some(dev);
        }
    }
    println!("Using device {}", device.clone().unwrap().name);
    let (mut _sender, mut receiver) = socket.split();

    let path = std::format!(
        "~/wifi_captures/{}_{}_{}_{}_{}_{}.pcap",
        day,
        month,
        year,
        hour,
        minutes,
        seconds);
    let path2: String = path.clone();

    _sender.send(Message::text(path.clone())).await.expect("Can't send path");
    let mut send_task = tokio::spawn(async move {
        // Setup Capture
        let mut cap = Capture::from_device(device.expect("Can't find device"))
            .unwrap()
            // .promisc(true)
            .immediate_mode(true)
            .open()
            .unwrap();

        // open savefile using the capture
        let mut savefile = cap
            .savefile(path)
            .unwrap();

        while let Ok(packet) = cap.next_packet() {
            // println!("received packet! {:?}", packet);
            savefile.write(&packet);
        }
    });

    // let (tx, rx) = channel();


    let mut recv_task = tokio::spawn(async move {
        loop {
            let msg = receiver.next().await.expect("error").expect("error2");
            if msg.to_text().expect("Can't parse to str") == "stop" {
                println!("Stopping clint");
                let _raw_res = Command::new("sh").arg("-c").arg("pkill -9 pig.py").spawn();
                // cancelled = true;
                break;
            }
        }
    });

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
    let res = fs::read(path2).expect("Can't read PCAP");
    _sender.send(Message::Binary(res.into())).await.unwrap();
    // send_task.
    println!("Destroying socket");
}
